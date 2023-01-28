//! Provides the parser for SpeedrunIGT splits files.

use alloc::borrow::Cow;
use serde::Deserialize;
use time::Duration;

use crate::{
    platform::{prelude::*, DateTime},
    AtomicDateTime, Run, Segment, Time,
};

/// The Error type for splits files that couldn't be parsed by the SpeedRunIGT
/// Parser.
#[derive(Debug, snafu::Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
    /// Failed to parse JSON.
    Json {
        /// The underlying error.
        #[cfg_attr(not(feature = "std"), snafu(source(false)))]
        source: serde_json::Error,
    },
}

/// The Result type for the SpeedRunIGT Parser.
pub type Result<T> = core::result::Result<T, Error>;

// Documented here:
// https://github.com/RedLime/SpeedRunIGT/wiki/Personal-Records-Document

#[derive(Deserialize)]
struct Splits<'a> {
    #[serde(borrow)]
    mc_version: Cow<'a, str>,
    #[serde(borrow)]
    speedrunigt_version: Cow<'a, str>,
    #[serde(borrow)]
    category: Cow<'a, str>,
    #[serde(borrow)]
    run_type: Cow<'a, str>,
    is_coop: bool,
    is_hardcore: bool,
    #[serde(borrow)]
    world_name: Cow<'a, str>,
    date: i64,
    final_igt: i64,
    final_rta: i64,
    timelines: Vec<Timeline<'a>>,
}

#[derive(Deserialize)]
struct Timeline<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,
    igt: i64,
    rta: i64,
}

#[derive(PartialEq, Eq)]
enum CategoryType {
    StandardOrUnknown,
    Extension,
    UnofficialExtension,
}

fn map_seed_type<'a>(
    run_type: &str,
    key: &'a str,
    set: &'a str,
    random: &'a str,
) -> Option<[&'a str; 2]> {
    match run_type {
        "set_seed" => Some([key, set]),
        "random_seed" => Some([key, random]),
        _ => None,
    }
}

fn time(rta: i64, igt: i64) -> Time {
    Time {
        real_time: Some(Duration::milliseconds(rta).into()),
        game_time: Some(Duration::milliseconds(igt).into()),
    }
}

