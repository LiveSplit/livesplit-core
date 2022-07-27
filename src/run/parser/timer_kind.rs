use alloc::borrow::Cow;

use core::fmt;

/// Describes the different Timers available that store splits files.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum TimerKind<'a> {
    /// LiveSplit
    LiveSplit,
    /// WSplit
    WSplit,
    /// SplitterZ
    SplitterZ,
    /// ShitSplit
    ShitSplit,
    /// Splitty
    Splitty,
    /// Time Split Tracker
    TimeSplitTracker,
    /// Portal 2 Live Timer
    Portal2LiveTimer,
    /// FaceSplit
    FaceSplit,
    /// Flitter
    Flitter,
    /// Llanfair
    Llanfair,
    /// Gered's fork of Llanfair
    LlanfairGered,
    /// Urn
    Urn,
    /// SourceLiveTimer
    SourceLiveTimer,
    /// Splitterino
    Splitterino,
    /// A Generic Timer. The name of the timer is associated with the variant.
    /// "Generic Timer" is used if there is no known name.
    Generic(Cow<'a, str>),
}

impl TimerKind<'_> {
    /// Returns an owned version of the timer kind.
    pub fn into_owned(self) -> TimerKind<'static> {
        match self {
            TimerKind::LiveSplit => TimerKind::LiveSplit,
            TimerKind::WSplit => TimerKind::WSplit,
            TimerKind::SplitterZ => TimerKind::SplitterZ,
            TimerKind::ShitSplit => TimerKind::ShitSplit,
            TimerKind::Splitty => TimerKind::Splitty,
            TimerKind::TimeSplitTracker => TimerKind::TimeSplitTracker,
            TimerKind::Portal2LiveTimer => TimerKind::Portal2LiveTimer,
            TimerKind::FaceSplit => TimerKind::FaceSplit,
            TimerKind::Flitter => TimerKind::Flitter,
            TimerKind::Llanfair => TimerKind::Llanfair,
            TimerKind::LlanfairGered => TimerKind::LlanfairGered,
            TimerKind::Urn => TimerKind::Urn,
            TimerKind::SourceLiveTimer => TimerKind::SourceLiveTimer,
            TimerKind::Splitterino => TimerKind::Splitterino,
            TimerKind::Generic(v) => TimerKind::Generic(v.into_owned().into()),
        }
    }
}

impl fmt::Display for TimerKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimerKind::LiveSplit => write!(f, "LiveSplit"),
            TimerKind::WSplit => write!(f, "WSplit"),
            TimerKind::SplitterZ => write!(f, "SplitterZ"),
            TimerKind::ShitSplit => write!(f, "ShitSplit"),
            TimerKind::Splitty => write!(f, "Splitty"),
            TimerKind::TimeSplitTracker => write!(f, "Time Split Tracker"),
            TimerKind::Portal2LiveTimer => write!(f, "Portal 2 Live Timer"),
            TimerKind::FaceSplit => write!(f, "FaceSplit"),
            TimerKind::Flitter => write!(f, "Flitter"),
            TimerKind::Llanfair => write!(f, "Llanfair"),
            TimerKind::LlanfairGered => write!(f, "Llanfair (Gered's fork)"),
            TimerKind::Urn => write!(f, "Urn"),
            TimerKind::SourceLiveTimer => write!(f, "SourceLiveTimer"),
            TimerKind::Splitterino => write!(f, "Splitterino"),
            TimerKind::Generic(name) => write!(f, "{name}"),
        }
    }
}
