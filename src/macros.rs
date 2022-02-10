#![deny(unused_results)]

/// Create an [`SgMap`][crate::map::SgMap] from a list of key-value pairs. Capacity precedes the list.
///
/// # Examples
///
/// ```
/// use scapegoat::{SgMap, sgmap};
///
/// let mut map = sgmap! {
///     4, // Const capacity
///     "a" => 0x61,
///     "b" => 0x62,
///     "c" => 0x63,
/// };
///
/// assert_eq!(map["a"], 0x61);
/// assert_eq!(map["b"], 0x62);
/// assert_eq!(map["c"], 0x63);
///
/// assert_eq!(map.get("d"), None);
/// assert_eq!(map.capacity(), 4);
/// assert_eq!(map.len(), 3);
///
/// map.insert("d", 0x64);
/// assert_eq!(map["d"], 0x64);
/// ```
#[macro_export]
macro_rules! sgmap {
    ( $capacity:expr $(, $key:expr => $value:expr)* $(,)? ) => {
        {
            let mut _sg_map = SgMap::<_,_, $capacity>::new();
            $(
                let _ = _sg_map.insert($key, $value);
            )*
            _sg_map
        }
    };
}

/// Create an [`SgSet`][crate::set::SgSet] from a list of values. Capacity precedes the list.
///
/// # Examples
///
/// ```
/// use scapegoat::{SgSet, sgset};
///
/// let mut set = sgset! {
///     4, // Const capacity
///     "a",
///     "b",
///     "c",
/// };
///
/// assert_eq!(set.get("d"), None);
/// assert_eq!(set.capacity(), 4);
/// assert_eq!(set.len(), 3);
///
/// set.insert("d");
/// assert_eq!(set.get("d"), Some(&"d"));
/// ```
#[macro_export]
macro_rules! sgset {
    ( $capacity:expr $(, $value:expr)* $(,)? ) => {
        {
            let mut _sg_set = SgSet::<_, $capacity>::new();
            $(
                let _ = _sg_set.insert($value);
            )*
            _sg_set
        }
    };
}
