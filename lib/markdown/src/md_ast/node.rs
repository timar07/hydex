use std::ops::Add;

#[derive(Debug, Clone)]
pub struct Token {
    pub tag: Node,
    pub lexeme: Lexeme,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
#[repr(i8)]
pub enum Node {
    Heading(u8, Box<Node>) = 0,
    Link {
        label: String,
        url: String,
    },
    TextRun(Vec<Node>),
    Bold(Box<Node>),
    Italic(Box<Node>),
    Highlight(Box<Node>),
    Strikethrough(Box<Node>),
    Code(Box<Node>),
    Normal(String),
}

impl Add for Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Node::Normal(a), Node::Normal(b)) => Node::Normal(a + &b),
            (Node::Bold(a), Node::Bold(b)) => Node::Bold(Box::new(*a + *b)),
            (Node::Italic(a), Node::Italic(b)) => Node::Italic(Box::new(*a + *b)),
            (Node::Highlight(a), Node::Highlight(b)) => {
                Node::Highlight(Box::new(*a + *b))
            },
            (a, b) => Node::TextRun(vec![a, b])
        }
    }
}

impl Into<String> for Node {
    fn into(self) -> String {
        match self {
            Node::Normal(s) => s,
            node => panic!("Unable to convert to string node {:?}", node)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lexeme {
    pub start: usize,
    pub end: usize,
}
