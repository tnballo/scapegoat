// TODO: update case to "SgErr" in v2.0.0
/// Errors for fallible operations, available when the `high_assurance` feature is enabled.
#[cfg(feature = "high_assurance")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SGErr {
    /// Requested insert or append operation cannot complete, stack storage is full.
    StackCapacityExceeded,

    /// Reserved for future use
    Reserved1,

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
