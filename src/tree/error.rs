// TODO: update case to "SgErr" in v2.0.0
/// Errors for fallible operations.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SGErr {
    /// Requested operation cannot complete, stack storage is full.
    StackCapacityExceeded,

    /*
    /// Requested operation cannot complete, heap storage is full.
    HeapCapacityExceeded,
    */
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

    /// Invalid rebalance factor requested, cannot set.
    RebalanceFactorOutOfRange,
}

/*

Requires nightly feature:

#[cfg(test)]
mod tests {
    use crate::SGErr;
    use std::mem::variant_count;

    #[test]
    fn test_err_var_cnt() {
        assert_eq!(variant_count::<SGErr>(), 8);
    }
}
*/
