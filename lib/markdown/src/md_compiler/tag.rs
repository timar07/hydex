use core::fmt;

use super::compilable::Compilable;

/// Represents single HTML element with following grammar:
/// ```bnf
/// tag = single | enclosed;
/// single = "<" OTAG ">" content "<" CTAG ">";
/// enclosed = "<" OTAG "\>";
/// OTAG = tag WHITESPACE attrs;
/// CTAG = tag;
/// ```
#[derive(Debug)]
pub struct HTMLElement<'a> {
    /// Tag name.
    ///
    /// Warning: it **does not** require tag name to exist.
    pub tag: String,

    /// Element attributes with following grammar:
    /// ```bnf
    /// attrs = ( attr ( WHITESPACE attr )* )?;
    /// ```
    pub attrs: Option<&'a Vec<HTMLAttribute>>,

    /// Element content
    pub content: Option<String>,

    /// HTML Element Level
    pub level: HTMLElementLevel
}

impl<'a> Compilable for HTMLElement<'a> {
    fn compile(&self) -> String {
        let attrs_str = match self.attrs {
            Some(attrs) => format!(" {}", (*attrs).compile()),
            _ => "".to_string()
        };

        let element = if let Some(content) = &self.content {
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
        };

        format!(
            "{element}{}",
            if self.level == HTMLElementLevel::Block {
                "\n"
            } else {
                ""
            }
        )
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

/// Represents a HTML Attribute with following grammar:
/// ```bnf
/// attr = boolean | value;
/// boolean = ;
/// value = ATTR_NAME "=" STRING;
/// ATTR_NAME = ( LOWERCASE_LETTER | "-" )*;
/// ```
#[derive(Clone, Debug)]
pub enum HTMLAttribute {
    /// Represents attribute that has specific value.
    ///
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

#[derive(Debug, Clone, PartialEq)]
pub enum HTMLElementLevel {
    Block,
    Inline
}
