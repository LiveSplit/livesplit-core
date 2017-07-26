#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum DigitsFormat {
    /// 1
    SingleDigitSeconds,
    /// 01
    DoubleDigitSeconds,
    /// 0:01
    SingleDigitMinutes,
    /// 00:01
    DoubleDigitMinutes,
    /// 0:00:01
    SingleDigitHours,
    /// 00:00:01
    DoubleDigitHours,
}
