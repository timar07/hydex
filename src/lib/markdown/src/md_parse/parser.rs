use super::cursor::Cursor;
use super::node::Node;
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
                    if self.src.check_next('*') {
                        self.parse_bold()
                    } else {
                        self.parse_enclosured(
                            "*",
                            |inner| {
                                Node::Italic(inner)
                            }
                        )
                    }
                },
                '_' => {
                    if self.src.check_next('_') {
                        self.parse_enclosured(
                            "__",
                            |inner| {
                                Node::Bold(inner)
                            }
                        )
                    } else {
                        self.parse_enclosured(
                            "_",
                            |inner| {
                                Node::Italic(inner)
                            }
                        )
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
            '!' | '`' | '*' | '_' |
            '{' | '}' | '[' | ']' |
            '<' | '>' | '(' | ')' |
            '#' | '+' | '-' | '|' |
            '\\' => false,
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
        self.src.match_curr(enclosure);

        if !self.lookahead(enclosure) {
            return self.parse_text_run();
        }

        let start = self.src.pos.index;
        self.consume_until(enclosure);

        result_constructor(Box::new(
            Parser::from_string(
                self.src.slice(start, self.src.pos.index-enclosure.len())
            ).tokenize()
        ))
    }

    fn consume_until(&mut self, enclosure: &'static str) {
        while !self.src.is_eof() {
            if self.src.match_curr(&enclosure[..1]) {
                if self.src.check_curr(enclosure) {
                    continue;
                }

                if enclosure.len() > 1 {
                    if self.src.match_curr(&enclosure[1..]) {
                        break;
                    }

                    continue;
                } else {
                    break;
                }
            }

            self.src.consume();
        }
    }

    fn lookahead(&self, matcher: &'static str) -> bool {
        let mut i = self.src.pos.index;

        while i <= self.src.len() - matcher.len() && self.src.char_at(i) != '\n' {
            if self.src.slice(i, i + matcher.len()) == matcher {
                return true;
            }

            i += 1
        }

        false
    }
}


