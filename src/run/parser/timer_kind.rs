use core::fmt;

/// Describes the different Timers available that store splits files.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum TimerKind {
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
    /// The Rewrite of Llanfair
    Llanfair2,
    /// Urn
    Urn,
    /// SourceLiveTimer
    SourceLiveTimer,
    /// Worstrun
    Worstrun,
    /// Splitterino
    Splitterino,
    /// A Generic Timer. The name of the timer is associated with the variant.
    /// "Generic Timer" is used if there is no known name.
    Generic(String),
}

impl fmt::Display for TimerKind {
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
            TimerKind::Llanfair2 => write!(f, "Llanfair Rewrite"),
            TimerKind::Urn => write!(f, "Urn"),
            TimerKind::SourceLiveTimer => write!(f, "SourceLiveTimer"),
            TimerKind::Worstrun => write!(f, "worstrun"),
            TimerKind::Splitterino => write!(f, "Splitterino"),
            TimerKind::Generic(name) => write!(f, "{}", name),
        }
    }
}
