mod md_compiler;
mod md_parse;
mod md_errors;

use md_parse::Parser;
use md_compiler::Compiler;

pub struct Markdown;

impl Markdown {
    pub fn compile(s: String) -> String {
        let mut parser = Parser::from_string(&s);
        Compiler::compile(&parser.parse())
    }
}
