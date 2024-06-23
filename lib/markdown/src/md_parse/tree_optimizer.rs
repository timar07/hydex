use std::vec;
use crate::md_ast::Node;

/// Optimizes tree by removing unused nested nodes
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
            Node::Italic(child) => Self::visit_child_node(child, Node::Italic),
            Node::Bold(child) => Self::visit_child_node(child, Node::Bold),
            Node::Highlight(child) => Self::visit_child_node(child, Node::Highlight),
            Node::Strikethrough(child) => Self::visit_child_node(child, Node::Strikethrough),
            Node::Heading(n, child) => Self::visit_child_node(child, |c| Node::Heading(n, c)),
            Node::Code(_) | Node::Link { .. } | Node::Normal(_) => node,
        }
    }

    fn visit_child_node<F: FnOnce(Box<Node>) -> Node>(
        node: Box<Node>,
        constructor: F
    ) -> Node {
        constructor(Box::new(Self::visit_node(*node)))
    }
}
