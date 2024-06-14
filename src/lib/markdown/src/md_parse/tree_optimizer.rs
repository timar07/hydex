use std::vec;
use super::token::Node;

/// Optimizes tree by removing unused nested nodes and
/// merging same nodes
pub struct TreeOptimizer;

impl TreeOptimizer {
    pub fn optimize(root: Node) -> Node {
        Self::visit_node(root)
    }

    fn visit_node(node: Node) -> Node {
        match node {
            Node::TextRun(ref child) => {
                if child.len() == 1 {
                    return child[0].clone();
                }

                Self::merge(
                    child.iter()
                        .map(|node| {
                            Self::visit_node(node.clone())
                        })
                        .collect()
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
        if items.len() == 1 {
            return items[0].clone();
        }

        if items.len() == 2 {
            return Self::merge_two(items[0].clone(), items[1].clone());
        }

        Self::merge_two(
            items[0].clone(),
            Self::merge(items[1..].to_vec())
        )
    }

    fn merge_two(a: Node, b: Node) -> Node {
        match [&a, &b] {
            [Node::Normal(a), Node::Normal(b)] => Node::Normal(
                [a.clone(), b.clone()].join("")
            ),
            _ => {
                Node::TextRun(vec![a, b])
            }
        }
    }
}
