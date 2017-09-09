use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TimerKind {
    LiveSplit,
    WSplit,
    SplitterZ,
    ShitSplit,
    Splitty,
    TimeSplitTracker,
    Portal2LiveTimer,
    FaceSplit,
    Llanfair,
    LlanfairGered,
    Llanfair2,
    Urn,
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
        }
    }
}
