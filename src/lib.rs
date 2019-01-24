/// The simple union-find algorithm

use std::fmt;

/// The union-find algorithm trait
pub trait UnionFind {
    type K;

    /// union the union by index of vector
    /// param k1: key of one item in union
    /// param k2: key of one item in union
    fn union(&mut self, k1: Self::K, k2: Self::K);

    /// find is the two item in same union
    /// param k1: key of one item in union
    /// param k2: key of one item in union
    /// retval: is the two items in same union
    fn find(&self, k1: Self::K, k2: Self::K) -> bool;
}

/// UnionWeightedNode is the node of WQU union-find algorithm
///     - parent: the parent index of current item, None for root node.
///     - children: the children index of current item, Empty for leaf.
///     - value: the value of the node.
#[derive(Debug, PartialEq, Clone)]
pub struct UnionWeightedNode<V: PartialEq + fmt::Debug> {
    parent: Option<usize>,  // index of parent, None for root
    children: Vec<usize>,  // index of child, Empty for leaf
    value: Option<V>,  // payload value of this Node
}

impl<V: PartialEq + fmt::Debug> Default for UnionWeightedNode<V> {
    fn default() -> Self {
        UnionWeightedNode {
            parent: None,
            children: vec![],
            value: None,
        }
    }
}

/// UnionWeighted is the data structure of WQU union-find algorithm
///     Represent union by tree in vector.
#[derive(Debug)]
pub struct UnionWeighted<V: PartialEq + fmt::Debug> {
    nodes: Vec<UnionWeightedNode<V>>,
}

impl<V: PartialEq + fmt::Debug> Default for UnionWeighted<V> {
    fn default() -> Self {
        UnionWeighted {
            nodes: vec![],
        }
    }
}


impl<V: PartialEq + fmt::Debug> UnionFind for UnionWeighted<V> {
    type K = usize;

    /// union the union by index of vector
    /// param k1: key of one item in union
    /// param k2: key of one item in union
    fn union(&mut self, k1: Self::K, k2: Self::K) {
        if self.find(k1, k2) {
            return;  // in one union already
        }

        // not well when k1 or k2 is lowset root
        let ((root1, depth1), (root2, depth2)) =
            (self.get_root_with_depth(k1), self.get_root_with_depth(k2));
        if depth1 < depth2 {  // link shorter tree follow to longer
            self.nodes[root1].parent = Some(root2);
            self.nodes[root2].children.push(root1);
        } else {
            self.nodes[root2].parent = Some(root1);
            self.nodes[root1].children.push(root2);
        }
    }

    /// find is the two item in same union
    /// param k1: key of one item in union
    /// param k2: key of one item in union
    /// retval: is the two items in same union
    fn find(&self, k1: Self::K, k2: Self::K) -> bool {
        let (root1, root2) = (self.get_root(k1), self.get_root(k2));
        if root1 == root2 {
            true
        } else {
            false
        }
    }
}

macro_rules! full {
    ( $x: expr ) => {
        {
            let mut temp_v = Vec::with_capacity($x);
            for _ in 0..$x {
                temp_v.push(UnionWeightedNode::default());
            }
            temp_v
        }
    }
}

impl<V: PartialEq + fmt::Debug> UnionWeighted<V> {

    /// create a UnionWeighted
    /// param count: the capacity of union
    /// retval: UnionWeighted
    pub fn new(count: usize) -> Self {
        UnionWeighted {
            nodes: Vec::with_capacity(count),
        }
    }

    /// create a UnionWeighted with category
    /// param count: the capacity of union
    /// retval: UnionWeighted full catgoried
    pub fn new_full_category(count: usize) -> Self {
        UnionWeighted {
            nodes: full!(count),
        }
    }

    /// push new value to new union
    /// param v: the value to push into
    pub fn push(&mut self, v: V) {
        self.nodes.push(UnionWeightedNode {
            parent: None,
            children: vec![],
            value: Some(v),
        });
    }

    /// insert new value to exist union
    /// param index: the union index which new value join to
    /// param v: the new value
    pub fn insert(&mut self, index: usize, v: V) {
        if index >= self.nodes.len() {
            panic!("Insert to invalid union!");
        }

        let new = self.nodes.len();  // the index of the new item
        let root = self.get_root(index);  // the root of union which join to

        self.nodes.push(UnionWeightedNode {
            parent: Some(root),
            children: vec![],
            value: Some(v),
        });
        self.nodes[root].children.push(new);
    }

    /// get the capacity of the union items
    pub fn capacity(&self) -> usize {
        self.nodes.capacity()
    }

    /// get the length of the union items
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// get the root of one union tree
    /// param index: the index of current id value in vector
    /// retval: the index of root of current union tree
    pub fn get_root(&self, index: usize) -> usize {
        if let Some(p) = self.nodes[index].parent {
            self.get_root(p)
        } else {
            index
        }
    }

    // get the depth of union from root
    // param root: the index of root of the union
    // param depth: the depth in current recursive stage
    // retval: the depth of current union tree
    fn get_depth_4_root(
            &self,
            root: &UnionWeightedNode<V>,
            depth: usize
    ) -> usize {
        const SAMPLES_COUNT: usize = 4;  // force to 4 samples when too long
        const STEP_MIN: usize = 4;
        let mut max: usize = 0;  // maximum depth of children
        // Sampling instead of iterating each to optimize time cost
        // 1. step = 1, step_count = N, when N < SAMPLES_COUNT*STEP_MIN
        // 2. step = N%SAMPLES_COUNT, step_count = SAMPLES_COUNT, other
        for child in root.children.iter().step_by(
            if root.children.len() > SAMPLES_COUNT*STEP_MIN {
                root.children.len() / SAMPLES_COUNT
            } else {
                1
            }
        ) {
            let children_depth = self.get_depth_4_root(
                    &self.nodes[*child],
                    depth + 1
            );
            if max < children_depth {
                max = children_depth;
            }
        }

        max + depth
    }

    /// get the root and the depth of tree
    /// param index: the index of current id value in vector
    /// retval: (root, depth) of current union
    pub fn get_root_with_depth(&self, index: usize) -> (usize, usize) {
        let root = self.get_root(index);
        let depth = self.get_depth_4_root(&self.nodes[root], 0);
        (root, depth)
    }
}
