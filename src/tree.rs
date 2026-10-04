use std::collections::BinaryHeap;

use crate::tree::Node::Leaf;

#[derive(Debug)]
pub enum Node {
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

impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency() == other.frequency()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.frequency().cmp(&self.frequency())
    }
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
        node_heap.push(leaf);
    }
    while node_heap.len() > 1 {
        let left = node_heap.pop().unwrap();
        let right = node_heap.pop().unwrap();
        let merged_node = Node::Internal {
            frequency: left.frequency() + right.frequency(),
            left: Box::new(left),
            right: Box::new(right),
        };
        node_heap.push(merged_node);
    }
    node_heap.pop()
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
        frequency[b'a' as usize] = 500;
        frequency[b'b' as usize] = 5;
        frequency[b'c' as usize] = 7;
        let tree = build_tree(&frequency).unwrap();
        assert_eq!(tree.frequency(), 512);

        let Node::Internal { .. } = &tree else {
            panic!("root node should be Internal");
        };

        let expected_tree = Node::Internal {
            frequency: 512,
            left: Box::new(Node::Leaf {
                byte: b'a',
                frequency: 500,
            }),
            right: Box::new(Node::Internal {
                frequency: 12,
                left: Box::new(Node::Leaf {
                    byte: b'b',
                    frequency: 5,
                }),
                right: Box::new(Node::Leaf {
                    byte: b'c',
                    frequency: 7,
                }),
            }),
        };
        assert_eq!(tree, expected_tree)
    }
}
