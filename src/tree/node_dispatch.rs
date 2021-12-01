use super::node::Node;

// Enum Dispatch -------------------------------------------------------------------------------------------------------

/// Encapsulate `U`.
pub trait SmallNode<K,V> {
    /// Get left index as `usize`.
    fn left_idx(&self) -> Option<usize>;

    /// Set left index.
    fn set_left_idx(&mut self, opt_idx: Option<usize>);

    /// Get right index as `usize`.
    fn right_idx(&self) -> Option<usize>;

    /// Set right index.
    fn set_right_idx(&mut self, opt_idx: Option<usize>);
}

// TODO: add to smallnum as macro with const -> UINT?
// so caller can to `small_unsigned!(my number)` and `small_unsigned_type!(MY_NUM)`
pub enum UINT {
    UINT,
    U8,
    U16,
    U32,
    U64,
    U128,
}

pub enum EnumSmallNode<K, V> {
    UINT(Node<K, V, usize>),
    U8(Node<K, V, u8>),
    U16(Node<K, V, u16>),
    U32(Node<K, V, u32>),
    U64(Node<K, V, u64>),
    U128(Node<K, V, u128>),
}

impl<K, V> EnumSmallNode<K, V> {
    fn new(key: K, val: V, uint: UINT) -> Self {
        match uint {
            UINT::UINT => EnumSmallNode::UINT(Node::<K,V, usize>::new(key, val)),
            UINT::U8 => EnumSmallNode::U8(Node::<K,V, u8>::new(key, val)),

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            UINT::U16 => EnumSmallNode::U16(Node::<K,V, u16>::new(key, val)),

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            UINT::U32 => EnumSmallNode::U32(Node::<K,V, u32>::new(key, val)),

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            UINT::U64 => EnumSmallNode::U64(Node::<K,V, u64>::new(key, val)),

            #[cfg(target_pointer_width = "128")]
            UINT::U128 => EnumSmallNode::U128(Node::<K,V, u128>::new(key, val)),
        }
    }
}

// TODO: write a macro these method bodies, to reduce boilerplate!
impl<K, V> SmallNode<K, V> for EnumSmallNode<K, V> {
    fn left_idx(&self) -> Option<usize> {
        match self {
            EnumSmallNode::UINT(node) => node.left_idx(),
            EnumSmallNode::U8(node) => node.left_idx(),

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            EnumSmallNode::U16(node) => node.left_idx(),

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            EnumSmallNode::U32(node) => node.left_idx(),

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            EnumSmallNode::U64(node) => node.left_idx(),

            #[cfg(target_pointer_width = "128")]
            EnumSmallNode::U128(node) => node.left_idx(),
        }
    }

    fn set_left_idx(&mut self, opt_idx: Option<usize>) {
        match self {
            EnumSmallNode::UINT(node) => node.set_left_idx(opt_idx),
            EnumSmallNode::U8(node) => node.set_left_idx(opt_idx),

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            EnumSmallNode::U16(node) => node.set_left_idx(opt_idx),

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            EnumSmallNode::U32(node) => node.set_left_idx(opt_idx),

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            EnumSmallNode::U64(node) => node.set_left_idx(opt_idx),

            #[cfg(target_pointer_width = "128")]
            EnumSmallNode::U128(node) => node.set_left_idx(opt_idx),
        }
    }

    fn right_idx(&self) -> Option<usize> {
        match self {
            EnumSmallNode::UINT(node) => node.right_idx(),
            EnumSmallNode::U8(node) => node.right_idx(),

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            EnumSmallNode::U16(node) => node.right_idx(),

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            EnumSmallNode::U32(node) => node.right_idx(),

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            EnumSmallNode::U64(node) => node.right_idx(),

            #[cfg(target_pointer_width = "128")]
            EnumSmallNode::U128(node) => node.right_idx(),
        }
    }

    fn set_right_idx(&mut self, opt_idx: Option<usize>) {
        match self {
            EnumSmallNode::UINT(node) => node.set_right_idx(opt_idx),
            EnumSmallNode::U8(node) => node.set_right_idx(opt_idx),

            #[cfg(any(
                target_pointer_width = "16",
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            EnumSmallNode::U16(node) => node.set_right_idx(opt_idx),

            #[cfg(any(
                target_pointer_width = "32",
                target_pointer_width = "64",
                target_pointer_width = "128",
            ))]
            EnumSmallNode::U32(node) => node.set_right_idx(opt_idx),

            #[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
            EnumSmallNode::U64(node) => node.set_right_idx(opt_idx),

            #[cfg(target_pointer_width = "128")]
            EnumSmallNode::U128(node) => node.set_right_idx(opt_idx),
        }

    }
}