use core::fmt;

use super::compilable::Compilable;

#[derive(Debug)]
pub struct HTMLTag<'a> {
    pub tag: String,
    pub attrs: Option<&'a Vec<HTMLAttribute>>,
    pub content: Option<String>
}

impl<'a> Compilable for HTMLTag<'a> {
    fn compile(&self) -> String {
        let attrs_str = match self.attrs {
            Some(attrs) => format!(" {}", (*attrs).compile()),
            _ => "".to_string()
        };

        if let Some(content) = &self.content {
            format!(
                "<{}{attrs_str}>{content}</{}>",
                self.tag,
                self.tag
            )
        } else {
            format!(
                "<{}{attrs_str} />",
                self.tag
            )
        }
    }
}

impl Compilable for Vec<HTMLAttribute> {
    fn compile(&self) -> String {
        self.iter()
            .map(|attr| {
                attr.compile()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

/// Represents a HTML Attribute
#[derive(Clone, Debug)]
pub enum HTMLAttribute {
    /// Represents attribute that has specific value.
    /// For example `<a href="...">...</a>`
    Value(String, String),

    /// Boolean attribute has no value.
    /// For example `<input type="checkbox" checked />`
    Boolean(String)
}

impl fmt::Display for HTMLAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.compile())
    }
}

impl Compilable for HTMLAttribute {
    fn compile(&self) -> String {
        match self {
            Self::Boolean(name) => name.clone(),
            Self::Value(name, value) => format!("{name}=\"{value}\"")
        }
    }
}
