use crate::md_ast::Node;
use crate::md_lex::Cursor;
use crate::md_parse::parser::Parsable;

pub struct NormalTextParserUnescaped<'src, 'a> {
    src: &'a mut Cursor<'src>
}

impl<'src, 'a> NormalTextParserUnescaped<'src, 'a> {
    pub fn new(src: &'a mut Cursor<'src>) -> Self {
        Self { src }
    }

    fn is_linebreak(&self) -> bool {
        self.src.check_curr("\\\n") || self.src.check_curr("  \n")
    }

    fn is_normal_char(ch: char) -> bool {
        match ch {
            '`' | '*' | '_' | '{' |
            '}' | '[' | ']' | '<' |
            '>' | '(' | ')' | '|' |
            '~' => false,
            _ => true
        }
    }

    fn is_backslash_escape(&self) -> bool {
        self.src.check_curr("\\") &&
        self.src
            .next()
            .is_some_and(|c| Self::is_escapable_char(c))
    }

    fn is_escapable_char(c: char) -> bool {
        c.is_ascii_punctuation()
    }
}

impl<'src, 'a> Parsable for NormalTextParserUnescaped<'src, 'a> {
    fn parse(&mut self) -> Node {
        let mut s = String::from("");

        while !self.src.is_eof() && Self::is_normal_char(self.src.current().unwrap()) {
            if self.is_linebreak() {
                s = s.trim_end().into();
                break;
            }

            if self.is_backslash_escape() {
                self.src.consume(); // \
            }

            s.push(self.src.consume().unwrap()); // unwrap is safe here
        }

        Node::Normal(s)
    }
}

pub struct NormalTextParserEscaped<'src, 'a> {
    src: &'a mut Cursor<'src>
}

impl<'src, 'a> NormalTextParserEscaped<'src, 'a> {
    pub fn new(src: &'a mut Cursor<'src>) -> Self {
        Self { src }
    }
}

impl<'a, 'b> Parsable for NormalTextParserEscaped<'a, 'b> {
    fn parse(&mut self) -> Node {
        let mut s = String::from("");

        // TODO: Optimization
        while !self.src.is_eof() {
            s.push(self.src.consume().unwrap());
        }

        Node::Normal(s)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        md_ast::Node,
        md_lex::Cursor,
        md_parse::{
            element::NormalTextParserUnescaped,
            parser::Parsable
        }
    };

    #[test]
    fn ascii_punctuation_escape() {
        assert_eq!(
            NormalTextParserUnescaped::new(
                &mut Cursor::from_string(
                    r"\!\#\$\%\&\'\(\)\*\+\,\-\.\/\:\;\<\=\>\?\@\[\\\]\^\_\`\{\|\}\~"
                )
            ).parse(),
            Node::Normal(r"!#$%&'()*+,-./:;<=>?@[\]^_`{|}~".to_string())
        );

        assert_eq!(
            NormalTextParserUnescaped::new(
                &mut Cursor::from_string(
                    r"\→\A\a\ \3\φ\«"
                )
            ).parse(),
            Node::Normal(r"\→\A\a\ \3\φ\«".to_string())
        );
    }
}
