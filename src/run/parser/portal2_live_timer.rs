//! Provides the parser for Portal 2 Live Timer splits files.

use std::io::{self, BufRead};
use std::num::ParseFloatError;
use std::result::Result as StdResult;
use {GameTime, Run, Segment, TimeSpan};

quick_error! {
    /// The Error types for splits files that couldn't be parsed by the Portal 2
    /// Live Timer Parser.
    #[derive(Debug)]
    pub enum Error {
        /// Expected another map, but didn't find it.
        ExpectedMap {}
        /// Expected the map's name, but didn't find it.
        ExpectedMapName {}
        /// Expected a different map.
        ExpectedDifferentMapName {}
        /// Expected the start ticks of the map, but didn't find it.
        ExpectedStartTicks {}
        /// Expected the end ticks of the map, but didn't find it.
        ExpectedEndTicks {}
        /// Couldn't parse the amount of ticks.
        Ticks(err: ParseFloatError) {
            from()
        }
        /// Failed to read from the source.
        Io(err: io::Error) {
            from()
        }
    }
}

/// The Result type for the Portal 2 Live Timer Parser.
pub type Result<T> = StdResult<T, Error>;

static CHAPTERS: [(&str, &[&str]); 9] = [
    (
        "Chapter 1 - The Courtesy Call",
        &[
            "sp_a1_intro1",
            "sp_a1_intro2",
            "sp_a1_intro3",
            "sp_a1_intro4",
            "sp_a1_intro5",
            "sp_a1_intro6",
            "sp_a1_intro7",
            "sp_a1_wakeup",
            "sp_a2_intro",
        ],
    ),
    (
        "Chapter 2 - The Cold Boot",
        &[
            "sp_a2_laser_intro",
            "sp_a2_laser_stairs",
            "sp_a2_dual_lasers",
            "sp_a2_laser_over_goo",
            "sp_a2_catapult_intro",
            "sp_a2_trust_fling",
            "sp_a2_pit_flings",
            "sp_a2_fizzler_intro",
        ],
    ),
    (
        "Chapter 3 - The Return",
        &[
            "sp_a2_sphere_peek",
            "sp_a2_ricochet",
            "sp_a2_bridge_intro",
            "sp_a2_bridge_the_gap",
            "sp_a2_turret_intro",
            "sp_a2_laser_relays",
            "sp_a2_turret_blocker",
            "sp_a2_laser_vs_turret",
            "sp_a2_pull_the_rug",
        ],
    ),
    (
        "Chapter 4 - The Surprise",
        &[
            "sp_a2_column_blocker",
            "sp_a2_laser_chaining",
            "sp_a2_triple_laser",
            "sp_a2_bts1",
            "sp_a2_bts2",
        ],
    ),
    (
        "Chapter 5 - The Escape",
        &[
            "sp_a2_bts3",
            "sp_a2_bts4",
            "sp_a2_bts5",
            "sp_a2_bts6",
            "sp_a2_core",
        ],
    ),
    (
        "Chapter 6 - The Fall",
        &[
            "sp_a3_00",
            "sp_a3_01",
            "sp_a3_03",
            "sp_a3_jump_intro",
            "sp_a3_bomb_flings",
            "sp_a3_crazy_box",
            "sp_a3_transition01",
        ],
    ),
    (
        "Chapter 7 - The Reunion",
        &[
            "sp_a3_speed_ramp",
            "sp_a3_speed_flings",
            "sp_a3_portal_intro",
            "sp_a3_end",
        ],
    ),
    (
        "Chapter 8 - The Itch",
        &[
            "sp_a4_intro",
            "sp_a4_tb_intro",
            "sp_a4_tb_trust_drop",
            "sp_a4_tb_wall_button",
            "sp_a4_tb_polarity",
            "sp_a4_tb_catch",
            "sp_a4_stop_the_box",
            "sp_a4_laser_catapult",
            "sp_a4_laser_platform",
            "sp_a4_speed_tb_catch",
            "sp_a4_jump_polarity",
        ],
    ),
    (
        "Chapter 9 - The Part Where...",
        &[
            "sp_a4_finale1",
            "sp_a4_finale2",
            "sp_a4_finale3",
            "sp_a4_finale4",
        ],
    ),
];

/// Attempts to parse a Portal 2 Live Timer splits file.
pub fn parse<R: BufRead>(source: R) -> Result<Run> {
    let mut run = Run::new();

    run.set_game_name("Portal 2");
    run.set_category_name("Any%");

    let mut lines = source.lines().peekable();
    lines.next(); // Skip the header

    let mut aggregate_ticks = 0.0;

    let mut line = lines.next().ok_or(Error::ExpectedMap)??;
    for &(chapter_name, maps) in &CHAPTERS {
        for &map in maps {
            {
                let mut splits = line.split(',');
                let map_name = splits.next().ok_or(Error::ExpectedMapName)?;
                if map_name != map {
                    return Err(Error::ExpectedDifferentMapName);
                }
                let start_ticks: f64 = splits.next().ok_or(Error::ExpectedStartTicks)?.parse()?;
                let end_ticks: f64 = splits.next().ok_or(Error::ExpectedEndTicks)?.parse()?;
                let map_ticks = end_ticks - start_ticks;
                aggregate_ticks += map_ticks;
            }

            while let Some(Ok(next_line)) = lines.next() {
                line = next_line;
                let mut splits = line.split(',');
                if splits.next() == Some(map) {
                    let start_ticks: f64 = splits.next().ok_or(Error::ExpectedStartTicks)?.parse()?;
                    let end_ticks: f64 = splits.next().ok_or(Error::ExpectedEndTicks)?.parse()?;
                    let map_ticks = end_ticks - start_ticks;
                    aggregate_ticks += map_ticks;
                } else {
                    break;
                }
            }
        }

        let time = GameTime(Some(TimeSpan::from_seconds(aggregate_ticks / 60.0))).into();
        let mut segment = Segment::new(chapter_name);
        segment.set_personal_best_split_time(time);

        run.push_segment(segment);
    }

    Ok(run)
}
