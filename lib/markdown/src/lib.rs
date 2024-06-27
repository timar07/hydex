pub mod ffi;
mod md_ast;
mod md_errors;
mod md_lex;
mod md_parse;
mod md_compiler;

use md_ast::Node;
use md_parse::Parser;
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
