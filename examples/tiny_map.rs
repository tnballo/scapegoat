use core::mem::size_of_val;
use scapegoat::SgMap;

fn main() {
    let tiny_map: SgMap<u8, u8, 128> = SgMap::new();

    // Default configuration
    #[cfg(target_pointer_width = "64")]
    #[cfg(not(feature = "fast_rebalance"))]
    #[cfg(not(feature = "low_mem_insert"))]
    {
        assert_eq!(size_of_val(&tiny_map), 1_608);
    }

    // Optimizing for low stack footprint
    #[cfg(target_pointer_width = "64")]
    #[cfg(not(feature = "fast_rebalance"))]
    #[cfg(feature = "low_mem_insert")]
    {
        assert_eq!(size_of_val(&tiny_map), 1_352);
    }
}
