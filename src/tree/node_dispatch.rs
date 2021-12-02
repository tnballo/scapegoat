use super::node::Node;
use smallnum::SmallUnsignedLabel;

// Size-optimized Node Trait -------------------------------------------------------------------------------------------

/// Interfaces encapsulates `U`.
pub trait SmallNode<K, V> {
    /// Get key.
    fn key(&self) -> K;

    /// Set key.
    fn set_key(&mut self, key: K);

    /// Get value.
    fn val(&self) -> V;

    /// Set value.
    fn set_val(&mut self, val: V);

    /// Get left index as `usize`.
    fn left_idx(&self) -> Option<usize>;

    /// Set left index.
    fn set_left_idx(&mut self, opt_idx: Option<usize>);

    /// Get right index as `usize`.
    fn right_idx(&self) -> Option<usize>;

    /// Set right index.
    fn set_right_idx(&mut self, opt_idx: Option<usize>);

    /// Get subtree size.
    #[cfg(feature = "fast_rebalance")]
    fn subtree_size(&self) -> usize;

    /// Set subtree size.
    #[cfg(feature = "fast_rebalance")]
    fn set_subtree_size(&mut self, size: usize);
}

// Enum Dispatch -------------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub enum SmallNodeDispatch<K, V> {
    USIZE(Node<K, V, usize>),
    U8(Node<K, V, u8>),
    U16(Node<K, V, u16>),
    U32(Node<K, V, u32>),
    U64(Node<K, V, u64>),
    U128(Node<K, V, u128>),
}

impl<K, V> SmallNodeDispatch<K, V> {
    pub const fn new(key: K, val: V, uint: SmallUnsignedLabel) -> Self {
        match uint {
            USIZE => SmallNodeDispatch::USIZE(Node::<K, V, usize>::new(key, val)),
            U8 => SmallNodeDispatch::U8(Node::<K, V, u8>::new(key, val)),

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            U16 => SmallNodeDispatch::U16(Node::<K, V, u16>::new(key, val)),

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            U32 => SmallNodeDispatch::U32(Node::<K, V, u32>::new(key, val)),

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            U64 => SmallNodeDispatch::U64(Node::<K, V, u64>::new(key, val)),

            #[cfg(target_pointer_width = "128")]
            U128 => SmallNodeDispatch::U128(Node::<K, V, u128>::new(key, val)),
        }
    }
}

macro_rules! dispatch_args {
    ( $self:ident, $func:ident, $args:expr $(,)? ) => {
        match $self {
            SmallNodeDispatch::USIZE(node) => node.$func($args),
            SmallNodeDispatch::U8(node) => node.$func($args),

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallNodeDispatch::U16(node) => node.$func($args),

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallNodeDispatch::U32(node) => node.$func($args),

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            SmallNodeDispatch::U64(node) => node.$func($args),

            #[cfg(target_pointer_width = "128")]
            SmallNodeDispatch::U128(node) => node.$func($args),
        }
    };
}

macro_rules! dispatch_no_args {
    ( $self:ident, $func:ident $(,)? ) => {
        match $self {
            SmallNodeDispatch::USIZE(node) => node.$func(),
            SmallNodeDispatch::U8(node) => node.$func(),

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallNodeDispatch::U16(node) => node.$func(),

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            SmallNodeDispatch::U32(node) => node.$func(),

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            SmallNodeDispatch::U64(node) => node.$func(),

            #[cfg(target_pointer_width = "128")]
            SmallNodeDispatch::U128(node) => node.$func(),
        }
    };
}

impl<K, V> SmallNode<K, V> for SmallNodeDispatch<K, V> {
    fn key(&self) -> K {
        dispatch_no_args!(self, key)
    }

    fn set_key(&mut self, key: K) {
        dispatch_args!(self, set_key, key);
    }

    fn val(&self) -> V {
        dispatch_no_args!(self, val)
    }

    fn set_val(&mut self, val: V) {
        dispatch_args!(self, set_val, val);
    }

    fn left_idx(&self) -> Option<usize> {
        dispatch_no_args!(self, left_idx)
    }

    fn set_left_idx(&mut self, opt_idx: Option<usize>) {
        dispatch_args!(self, set_left_idx, opt_idx);
    }

    fn right_idx(&self) -> Option<usize> {
        dispatch_no_args!(self, right_idx)
    }

    fn set_right_idx(&mut self, opt_idx: Option<usize>) {
        dispatch_args!(self, set_right_idx, opt_idx);
    }

    #[cfg(feature = "fast_rebalance")]
    fn subtree_size(&self) -> usize {
        dispatch_no_args!(self, subtree_size)
    }

    #[cfg(feature = "fast_rebalance")]
    fn set_subtree_size(&mut self, size: usize) {
        dispatch_args!(self, set_subtree_size, size);
    }
}