type Var = [&'static str; 2];

enum Vars {
    Zero,
    One(Var),
    Two([Var; 2]),
}

impl Vars {
    const fn slice(&self) -> &[Var] {
        match self {
            Vars::Zero => &[],
            Vars::One(slice) => core::slice::from_ref(slice),
            Vars::Two(slice) => slice,
        }
    }

    const fn with(self, var: Var) -> Self {
        match self {
            Vars::Zero => Vars::One(var),
            Vars::One(first) => Vars::Two([first, var]),
            v => v,
        }
    }
}

impl From<Option<Var>> for Vars {
    fn from(v: Option<Var>) -> Self {
        match v {
            Some(v) => Vars::One(v),
            None => Vars::Zero,
        }
    }
}

/// Attempts to parse an SpeedRunIGT splits file.
pub fn parse(source: &str) -> Result<Run> {
    let splits: Splits<'_> =
        serde_json::from_str(source).map_err(|source| Error::Json { source })?;
    let mut run = Run::new();

    // FIXME: There is no way to tell if it's glitchless, but glitchless is way
    // more popular, so we simply assume it's the case by default.

    // https://github.com/RedLime/SpeedRunIGT/wiki/Category-IDs-Document
    let (category, category_variables, category_type) = match &*splits.category {
        "ANY" => (
            if splits.is_coop {
                // There is no non-glitchless Co-op category.
                "Any% Glitchless Co-op"
            } else {
                "Any% Glitchless"
            }
            .into(),
            // FIXME: Version Range (Any% Glitchless)
            // FIXME: Players, Version Range (Any% RSG Co-op)
            map_seed_type(
                &splits.run_type,
                if splits.is_coop {
                    "Seed type (Any% Glitchless Co-op)"
                } else {
                    "Seed Type (Any% Glitchless)"
                },
                "Set Seed",
                "Random Seed",
            )
            .into(),
            CategoryType::StandardOrUnknown,
        ),
        "HIGH" => (
            "High%".into(),
            map_seed_type(&splits.run_type, "RSG/SSG/RS/SS High%", "SSG", "RSG").into(),
            CategoryType::Extension,
        ),
        "KILL_ALL_BOSSES" => kill_bosses(&splits, "All Bosses"),
        "KILL_WITHER" => kill_bosses(&splits, "Wither"),
        "KILL_ELDER_GUARDIAN" => kill_bosses(&splits, "Elder Guardian"),
        "KILL_WARDEN" => kill_bosses(&splits, "Warden"),
        "ALL_ADVANCEMENTS" => (
            if splits.is_coop {
                "All Advancements Co-op"
            } else {
                "All Advancements"
            }
            .into(),
            if splits.is_coop {
                // FIXME: Version Range (AAdv Co-op), Player Count (AAdv Co-op)
                if splits.run_type == "random_seed" {
                    Vars::One(["Seed/Glitch (AAdv Co-op)", "RSG"])
                } else {
                    // Weirdly there's no SSG
                    Vars::Zero
                }
            } else {
                // FIXME: Version Range (AAdv)
                map_seed_type(&splits.run_type, "Seed Type (AAdv)", "SSG", "RSG").into()
            },
            CategoryType::StandardOrUnknown,
        ),
        "ALL_ACHIEVEMENTS" => (
            "All Achievements".into(),
            // FIXME: Version Range (AA)
            map_seed_type(&splits.run_type, "Seed Type (AA)", "SSG", "RSG").into(),
            CategoryType::StandardOrUnknown,
        ),
        "HALF" => (
            "Half%".into(),
            // FIXME: Version (Half%)
            map_seed_type(&splits.run_type, "SS/SSG/RS/RSG (Half%)", "SSG", "RSG").into(),
            CategoryType::Extension,
        ),
        "HOW_DID_WE_GET_HERE" => (
            "How Did We Get Here?".into(),
            // FIXME: Version (HDWGH)
            map_seed_type(&splits.run_type, "SS/RS/SSG/RSG (HDWGH)", "SSG", "RSG").into(),
            CategoryType::Extension,
        ),
        "HERO_OF_VILLAGE" => (
            "Hero of the Village".into(),
            map_seed_type(
                &splits.run_type,
                "SSG/RSG (Hero of the Village)",
                "Set Seed Glitchless",
                "Random Seed Glitchless",
            )
            .into(),
            CategoryType::Extension,
        ),
        "ARBALISTIC" => (
            "Arbalistic".into(),
            // FIXME: Structures (Arbalistic)
            map_seed_type(
                &splits.run_type,
                "SSG/RSG (Arbalistic)",
                "Set Seed Glitchless",
                "Random Seed Glitchless",
            )
            .into(),
            CategoryType::Extension,
        ),
        "COVER_ME_IN_DEBRIS" => (
            "Cover Me in Debris".into(),
            // FIXME: Structures (Cover Me in Debris)
            map_seed_type(
                &splits.run_type,
                "SSG/RSG (Cover Me in Debris)",
                "SSG",
                "RSG",
            )
            .into(),
            CategoryType::Extension,
        ),
        "ENTER_NETHER" => (
            "Enter Nether".into(),
            // FIXME: Structures (Enter Nether)
            map_seed_type(
                &splits.run_type,
                "SSG/RSG (Enter Nether)",
                "Set Seed Glitchless",
                "Random Seed Glitchless",
            )
            .into(),
            CategoryType::Extension,
        ),
        "ENTER_END" => (
            "Etner Edn".into(), // Nice typo
            Vars::One(["SS/SSG (Enter End)", "SSG"]),
            CategoryType::Extension,
        ),
        "ALL_SWORDS" => (
            "All Swords".into(),
            // FIXME: Structures (All Swords), Version (All Swords)
            map_seed_type(
                &splits.run_type,
                "SSG/RSG (All Swords)",
                "Set Seed Glitchless",
                "Random Seed Glitchless",
            )
            .into(),
            CategoryType::Extension,
        ),
        "ALL_MINERALS" => (
            "All Minerals".into(),
            // FIXME: Structures (All Minerals), Version (All Minerals)
            map_seed_type(
                &splits.run_type,
                "SSG/RSG (All Minerals)",
                "Set Seed Glitchless",
                "Random Seed Glitchless",
            )
            .into(),
            CategoryType::Extension,
        ),
        "FULL_IA_15_LVL" => (
            "Full Iron Armor and 15 Levels".into(),
            // FIXME: Structures (Full Iron 15 Levels)
            map_seed_type(
                &splits.run_type,
                "SSG/RSG (Full Iron 15 Levels)",
                "Set Seed Glitchless",
                "Random Seed Glitchless",
            )
            .into(),
            CategoryType::Extension,
        ),
        "ALL_WORKSTATIONS" => (
            "All Workstations".into(),
            map_seed_type(
                &splits.run_type,
                "SSG/RSG (All Workstations)",
                "Set Seed Glitchless",
                "Random Seed Glitchless",
            )
            .into(),
            CategoryType::Extension,
        ),
        "FULL_INV" => (
            "Full Inventory".into(),
            // FIXME: Structures (Full Inventory)
            map_seed_type(
                &splits.run_type,
                "SSG/RSG (Full Inventory)",
                "Set Seed Glitchless",
                "Random Seed Glitchless",
            )
            .into(),
            CategoryType::Extension,
        ),
        "CUSTOM" => ("Custom".into(), Vars::Zero, CategoryType::StandardOrUnknown),
        "STACK_OF_LIME_WOOL" => (
            "Stack of Lime Wool".into(),
            map_seed_type(
                &splits.run_type,
                "SS/SSG/RS/RSG (Stack of Lime Wool)",
                "Set Seed Glitchless",
                "Random Seed Glitchless",
            )
            .into(),
            CategoryType::Extension,
        ),
        "POGLOOT_QUATER" => (
            "Quater%".into(),
            // FIXME: Version (Quater)
            map_seed_type(&splits.run_type, "Seed (Quater)", "SSG", "RSG").into(),
            CategoryType::UnofficialExtension,
        ),
        "ALL_PORTALS" => (
            "All Portals".into(),
            // FIXME: Version (All Portals)
            map_seed_type(
                &splits.run_type,
                "SS/SSG/RS/RSG (All Portals)",
                "SSG",
                "RSG",
            )
            .into(),
            CategoryType::Extension,
        ),
        // Not currently on speedrun.com
        "ALL_BLOCKS" => (
            "All Blocks".into(),
            Vars::Zero,
            CategoryType::StandardOrUnknown,
        ),
        "MINE_A_CHUNK" => (
            "Mine a Chunk".into(),
            {
                // FIXME: Dimension (Mine a Chunk)
                let vars = Vars::from(map_seed_type(
                    &splits.run_type,
                    "SS/SSG/RS/RSG (Mine a Chunk)",
                    "SSG",
                    "RSG",
                ));
                if splits.is_coop {
                    // FIXME: Player count
                    vars
                } else {
                    vars.with(["Player Count (Mine a Chunk)", "Solo"])
                }
            },
            CategoryType::Extension,
        ),
        other => {
            let mut new_category = String::with_capacity(other.len());
            let mut last_is_space = true;
            for c in other.chars() {
                if c == '_' {
                    new_category.push(' ');
                    last_is_space = true;
                } else if last_is_space {
                    new_category.push(c);
                    last_is_space = false;
                } else {
                    new_category.extend(c.to_lowercase());
                }
            }
            (
                Cow::from(new_category),
                Vars::Zero,
                CategoryType::StandardOrUnknown,
            )
        }
    };

    run.set_game_name(match category_type {
        CategoryType::StandardOrUnknown => "Minecraft: Java Edition",
        CategoryType::Extension => "Minecraft: Java Edition Category Extensions",
        CategoryType::UnofficialExtension => {
            "Minecraft: Java Edition Unofficial Category Extensions"
        }
    });

    run.set_category_name(category);

    run.set_attempt_count(1);

    let metadata = run.metadata_mut();

    for &[key, value] in category_variables.slice() {
        metadata.set_speedrun_com_variable(key, value);
    }

    metadata.set_platform_name("PC");

    if category_type != CategoryType::Extension {
        metadata.set_speedrun_com_variable("Version", splits.mc_version);
    }
    // FIXME: Category Extensions have Version / Subversion split

    if splits.is_hardcore {
        // The specific difficulty is not currently a setting in SpeedRunIGT.
        metadata.set_speedrun_com_variable("Difficulty", "Hardcore");
    }

    // FIXME: F3

    // The unofficial extensions are missing the "Modded" value.
    if category_type != CategoryType::UnofficialExtension {
        metadata.set_speedrun_com_variable("Mods", "Modded");
    }

    let speedrun_igt_version = metadata.custom_variable_mut("SpeedRunIGT Version");
    speedrun_igt_version.set_value(splits.speedrunigt_version);
    speedrun_igt_version.is_permanent = true;

    let world_name = metadata.custom_variable_mut("World Name");
    world_name.set_value(splits.world_name);
    world_name.is_permanent = true;

    for timeline in splits.timelines {
        let name: Cow<'_, str> = match &*timeline.name {
            "crafted_ender_eye" => "Crafted Ender Eye".into(),
            "enter_bastion" => "Found Bastion".into(),
            "enter_end" => "Enter The End".into(),
            "enter_fortress" => "Found Fortress".into(),
            "enter_nether" => "Enter Nether".into(),
            "enter_stronghold" => "Enter Stronghold".into(),
            "found_villager" => "Found Villager".into(),
            "got_trident" => "Got Trident".into(),
            "kill_elder_guardian" => "Defeat Elder Guardian".into(),
            "kill_ender_dragon" => "Defeat Ender Dragon".into(),
            "kill_warden" => "Defeat Warden".into(),
            "kill_wither" => "Defeat Wither".into(),
            "nether_travel" => "Nether Travel".into(),
            "pick_gold_block" => "Pick Gold Block".into(),
            "pickup_book" => "Pickup Book".into(),
            "sleep_on_tower" => "Sleep on Tower".into(),
            "trade_with_villager" => "Trade with Villager".into(),
            name => {
                if let Some(rem) = name.strip_prefix("portal_no_") {
                    format!("Portal No. {rem}").into()
                } else if let Some(rem) = name.strip_prefix("got_shell_") {
                    format!("Got Nautilus Shell {rem}").into()
                } else {
                    let mut new_name = String::with_capacity(name.len());
                    let mut last_is_space = true;
                    for c in name.chars() {
                        if c == '_' {
                            new_name.push(' ');
                            last_is_space = true;
                        } else if last_is_space {
                            new_name.extend(c.to_uppercase());
                            last_is_space = false;
                        } else {
                            new_name.push(c);
                        }
                    }
                    new_name.into()
                }
            }
        };
        let mut segment = Segment::new(name);
        segment.set_personal_best_split_time(time(timeline.rta, timeline.igt));
        run.push_segment(segment);
    }

    let mut segment = Segment::new("Finish");
    segment.set_personal_best_split_time(time(splits.final_rta, splits.final_igt));
    run.push_segment(segment);

    process_segments(&mut run, splits.date);

    Ok(run)
}

