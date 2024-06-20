use crate::md_ast::Node;
use crate::md_ast::NodeCollection;

use super::tag::HTMLAttribute;
use super::tag::HTMLTag;
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
        HTMLTag {
            tag: $tag.into(),
            content: Some(Self::compile($child)),
            attrs: None
        }.compile()
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
            Node::Link { label, url } => HTMLTag {
                tag: "a".to_string(),
                attrs: Some(&vec![
                    HTMLAttribute::Value("href".to_string(), url.clone())
                ]),
                content: Some(label.clone()),
            }.compile(),
            Node::Heading(n, child) => compile_enclosured!(format!("h{n}"), child),
            Node::Normal(text) => NormalTextCompiler::new(text.clone()).compile(),
        }
    }
}
