// TODO: update case to "SgErr" in v2.0.0
/// Errors for fallible operations.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SGErr {
    /// Requested operation cannot complete, stack storage is full.
    /// This error is unique to the `high_assurance` feature.
    StackCapacityExceeded,

    /// Invalid rebalance factor requested, cannot set.
    RebalanceFactorOutOfRange,

    /// Reserved for future use
    Reserved2,

    /// Reserved for future use
    Reserved3,

    /// Reserved for future use
    Reserved4,

    /// Reserved for future use
    Reserved5,

    /// Reserved for future use
    Reserved6,

    /// Reserved for future use
    Reserved7,
}
