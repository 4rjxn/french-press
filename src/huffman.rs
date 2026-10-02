use crate::tree::Node;

#[derive(Default, Clone, Copy)]
struct Code {
    bits: u64,
    len: u8,
}

fn generate_codes(node: &Node, bits: u64, len: u8, table: &mut [Code; 256]) {
    match node {
        Node::Leaf { byte, .. } => {
            let len = len.max(1);
            table[*byte as usize] = Code { bits: bits, len };
        }
        Node::Internal { left, right, .. } => {
            generate_codes(left, bits << 1, len + 1, table);
            generate_codes(right, (bits << 1) | 1, len + 1, table);
        }
    }
}

#[cfg(test)]
mod huffman_tester {
    use super::*;

    fn test_tree() -> Node {
        Node::Internal {
            frequency: 35,
            left: Box::new(Node::Leaf {
                byte: b'a',
                frequency: 35,
            }),
            right: Box::new(Node::Leaf {
                byte: b'b',
                frequency: 0,
            }),
        }
    }

    #[test]
    fn single_node_tree() {
        let node = Node::Leaf {
            byte: b'a',
            frequency: 35,
        };
        let mut code_tables = [Code::default(); 256];
        generate_codes(&node, 0, 0, &mut code_tables);
        assert_eq!(code_tables[b'a' as usize].len, 1);
    }

    #[test]
    fn test_node_tree() {
        let mut code_tables = [Code::default(); 256];
        generate_codes(&test_tree(), 0, 0, &mut code_tables);
        assert_eq!(code_tables[b'a' as usize].len, 1);
        assert_eq!(code_tables[b'b' as usize].len, 1);
    }
}
