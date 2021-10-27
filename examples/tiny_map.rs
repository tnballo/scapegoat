use core::mem::size_of;
use scapegoat::{SGMap, SGSet};

// Index packing saving 75% (9KB) of stack usage (the extreme case!).
//
// Usage:
//
// $ export SG_MAX_STACK_ELEMS=256
// $ cargo run --example tiny_map
// $ cargo run --example tiny_map --features="high_assurance"
fn main() {
    // This code assumes `SG_MAX_STACK_ELEMS == 256` (non-default)
    let temp: SGMap<u8, u8> = SGMap::new();
    if temp.capacity() == 256 {
        // Without packing
        #[cfg(target_pointer_width = "64")]
        #[cfg(not(feature = "high_assurance"))]
        {
            // Map of 256 (u8, u8) pairs
            assert_eq!(size_of::<SGMap<u8, u8>>(), 12_376);

            // Set of 256 u8 values
            // Unfortunately the internal () in the pair is not optimized out, so same size as map
            assert_eq!(size_of::<SGSet<u8>>(), 12_376);
        }

        // With packing
        #[cfg(target_pointer_width = "64")]
        #[cfg(feature = "high_assurance")]
        {
            // Packed map of 256 (u8, u8) pairs
            assert_eq!(size_of::<SGMap<u8, u8>>(), 3_128);

            // Packed set of 256 u8 values
            // Unfortunately the internal () in the pair is not optimized out, so same size as map
            assert_eq!(size_of::<SGSet<u8>>(), 3_128);
        }
    }
}