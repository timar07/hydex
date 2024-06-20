use crate::md_ast::{
    Node,
    NodeCollection
};

use super::normal_text::NormalTextCompiler;

pub trait Compilable {
    fn compile(&self) -> String;
}

impl<'a> Compilable for NodeCollection<'a> {
    fn compile(&self) -> String {
        self.get_nodes()
            .iter()
            .map(|child| child.compile())
            .collect()
    }
}

impl Compilable for Node {
    fn compile(&self) -> String {
        match self {
            Node::TextRun(children) => NodeCollection::new(children).compile(),
            Node::Bold(child) => Enclosured::new(
                "b".into(),
                Self::compile(child)
            ).compile(),
            Node::Highlight(child) => Enclosured::new(
                "mark".into(),
                Self::compile(child)
            ).compile(),
            Node::Italic(child) => Enclosured::new(
                "i".into(),
                Self::compile(child)
            ).compile(),
            Node::Code(child) => Enclosured::new(
                "code".into(),
                Self::compile(child)
            ).compile(),
            Node::Strikethrough(child) => Enclosured::new(
                "s".into(),
                Self::compile(child)
            ).compile(),
            Node::Normal(text) => NormalTextCompiler::new(text.clone()).compile(),
            Node::Link { label: _, url: _ } => todo!(),
            Node::Heading(n, child) => Enclosured::new(
                format!("h{n}"),
                Self::compile(child)
            ).compile()
        }
    }
}

pub struct Enclosured {
    tag: String,
    content: String
}

impl Enclosured {
    pub fn new(tag: String, content: String) -> Self {
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

