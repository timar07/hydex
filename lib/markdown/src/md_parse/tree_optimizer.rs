use std::vec;
use crate::md_ast::Node;

/// Optimizes tree by removing unused nested nodes
pub struct TreeOptimizer;

impl TreeOptimizer {
    pub fn optimize(root: Node) -> Node {
        Self::visit_node(root).unwrap_or(Node::TextRun(vec![]))
    }

    fn visit_node(node: Node) -> Option<Node> {
        match node {
            Node::TextRun(ref children) => {
                if children.len() == 1 {
                    return Self::visit_node(children[0].clone());
                }

                Self::visit_child_collection(
                    children,
                    Node::TextRun
                )
            },
            Node::Paragraph(ref children) => Self::visit_child_collection(
                children,
                Node::Paragraph
            ),
            Node::UnorderedList(ref children) => {
                Self::visit_child_collection(children, Node::UnorderedList)
            },
            Node::OrderedList(ref children) => {
                Self::visit_child_collection(children, Node::OrderedList)
            },
            Node::Italic(child) => Self::visit_child_node(child, Node::Italic),
            Node::Bold(child) => Self::visit_child_node(child, Node::Bold),
            Node::Highlight(child) => Self::visit_child_node(child, Node::Highlight),
            Node::Strikethrough(child) => Self::visit_child_node(child, Node::Strikethrough),
            Node::Heading(n, child) => Self::visit_child_node(child, |c| Node::Heading(n, c)),
            Node::Blockquote(child) => Self::visit_child_node(child, Node::Blockquote),
            Node::Code(_) | Node::CodeBlock(_) | Node::Link { .. } | Node::Normal(_) => Some(node),
        }
    }

    fn visit_child_node<F: FnOnce(Box<Node>) -> Node>(
        node: Box<Node>,
        constructor: F
    ) -> Option<Node> {
        if let Node::Normal(ref s) = *node {
            if s.chars().all(|c| c.is_whitespace()) || s.is_empty() {
                return None
            }
        }

        if let Some(node) = Self::visit_node(*node) {
            return Some(constructor(
                Box::new(node)
            ))
        }

        None
    }

    fn visit_child_collection<F: FnOnce(Vec<Node>) -> Node>(
        nodes: &Vec<Node>,
        constructor: F,
    ) -> Option<Node> {
        let nodes = nodes.iter()
            .map(|node| {
                // TODO: cleanup here
                let node = Self::visit_node(node.clone());

                if let Some(Node::TextRun(children)) = node {
                    return children;
                }

                if let Some(node) = node {
                    return vec![node];
                }

                vec![]
            })
            .flatten()
            .collect::<Vec<Node>>();

        if !nodes.is_empty() { Some(constructor(nodes)) } else { None }
    }
}
