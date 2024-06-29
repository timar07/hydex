use crate::md_ast::Node;
use crate::md_parse::block::BlockParser;

impl BlockParser<'_, '_> {
    // TODO: Language specification
    /// ```bnf
    /// codeblock = ( "~" )* STRING ( "~" )*
    /// ```
    pub fn parse_fenced_codeblock(&mut self) -> Node {
        self.src.consume_line(); // TODO: parse lang here
        self.src.consume(); // \n
        let code = self.src.consume_until("\n~~~");
        self.src.consume(); // \n
        self.src.consume_while("~");
        Node::CodeBlock(code[0..code.len()].to_string())
    }
}

