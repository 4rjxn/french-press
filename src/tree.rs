#[derive(PartialEq, Debug)]
enum Node {
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
}
