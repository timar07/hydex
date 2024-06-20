use super::Node;

pub struct NodeCollection<'a> {
    nodes: &'a Vec<Node>
}

impl<'a> NodeCollection<'a> {
    pub fn new(nodes: &'a Vec<Node>) -> Self {
        Self {
            nodes
        }
    }

    pub fn get_nodes(&self) -> &'a Vec<Node> {
        &self.nodes
    }
}
