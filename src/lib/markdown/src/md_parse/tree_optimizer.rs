use std::vec;
use super::node::Node;

/// Optimizes tree by removing unused nested nodes and
/// merging same nodes
pub struct TreeOptimizer;

impl TreeOptimizer {
    pub fn optimize(root: Node) -> Node {
        Self::visit_node(root)
    }

    fn visit_node(node: Node) -> Node {

        match node {
            Node::TextRun(ref children) => {
                if children.len() == 1 {
                    return Self::visit_node(children[0].clone());
                }

                Node::TextRun(
                    children.iter()
                        .map(|node| {
                            let node = Self::visit_node(node.clone());

                            if let Node::TextRun(children) = node {
                                return children;
                            }

                            vec![node]
                        })
                        .flatten()
                        .collect::<Vec<Node>>()
                )
            }
            // TODO: Remove duplicating code
            Node::Italic(child) => Node::Italic(
                Box::new(Self::visit_node(*child))
            ),
            Node::Bold(child) => Node::Bold(
                Box::new(Self::visit_node(*child))
            ),
            Node::Highlight(child) => Node::Highlight(
                Box::new(Self::visit_node(*child))
            ),
            _ => node
        }
    }

    fn merge(items: Vec<Node>) -> Node {
        match items.len() {
            1 => items[0].clone(),
            2 => Self::merge_two(
                items[0].clone(), items[1].clone()
            ),
            _ => Self::merge_two(
                items[0].clone(),
                Self::merge(items[1..].to_vec())
            )
        }
    }

    fn merge_two(a: Node, b: Node) -> Node {
        a + b
    }
}