fn kill_bosses<'a>(splits: &Splits<'a>, boss: &'static str) -> (Cow<'a, str>, Vars, CategoryType) {
    (
        "Kill Bosses".into(),
        // FIXME: Version (Kill Bosses)
        match map_seed_type(
            &splits.run_type,
            "SS/RS/SSG/RSG (Kill Bosses)",
            "SSG",
            "RSG",
        ) {
            Some(value) => Vars::Two([["Boss", boss], value]),
            None => Vars::One(["Boss", boss]),
        },
        CategoryType::Extension,
    )
}

fn process_segments(run: &mut Run, ended: i64) {
    let mut previous_split = Time::zero();

    for segment in run.segments_mut() {
        let split_time = segment.personal_best_split_time();
        // This assumes that there aren't any skipped segments, which should
        // always be the case with SpeedRunIGT.
        let segment_time = split_time - previous_split;
        *segment.best_segment_time_mut() = segment_time;
        segment.segment_history_mut().insert(1, segment_time);
        previous_split = split_time;
    }

    let ended = DateTime::from_unix_timestamp(ended / 1000)
        .ok()
        .map(|date| AtomicDateTime::new(date + Duration::milliseconds(ended % 1000), false));

    let started = ended.and_then(|ended| {
        Some(AtomicDateTime::new(
            ended.time - previous_split.real_time?.to_duration(),
            false,
        ))
    });

    run.add_attempt_with_index(previous_split, 1, started, ended, None);
}
