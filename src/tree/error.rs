/// Errors for fallible operations.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum SgError {
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
    use crate::SgError;
    use std::mem::variant_count;

    #[test]
    fn test_err_var_cnt() {
        assert_eq!(variant_count::<SgError>(), 8);
    }
}
*/
