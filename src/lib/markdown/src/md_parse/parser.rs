use super::cursor::Cursor;
use super::emphasis::EmphasisParser;
use super::node::Node;
use super::tree_optimizer::TreeOptimizer;

pub struct Parser<'src> {
    src: Cursor<'src>,
}

impl<'a> Parser<'a> {
    pub fn from_string(s: &'a str) -> Parser<'a> {
        return Self {
            src: Cursor::from_string(s)
        }
    }

    pub fn parse(&mut self) -> Node {
        TreeOptimizer::optimize(
            self.parse_text_run()
        )
    }

    fn parse_text_run(&mut self) -> Node {
        let mut text = vec![];

        while !self.src.is_eof() {
            let token = EmphasisParser::new(&mut self.src).parse();

            text.push(token);
        }

        Node::TextRun(text)
    }
}

pub trait Parsable {
    fn parse(&mut self) -> Node;
}
