use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Token {
    pub tag: Node,
    pub lexeme: Lexeme,
    // pub info: DebugInfo
}

// impl Token {
//     pub fn get_lexeme(&self) -> String {
//         self.info.src[self.lexeme.start..self.lexeme.end].to_string()
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub enum Node {
    Heading(u8),
    TextRun(Vec<Node>),
    Bold(Box<Node>),
    Italic(Box<Node>),
    Highlight(Box<Node>),
    Normal(String),
}

impl Add for Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        match [self, rhs] {
            [Node::Normal(a), Node::Normal(b)] => Node::Normal([a, b].join("")),
            _ => todo!()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lexeme {
    pub start: usize,
    pub end: usize,
}
