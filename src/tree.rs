use std::{cmp::Reverse, collections::BinaryHeap};

use crate::tree::Node::Leaf;

#[derive(PartialEq, Debug, Eq, PartialOrd, Ord)]
pub(crate) enum Node {
    Leaf {
        byte: u8,
        frequency: u64,
    },
    Internal {
        frequency: u64,
        left: Box<Node>,
        right: Box<Node>,
    },
}

impl Node {
    pub fn frequency(&self) -> u64 {
        match self {
            Leaf { frequency, .. } => *frequency,
            Node::Internal { frequency, .. } => *frequency,
        }
    }
    pub fn left(&self) -> Option<&Node> {
        match self {
            Leaf { .. } => None,
            Node::Internal { left, .. } => Some(left),
        }
    }
    pub fn right(&self) -> Option<&Node> {
        match self {
            Leaf { .. } => None,
            Node::Internal { right, .. } => Some(right),
        }
    }
}

fn generate_leaf_nodes(frequencies: &[u64; 256]) -> Vec<Node> {
    let mut leaf_nodes = vec![];
    for (index, &freq) in frequencies.iter().enumerate() {
        if freq == 0 {
            continue;
        }
        let leaf = Node::Leaf {
            byte: index as u8,
            frequency: freq,
        };
        leaf_nodes.push(leaf);
    }
    leaf_nodes
}

pub fn build_tree(frequencies: &[u64; 256]) -> Option<Node> {
    let leaf_nodes = generate_leaf_nodes(frequencies);
    let mut node_heap = BinaryHeap::new();
    for leaf in leaf_nodes {
        node_heap.push(Reverse(leaf));
    }
    while node_heap.len() > 1 {
        let Reverse(left) = node_heap.pop().unwrap();
        let Reverse(right) = node_heap.pop().unwrap();
        let merged_node = Node::Internal {
            frequency: left.frequency() + right.frequency(),
            left: Box::new(left),
            right: Box::new(right),
        };
        node_heap.push(Reverse(merged_node));
    }
    node_heap.pop().map(|Reverse(root)| root)
}

#[cfg(test)]
mod tree_tester {

    use super::*;

    #[test]
    fn generate_leaf_for_empty_frequiencies() {
        let leaf_nodes = generate_leaf_nodes(&[0u64; 256]);
        assert!(leaf_nodes.is_empty());
    }

    #[test]
    fn generate_leaf_nodes_for_sample() {
        let mut frequencies = [0u64; 256];
        frequencies[1] = 55;
        frequencies[20] = 575;
        frequencies[200] = 5;
        let leaf_nodes = generate_leaf_nodes(&frequencies);
        assert!(leaf_nodes.len() == 3);
        assert_eq!(
            leaf_nodes[0],
            Node::Leaf {
                byte: 1,
                frequency: 55
            }
        )
    }

    #[test]
    fn build_tree_on_empty_frequencies() {
        let frequency = [0u64; 256];
        let tree = build_tree(&frequency);
        assert_eq!(tree, None);
    }

    #[test]
    fn build_tree_on_single_frequency() {
        let mut frequency = [0u64; 256];
        frequency[55] = 55;
        let tree = build_tree(&frequency).unwrap();
        assert_eq!(
            tree,
            Node::Leaf {
                byte: 55,
                frequency: 55
            }
        );
    }

    #[test]
    fn build_tree_on_frequencies() {
        let mut frequency = [0u64; 256];
        frequency[b'a' as usize] = 1;
        frequency[b'b' as usize] = 5;
        frequency[b'c' as usize] = 7;
        let tree = build_tree(&frequency).unwrap();
        assert_eq!(tree.frequency(), 13);

        let Node::Internal { left, right, .. } = &tree else {
            panic!("root node should be Internal");
        };
        let c_is_direct_child = matches!(**left, Node::Leaf { byte: b'c', .. })
            || matches!(**right, Node::Leaf { byte: b'c', .. });
        assert!(c_is_direct_child);
    }
}
