use crate::md_ast::Node;

pub trait Compilable {
    fn compile(&self) -> String;
}

pub type NodeCollection = Vec<Node>;

impl Compilable for NodeCollection {
    fn compile(&self) -> String {
        self.iter()
            .map(|child| child.compile())
            .collect()
    }
}

impl Compilable for Node {
    fn compile(&self) -> String {
        match self {
            Node::TextRun(children) => {
                children.compile()
            },
            Node::Bold(child) => Enclosured::new(
                "b",
                Self::compile(child)
            ).compile(),
            Node::Highlight(child) => Enclosured::new(
                "mark",
                Self::compile(child)
            ).compile(),
            Node::Italic(child) => Enclosured::new(
                "i",
                Self::compile(child)
            ).compile(),
            Node::Normal(text) => text.clone(),
            node => todo!("{:?}", node)
        }
    }
}

pub struct Enclosured {
    tag: &'static str,
    content: String
}

impl Enclosured {
    pub fn new(tag: &'static str, content: String) -> Self {
        Self {
            tag,
            content
        }
    }
}

impl Compilable for Enclosured {
    fn compile(&self) -> String {
        format!("<{}>{}</{}>", self.tag, self.content, self.tag)
    }
}

