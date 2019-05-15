//! The component module provides all the different components available. A
//! Component allows querying different kinds of information from a Timer. This
//! information is provided as state objects in a way that can easily be
//! visualized by any kind of User Interface.

pub mod blank_space;
pub mod current_comparison;
pub mod current_pace;
pub mod delta;
pub mod detailed_timer;
pub mod graph;
pub mod possible_time_save;
pub mod previous_segment;
pub mod separator;
pub mod splits;
pub mod sum_of_best;
pub mod text;
pub mod timer;
pub mod title;
pub mod total_playtime;

pub use blank_space::Component as BlankSpace;
pub use current_comparison::Component as CurrentComparison;
pub use current_pace::Component as CurrentPace;
pub use delta::Component as Delta;
pub use detailed_timer::Component as DetailedTimer;
pub use graph::Component as Graph;
pub use possible_time_save::Component as PossibleTimeSave;
pub use previous_segment::Component as PreviousSegment;
pub use separator::Component as Separator;
pub use splits::Component as Splits;
pub use sum_of_best::Component as SumOfBest;
pub use text::Component as Text;
pub use timer::Component as Timer;
pub use title::Component as Title;
pub use total_playtime::Component as TotalPlaytime;

use crate::settings::{Color, Gradient};
use palette::rgb::Rgb;
use palette::Alpha;
use std::marker::PhantomData;

const DEFAULT_INFO_TEXT_GRADIENT: Gradient = Gradient::Vertical(
    Color {
        rgba: Alpha {
            alpha: 0.06,
            color: Rgb {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                standard: PhantomData,
            },
        },
    },
    Color {
        rgba: Alpha {
            alpha: 0.005,
            color: Rgb {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                standard: PhantomData,
            },
        },
    },
);
