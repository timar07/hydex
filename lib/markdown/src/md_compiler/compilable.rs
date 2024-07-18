use crate::md_ast::Node;
use crate::md_ast::NodeCollection;

use super::tag::HTMLElement;
use super::tag::HTMLAttribute;
use super::tag::HTMLElementLevel::*;
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
    ($tag:expr, $child:expr, $level:expr) => {
        HTMLElement {
            tag: $tag.into(),
            content: Some(Self::compile($child)),
            attrs: None,
            level: $level
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
                    .map(|li| compile_enclosured!($item_tag, li, Block))
                    .collect()
            ),
            attrs: None,
            level: Block
        }.compile()
    };
}

impl Compilable for Node {
    fn compile(&self) -> String {
        match self {
            Node::TextRun(children) => NodeCollection::new(children).compile(),
            Node::Bold(child) => compile_enclosured!("b", child, Inline),
            Node::Highlight(child) => compile_enclosured!("mark", child, Inline),
            Node::Italic(child) => compile_enclosured!("em", child, Inline),
            Node::Code(child) => compile_enclosured!("code", child, Inline),
            Node::Strikethrough(child) => compile_enclosured!("s", child, Inline),
            Node::Link { label, url } => HTMLElement {
                tag: "a".to_string(),
                attrs: Some(&vec![
                    HTMLAttribute::Value("href".to_string(), url.clone())
                ]),
                content: Some(label.clone()),
                level: Inline
            }.compile(),
            Node::Heading(n, child) => compile_enclosured!(format!("h{n}"), child, Block),
            Node::Blockquote(child) => compile_enclosured!("blockquote", child, Block),
            Node::Normal(text) => NormalTextCompiler::new(text.clone()).compile(),
            Node::Paragraph(children) => HTMLElement {
                tag: "p".into(),
                content: Some(NodeCollection::new(children).compile()),
                attrs: None,
                level: Block
            }.compile(),
            Node::UnorderedList(children) => compile_item_collection!("ul", "li", children),
            Node::OrderedList(children) => compile_item_collection!("ol", "li", children),
            Node::CodeBlock(child) => HTMLElement {
                tag: "pre".to_string(),
                attrs: None,
                content: Some(HTMLElement {
                    tag: "code".to_string(),
                    attrs: None,
                    content: Some(child.to_owned()),
                    level: Inline
                }.compile()),
                level: Block
            }.compile(),
            Node::Linebreak => format!(
                "{}\n",
                HTMLElement {
                    tag: "br".to_string(),
                    attrs: None,
                    content: None,
                    level: Inline
                }.compile()
            )
        }
    }
}
