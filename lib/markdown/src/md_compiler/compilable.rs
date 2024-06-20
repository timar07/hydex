use crate::md_ast::Node;
use crate::md_ast::NodeCollection;

use super::enclosured::Enclosured;
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

macro_rules! compile_enclosured {
    ($tag:expr, $child:expr) => {
        Enclosured::new($tag.into(), Self::compile($child)).compile()
    };
}

impl Compilable for Node {
    fn compile(&self) -> String {
        match self {
            Node::TextRun(children) => NodeCollection::new(children).compile(),
            Node::Bold(child) => compile_enclosured!("b", child),
            Node::Highlight(child) => compile_enclosured!("mark", child),
            Node::Italic(child) => compile_enclosured!("i", child),
            Node::Code(child) => compile_enclosured!("code", child),
            Node::Strikethrough(child) => compile_enclosured!("s", child),
            Node::Link { label: _, url: _ } => todo!(),
            Node::Heading(n, child) => compile_enclosured!(format!("h{n}"), child),
            Node::Normal(text) => NormalTextCompiler::new(text.clone()).compile(),
        }
    }
}
