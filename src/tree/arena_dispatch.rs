use super::node::NodeGetHelper;
use super::node_dispatch::SmallNodeDispatch;

use smallvec::SmallVec;

// TODO: move to arena.rs

// Size-optimized Arena Trait -------------------------------------------------------------------------------------------

/// Interface encapsulates `U`.
pub trait SmallArena<K: Default, V: Default, const N: usize> {
    /// `#![no_std]`: total capacity, e.g. maximum number of items.
    /// Attempting to insert items beyond capacity will panic.
    ///
    /// If using `std`: fast capacity, e.g. number of map items stored on the stack.
    /// Items inserted beyond capacity will be stored on the heap.
    fn capacity(&self) -> usize;

    /// Add node to area, growing if necessary, and return addition index.
    fn add(&mut self, key: K, val: V) -> usize;

    /// Remove node at a given index from area, return it.
    fn remove(&mut self, idx: usize) -> Option<SmallNodeDispatch<K, V>>;

    /// Remove node at a known-good index (simpler callsite and error handling) from area.
    /// This function can panic. If the index might be invalid, use `remove` instead.
    fn hard_remove(&mut self, idx: usize) -> SmallNodeDispatch<K, V>;

    /// Sort the arena in caller-requested order and update all tree metadata accordingly
    /// `unwraps` will never panic if caller invariants upheld (checked via `debug_assert`)
    fn sort(
        &mut self,
        root_idx: usize,
        sort_metadata: SmallVec<[NodeGetHelper<usize>; N]>, // `usize` instead of `U` avoids `U` in tree iter sigs
    ) -> usize;

    /// Returns the number of entries in the arena, some of which may be `None`.
    fn len(&self) -> usize;

    /// Get the size of an individual arena node, in bytes.
    fn node_size(&self) -> usize;
}

// TODO: remove everything below this line? With "stack_pack" feature.
/*
// Enum Dispatch -------------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub enum SmallArenaDispatch<K: Default, V: Default, const N: usize> {
    ArenaUSIZE(Arena<K, V, usize, N>),
    ArenaU8(Arena<K, V, u8, N>),

    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64",
        target_pointer_width = "128",
    ))]
    ArenaU16(Arena<K, V, u16, N>),

    #[cfg(any(
        target_pointer_width = "32",
        target_pointer_width = "64",
        target_pointer_width = "128",
    ))]
    ArenaU32(Arena<K, V, u32, N>),

    #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
    ArenaU64(Arena<K, V, u64, N>),

    #[cfg(target_pointer_width = "128")]
    ArenaU128(Arena<K, V, u128, N>),
}

impl<K: Default, V: Default, const N: usize> SmallArenaDispatch<K, V, N> {
    pub fn new(uint: SmallUnsignedLabel) -> Self {
        match uint {
            SmallUnsignedLabel::USIZE => SmallArenaDispatch::ArenaUSIZE(Arena::<K, V, usize, N>::new()),
            SmallUnsignedLabel::U8 => SmallArenaDispatch::ArenaU8(Arena::<K, V, u8, N>::new()),

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallUnsignedLabel::U16 => SmallArenaDispatch::ArenaU16(Arena::<K, V, u16, N>::new()),

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallUnsignedLabel::U32 => SmallArenaDispatch::ArenaU32(Arena::<K, V, u32, N>::new()),

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            SmallUnsignedLabel::U64 => SmallArenaDispatch::ArenaU64(Arena::<K, V, u64, N>::new()),

            #[cfg(target_pointer_width = "128")]
            SmallUnsignedLabel::U128 => SmallArenaDispatch::ArenaU128(Arena::<K, V, u128, N>::new()),

            _ => unreachable!()
        }
    }
}

macro_rules! dispatch {
    ( $self:ident, $func:ident $(, $args:expr)* $(,)? ) => {
        match $self {
            SmallArenaDispatch::ArenaUSIZE(arena) => arena.$func($($args,)*),
            SmallArenaDispatch::ArenaU8(arena) => arena.$func($($args,)*),

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallArenaDispatch::ArenaU16(arena) => arena.$func($($args,)*),

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallArenaDispatch::ArenaU32(arena) => arena.$func($($args,)*),

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            SmallArenaDispatch::ArenaU64(arena) => arena.$func($($args,)*),

            #[cfg(target_pointer_width = "128")]
            SmallArenaDispatch::ArenaU128(arena) => arena.$func($($args,)*),
        }
    };
}

impl<K: Default, V: Default, const N: usize> SmallArena<K, V, N> for SmallArenaDispatch<K, V, N> {
    fn capacity(&self) -> usize {
        dispatch!(self, capacity)
    }

    fn iter(&self) -> Iter<'_, Option<SmallNodeDispatch<K, V>>> {
        dispatch!(self, iter)
    }

    fn iter_mut(&mut self) -> IterMut<'_, Option<SmallNodeDispatch<K, V>>> {
        dispatch!(self, iter_mut)
    }

    fn add(&mut self, node: SmallNodeDispatch<K, V>) -> usize {
        dispatch!(self, add, node)
    }

    fn remove(&mut self, idx: usize) -> Option<SmallNodeDispatch<K, V>> {
        dispatch!(self, remove, idx)
    }

    fn hard_remove(&mut self, idx: usize) -> SmallNodeDispatch<K, V> {
        dispatch!(self, hard_remove, idx)
    }

    fn sort(
        &mut self,
        root_idx: usize,
        sort_metadata: SmallVec<[NodeGetHelper<usize>; N]>,
    ) -> usize {
        dispatch!(self, sort, root_idx, sort_metadata)
    }

    fn len(&self) -> usize {
        dispatch!(self, len)
    }

    fn node_size(&self) -> usize {
        dispatch!(self, node_size)
    }
}

// Convenience Traits --------------------------------------------------------------------------------------------------

/// Immutable indexing.
/// Indexed location MUST be occupied.
/// This adds some overhead because enum variant must be matched to access inner.
impl<K: Default, V: Default, const N: usize> Index<usize> for SmallArenaDispatch<K, V, N> {
    type Output = SmallNodeDispatch<K, V>;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            SmallArenaDispatch::ArenaUSIZE(arena) => &arena[index],
            SmallArenaDispatch::ArenaU8(arena) => &arena[index],

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallArenaDispatch::ArenaU16(arena) => &arena[index],

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallArenaDispatch::ArenaU32(arena) => &arena[index],

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            SmallArenaDispatch::ArenaU64(arena) => &arena[index],

            #[cfg(target_pointer_width = "128")]
            SmallArenaDispatch::ArenaU128(arena) => &arena[index],
        }
    }
}

/// Mutable indexing
/// Indexed location MUST be occupied.
/// This adds some overhead because enum variant must be matched to access inner.
impl<K: Default, V: Default, const N: usize> IndexMut<usize> for SmallArenaDispatch<K, V, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            SmallArenaDispatch::ArenaUSIZE(arena) => &mut arena[index],
            SmallArenaDispatch::ArenaU8(arena) => &mut arena[index],

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallArenaDispatch::ArenaU16(arena) => &mut arena[index],

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallArenaDispatch::ArenaU32(arena) => &mut arena[index],

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            SmallArenaDispatch::ArenaU64(arena) => &mut arena[index],

            #[cfg(target_pointer_width = "128")]
            SmallArenaDispatch::ArenaU128(arena) => &mut arena[index],
        }
    }
}
*/