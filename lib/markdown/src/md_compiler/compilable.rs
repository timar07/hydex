use crate::md_ast::Node;
use crate::md_ast::NodeCollection;

use super::tag::HTMLAttribute;
use super::tag::HTMLElement;
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
        HTMLElement {
            tag: $tag.into(),
            content: Some(Self::compile($child)),
            attrs: None
        }.compile()
    };
}

macro_rules! compile_item_collection {
    ($tag:expr, $item_tag:expr, $children:expr) => {
        HTMLElement {
            tag: $tag.into(),
            content: Some(
                $children
                    .iter()
                    .map(|li| compile_enclosured!($item_tag, li))
                    .collect()
            ),
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
            Node::Link { label, url } => HTMLElement {
                tag: "a".to_string(),
                attrs: Some(&vec![
                    HTMLAttribute::Value("href".to_string(), url.clone())
                ]),
                content: Some(label.clone()),
            }.compile(),
            Node::Heading(n, child) => compile_enclosured!(format!("h{n}"), child),
            Node::Blockquote(child) => compile_enclosured!("blockquote", child),
            Node::Normal(text) => NormalTextCompiler::new(text.clone()).compile(),
            Node::Paragraph(children) => HTMLElement {
                tag: "p".into(),
                content: Some(NodeCollection::new(children).compile()),
                attrs: None
            }.compile(),
            Node::UnorderedList(children) => compile_item_collection!("ul", "li", children),
            Node::OrderedList(children) => compile_item_collection!("ol", "li", children),
            Node::CodeBlock(child) => HTMLElement {
                tag: "pre".to_string(),
                attrs: None,
                content: Some(HTMLElement {
                    tag: "code".to_string(),
                    attrs: None,
                    content: Some(child.to_owned())
                }.compile())
            }.compile()
        }
    }
}
