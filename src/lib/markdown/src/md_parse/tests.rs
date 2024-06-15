#[cfg(test)]
mod tests {
    use crate::md_parse::Node;
    use crate::md_parse::Parser;

    #[test]
    fn emphasis_mixed() {
        assert_eq!(
            Parser::from_string("This text is ***really important***.").parse(),
            Node::TextRun(vec![
                Node::Normal("This text is ".into()),
                Node::Bold(Box::new(
                    Node::Italic(Box::new(
                        Node::Normal("really important".into())
                    ))
                )),
                Node::Normal(".".into())
            ])
        );

        assert_eq!(
            Parser::from_string("This text is **_really important_**.").parse(),
            Node::TextRun(vec![
                Node::Normal("This text is ".into()),
                Node::Bold(Box::new(
                    Node::Italic(Box::new(
                        Node::Normal("really important".into())
                    ))
                )),
                Node::Normal(".".into())
            ])
        );
    }

    #[test]
    fn emphasis_italic() {
        assert_eq!(
            Parser::from_string("Italicized text is the *cat's meow*.").parse(),
            Node::TextRun(vec![
                Node::Normal("Italicized text is the ".into()),
                Node::Italic(Box::new(
                    Node::Normal("cat's meow".into())
                )),
                Node::Normal(".".into())
            ])
        );

        assert_eq!(
            Parser::from_string("Italicized text is the _cat's meow_.").parse(),
            Node::TextRun(vec![
                Node::Normal("Italicized text is the ".into()),
                Node::Italic(Box::new(
                    Node::Normal("cat's meow".into())
                )),
                Node::Normal(".".into())
            ])
        );

        assert_eq!(
            Parser::from_string("A*cat*meow").parse(),
            Node::TextRun(vec![
                Node::Normal("A".into()),
                Node::Italic(Box::new(
                    Node::Normal("cat".into())
                )),
                Node::Normal("meow".into())
            ])
        );

        assert_eq!(
            Parser::from_string("*italic*").parse(),
            Node::Italic(Box::new(
                Node::Normal("italic".into())
            )),
        );
    }
}