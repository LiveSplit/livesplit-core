//! Typed responses from the Speedrun.com API.

use crate::util::ordered_map::Map;
use serde_derive::Deserialize;

/// The names supplied by the game community.
#[derive(Clone, Debug, Deserialize)]
pub struct Names {
    /// The international display name.
    pub international: String,
    /// The Japanese display name, when supplied.
    pub japanese: Option<String>,
}

/// A game's small representation returned by bulk game listing.
#[derive(Clone, Debug, Deserialize)]
pub struct GameHeader {
    /// The stable game identifier.
    pub id: String,
    /// Community-provided game names.
    pub names: Names,
    /// The mutable game abbreviation. Prefer the ID for requests.
    pub abbreviation: String,
    /// The site's human-facing URL.
    pub weblink: String,
}

/// Game metadata, including variables embedded by [`super::Client::game`].
#[derive(Clone, Debug, Deserialize)]
pub struct Game {
    /// The small game representation's fields.
    #[serde(flatten)]
    pub header: GameHeader,
    /// Game artwork. Missing artwork can be represented by a placeholder URL.
    pub assets: Assets,
    /// Rules controlling metadata fields.
    pub ruleset: Rules,
    /// IDs of the supported platforms.
    pub platforms: Vec<String>,
    /// IDs of the supported regions.
    pub regions: Vec<String>,
    /// Variables embedded by the metadata request.
    pub variables: Embedded<Vec<Variable>>,
}

/// An embedded API resource. An empty collection is different from a missing embed.
#[derive(Clone, Debug, Deserialize)]
pub struct Embedded<T> {
    /// The resource payload.
    pub data: T,
}

/// A game's timing and run requirements.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Rules {
    /// Whether the community allows emulators for this game.
    pub emulators_allowed: bool,
    /// Whether runs require video evidence. Absent older metadata has no requirement.
    #[serde(default)]
    pub require_video: bool,
    /// The community's default timing method; older metadata defaults to real time.
    #[serde(default)]
    pub default_time: TimingMethod,
}

/// Timing modes used by the site's game rules, including real time without loads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Deserialize)]
pub enum TimingMethod {
    /// Ordinary elapsed real time.
    #[default]
    #[serde(rename = "realtime")]
    RealTime,
    /// Real time with loading screens removed.
    #[serde(rename = "realtime_noloads")]
    RealTimeWithoutLoads,
    /// The game's own timer.
    #[serde(rename = "ingame")]
    GameTime,
}

/// Game artwork.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Assets {
    /// Medium box art, matching the web editor's choice.
    pub cover_medium: Option<Asset>,
    /// The game's icon.
    pub icon: Option<Asset>,
}

/// A site-provided image URL.
#[derive(Clone, Debug, Deserialize)]
pub struct Asset {
    /// The supplied image URL; downloads require HTTPS.
    pub uri: String,
}

/// The kind of game artwork to request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Artwork {
    /// The medium-sized game cover.
    BoxArt,
    /// The game icon.
    Icon,
}

impl Game {
    /// Returns the requested artwork when the site supplies a real asset.
    /// The web editor's placeholder images are treated as missing artwork.
    pub fn artwork(&self, kind: Artwork) -> Option<&Asset> {
        let (asset, placeholder) = match kind {
            Artwork::BoxArt => (
                &self.assets.cover_medium,
                "https://www.speedrun.com/images/blankcover.png",
            ),
            Artwork::Icon => (&self.assets.icon, "https://www.speedrun.com/images/1st.png"),
        };

        asset.as_ref().filter(|asset| asset.uri != placeholder)
    }
}

/// A game's category, including full-game and individual-level categories.
#[derive(Clone, Debug, Deserialize)]
pub struct Category {
    /// The stable category identifier.
    pub id: String,
    /// The community-provided name.
    pub name: String,
    /// Whether this is a full-game or individual-level category.
    #[serde(rename = "type")]
    pub kind: CategoryKind,
    /// Markdown rules, when provided.
    pub rules: Option<String>,
}

/// The kind of runs to which a category applies.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum CategoryKind {
    /// A full-game category.
    #[serde(rename = "per-game")]
    PerGame,
    /// An individual-level category.
    #[serde(rename = "per-level")]
    PerLevel,
}

/// A game-specific variable.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Variable {
    /// The stable variable identifier.
    pub id: String,
    /// The name stored in LiveSplit metadata.
    pub name: String,
    /// A specific category ID, or `None` for every category.
    pub category: Option<String>,
    /// The kinds of runs this variable describes.
    pub scope: VariableScope,
    /// Available values and the site's default value ID.
    pub values: VariableValues,
    /// Whether the site requires a value when submitting runs.
    pub mandatory: bool,
    /// Whether the site presents this variable as a subcategory.
    pub is_subcategory: bool,
}

impl Variable {
    /// Matches the web splits editor: only global/full-game variables appear,
    /// and category-specific ones require the matching full-game category.
    pub fn applies_to(&self, category: Option<&Category>) -> bool {
        matches!(self.scope, VariableScope::Global | VariableScope::FullGame)
            && self
                .category
                .as_deref()
                .is_none_or(|id| category.is_some_and(|c| c.id == id))
    }
}

/// The scope of a variable, as a tagged v1 API object.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum VariableScope {
    /// All run types.
    Global,
    /// Full-game runs only.
    FullGame,
    /// All individual-level runs.
    AllLevels,
    /// Runs of one particular level.
    SingleLevel {
        /// The level identifier.
        level: String,
    },
}

/// The community-defined values of a variable, in the site's display order.
#[derive(Clone, Debug, Deserialize)]
pub struct VariableValues {
    /// Value IDs mapped to labels and optional rules.
    pub values: Map<VariableValue>,
    /// The site's default value ID, when configured.
    pub default: Option<String>,
}

/// One predefined value of a game variable.
#[derive(Clone, Debug, Deserialize)]
pub struct VariableValue {
    /// The label stored in LiveSplit metadata.
    pub label: String,
    /// Additional Markdown rules, when provided.
    pub rules: Option<String>,
}

/// A hardware platform supported by games on the site.
#[derive(Clone, Debug, Deserialize)]
pub struct Platform {
    /// The platform identifier.
    pub id: String,
    /// Its community-provided display name.
    pub name: String,
}

/// A release region supported by games on the site.
#[derive(Clone, Debug, Deserialize)]
pub struct Region {
    /// The region identifier.
    pub id: String,
    /// Its community-provided display name.
    pub name: String,
}
