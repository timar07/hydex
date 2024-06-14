use super::cursor::Cursor;
use super::token::Node;
use super::tree_optimizer::TreeOptimizer;

pub struct Parser<'source> {
    src: Cursor<'source>,
}

impl<'a> Parser<'a> {
    pub fn from_string(s: &'a str) -> Parser<'a> {
        return Self {
            src: Cursor::from_string(s)
        }
    }

    pub fn parse(&mut self) -> Node {
        TreeOptimizer::optimize(self.tokenize())
    }

    pub fn tokenize(&mut self) -> Node {
        self.parse_text_run()
    }

    fn parse_text_run(&mut self) -> Node {
        let mut text = vec![];

        while !self.src.is_eof() {
            let token = match self.src.current() {
                '*' => {
                    if self.src.match_next('*') {
                        self.parse_bold()
                    } else {
                        self.parse_italic()
                    }
                },
                '=' => self.parse_highlight(),
                '\\' => {
                    self.src.consume();
                    self.parse_normal_text()
                }
                _ => self.parse_normal_text()
            };

            text.push(token);
        }

        Node::TextRun(text)
    }

    fn parse_bold(&mut self) -> Node {
        self.parse_enclosured(
            "**",
            |inner| {
                Node::Bold(inner)
            }
        )
    }

    fn parse_italic(&mut self) -> Node {
        self.parse_enclosured(
            "*",
            |inner| {
                Node::Italic(inner)
            }
        )
    }

    fn parse_highlight(&mut self) -> Node {
        self.parse_enclosured(
            "=",
            |inner| {
                Node::Highlight(inner)
            }
        )
    }

    fn parse_normal_text(&mut self) -> Node {
        let start = self.src.pos.index;

        while !self.src.is_eof() &&
              (Parser::is_normal_char(self.src.current()) ||
              self.src.prev() == '\\') {
            self.src.consume();
        }

        Node::Normal(self.src.slice(start, self.src.pos.index).to_owned())
    }

    fn is_normal_char(ch: char) -> bool {
        match ch {
            '*' | '=' | '_' | '\\' => false,
            _ => true
        }
    }

    fn parse_enclosured<T>(
        &mut self,
        enclosure: &'static str,
        result_constructor: T
    ) -> Node
    where
        T: Fn(Box<Node>) -> Node
    {
        self.src.consume();

        if !self.lookahead(enclosure) {
            return self.parse_text_run();
        }

        let start = self.src.pos.index;

        self.consume_until(enclosure);

        result_constructor(Box::new(
            Parser::from_string(
                self.src.slice(start, self.src.pos.index-1)
            ).tokenize()
        ))
    }

    fn consume_until(&mut self, enclosure: &'static str) {
        while !self.src.is_eof() {
            if self.src.match_curr(enclosure)
            && !self.src.match_next(enclosure.chars().nth(0).unwrap())
            || self.src.match_curr("\n"){
                break;
            }

            self.src.consume();
        }
    }

    fn lookahead(&self, matcher: &'static str) -> bool {
        let mut i = self.src.pos.index;

        while i < self.src.len() - matcher.len() && self.src.char_at(i) != '\n' {
            if self.src.slice(i, i + matcher.len()) == matcher {
                return true;
            }

            i += 1
        }

        false
    }
}


