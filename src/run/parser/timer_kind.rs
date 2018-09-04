use std::fmt;

/// Describes the different Timers available that store splits files.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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
    /// Generic Splits I/O Timer
    GenericSplitsIO,
}

impl fmt::Display for TimerKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TimerKind::LiveSplit => write!(f, "LiveSplit"),
            TimerKind::WSplit => write!(f, "WSplit"),
            TimerKind::SplitterZ => write!(f, "SplitterZ"),
            TimerKind::ShitSplit => write!(f, "ShitSplit"),
            TimerKind::Splitty => write!(f, "Splitty"),
            TimerKind::TimeSplitTracker => write!(f, "Time Split Tracker"),
            TimerKind::Portal2LiveTimer => write!(f, "Portal 2 Live Timer"),
            TimerKind::FaceSplit => write!(f, "FaceSplit"),
            TimerKind::Llanfair => write!(f, "Llanfair"),
            TimerKind::LlanfairGered => write!(f, "Llanfair (Gered's fork)"),
            TimerKind::Llanfair2 => write!(f, "Llanfair Rewrite"),
            TimerKind::Urn => write!(f, "Urn"),
            TimerKind::SourceLiveTimer => write!(f, "SourceLiveTimer"),
            TimerKind::Worstrun => write!(f, "worstrun"),
            TimerKind::GenericSplitsIO => write!(f, "Generic Splits I/O Timer"),
        }
    }
}
