use std::mem::{self, MaybeUninit};

/// B-Tree of degree D+1
pub struct BTree<const D: usize> {
    root: NodeIndex,
    nodes: Vec<Node<D>>,
}

pub struct NodeIndex(u32);

struct Node<const D: usize> {
    len: usize,
    keys: [MaybeUninit<(Box<[u8]>, Box<[u8]>)>; D],
    first_child: MaybeUninit<NodeIndex>,
    other_children: [MaybeUninit<NodeIndex>; D],
}

impl<const D: usize> BTree<D> {
    pub fn new() -> BTree<D> {
        BTree {
            root: NodeIndex(0),
            nodes: vec![Node::default()],
        }
    }

    // pub fn insert(&mut self, )
}

impl<const D: usize> Node<D> {
    fn new() -> Self {
        Self {
            len: 0,
            keys: unsafe { MaybeUninit::uninit().assume_init() },
            first_child: unsafe { MaybeUninit::uninit().assume_init() },
            other_children: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }
}

impl<const D: usize> Default for Node<D> {
    fn default() -> Self {
        Self::new()
    }
}
