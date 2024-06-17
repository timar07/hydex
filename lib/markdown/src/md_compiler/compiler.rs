use crate::md_ast::Node;
use super::compilable::Compilable;

pub struct Compiler;

impl Compiler {
    pub fn compile(node: &Node) -> String {
        node.compile()
    }
}
