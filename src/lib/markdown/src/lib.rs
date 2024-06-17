mod md_compiler;
mod md_parse;
mod md_errors;

use md_parse::{Node, Parser};
use md_compiler::Compiler;

pub struct Markdown;

impl Markdown {
    pub fn compile(s: String) -> String {
        let node = Self::parse(s);
        Compiler::compile(&node)
    }

    pub fn parse(s: String) -> Node {
        Parser::from_string(&s).parse()
    }
}
