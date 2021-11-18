#![no_main]
use std::collections::BTreeMap;

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) -> isize {
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    map.insert(1, 2);
    assert_eq!(map.get(&1), Some(&2));
    assert_eq!(map.remove(&1), Some(2));
    return 0;
}