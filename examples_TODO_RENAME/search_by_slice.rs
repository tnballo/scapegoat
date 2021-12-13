use core::mem::size_of_val;

use scapegoat::SgSet;
use smallvec::{smallvec, SmallVec};

const U8_BUF_LEN: usize = 32;

// About:
// * Store 8-byte hexspeak words, e.g. values of type [u8; 8], in a set.
// * Query by hexspeak words of varying sizes, e.g. values of type &[u8].
fn main() {
    // Two hexspeak words
    let bad_code: [u8; 8] = [0xB, 0xA, 0xA, 0xD, 0xC, 0x0, 0xD, 0xE];
    let bad_food: [u8; 8] = [0xB, 0xA, 0xA, 0xD, 0xF, 0x0, 0x0, 0xD];

    // Note we're about to store uniformly sized values in our set
    assert_eq!(size_of_val(&bad_code), 8);
    assert_eq!(size_of_val(&bad_food), 8);

    // Store the two words in our set
    let mut set = SgSet::new();
    #[cfg(not(feature = "high_assurance"))]
    {
        set.insert(bad_code);
        set.insert(bad_food);
    }
    #[cfg(feature = "high_assurance")]
    {
        assert!(set.insert(bad_code).is_ok());
        assert!(set.insert(bad_food).is_ok());
    }

    // Vec<u8> is sized, it's actually a fat pointer to a heap buffer.
    // SmallVec<[u8; U8_BUF_LEN]> is sized, it's actually a stack buffer.
    // But slices of the vec are unsized! For example:
    //     &my_vec[0..5] is the first 5 elements
    //     &my_vec[1..] is all but the first element
    //     &my_vec[..] is all elements
    let bad_food_vec: SmallVec<[u8; U8_BUF_LEN]> =
        smallvec![0xB, 0xA, 0xA, 0xD, 0xF, 0x0, 0x0, 0xD];
    let bad_dude_vec: SmallVec<[u8; U8_BUF_LEN]> =
        smallvec![0xB, 0xA, 0xA, 0xD, 0xD, 0x0, 0x0, 0xD];

    // We're effectively searching for a [u8; 8] present
    assert_eq!(
        set.get(&bad_food_vec[..]), // 0xBAADFOOD
        Some(&[0xB, 0xA, 0xA, 0xD, 0xF, 0x0, 0x0, 0xD])
    );

    // We're effectively searching for a [u8; 4] not present
    assert_eq!(
        set.get(&bad_food_vec[..4]), // 0xBAAD
        None
    );

    // We're effectively searching for an [u8; 8] not present
    assert_eq!(
        set.get(&bad_dude_vec[..]), // 0xBAADDUDE
        None
    );
}
