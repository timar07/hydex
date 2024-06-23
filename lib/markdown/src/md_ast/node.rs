/// Represents single node in the parse tree
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Node {
    Heading(u8, Box<Node>),
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

impl Into<String> for Node {
    fn into(self) -> String {
        match self {
            Node::Normal(s) => s,
            node => {
                dbg!("Unable to convert node into string: {:?}", &node);
                format!("{:?}", node)
            }
        }
    }
}
