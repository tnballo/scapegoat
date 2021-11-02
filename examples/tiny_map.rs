#[allow(unused_imports)]
use core::mem::size_of;

#[allow(unused_imports)]
use scapegoat::{SGMap, SGSet};

// About:
// * Index packing saving 75% (7KB) of stack usage (the extreme case!).
// * Usage:
//      $ export SG_MAX_STACK_ELEMS=256
//      $ cargo run --example tiny_map
//      $ cargo run --example tiny_map --features="high_assurance"
fn main() {
    // This code assumes `SG_MAX_STACK_ELEMS == 256` (non-default)
    let temp: SGMap<u8, u8> = SGMap::new();
    if temp.capacity() == 256 {
        // Without packing
        #[cfg(target_pointer_width = "64")]
        #[cfg(not(feature = "high_assurance"))]
        #[cfg(not(feature = "fast_insert"))]
        {
            // Map of 256 (u8, u8) pairs
            assert_eq!(size_of::<SGMap<u8, u8>>(), 10_312);

            // Set of 256 u8 values
            // Unfortunately the internal () value in the pair is not optimized out, so same size as map
            assert_eq!(size_of::<SGSet<u8>>(), 10_312);

            // Moving up to a u16 without packing, both the set and map are actually the same size as the u8 case above!
            assert_eq!(size_of::<SGMap<u16, u16>>(), 10_312);
            assert_eq!(size_of::<SGSet<u16>>(), 10_312);
        }

        // With packing
        #[cfg(target_pointer_width = "64")]
        #[cfg(feature = "high_assurance")]
        #[cfg(not(feature = "fast_insert"))]
        {
            // Packed map of 256 (u8, u8) pairs
            assert_eq!(size_of::<SGMap<u8, u8>>(), 2_600);

            // Packed set of 256 u8 values
            // Unfortunately the internal () value in the pair is not optimized out, so same size as map
            assert_eq!(size_of::<SGSet<u8>>(), 2_600);

            // Moving up to a u16, we see alignment that allows optimization of the () value - set is smaller than map!
            assert_eq!(size_of::<SGMap<u16, u16>>(), 3_112);
            assert_eq!(size_of::<SGSet<u16>>(), 2_600);
        }
    }
}
