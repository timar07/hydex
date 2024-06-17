#[cfg(test)]
mod tests {
    use crate::md_parse::Node;
    use crate::md_parse::Parser;

    #[test]
    fn span_link() {
        assert_eq!(
            Parser::from_string("[This link](http://example.net/) has no title attribute.").parse(),
            Node::TextRun(vec![
                Node::Link {
                    label: "This link".into(),
                    url: "http://example.net/".into()
                },
                Node::Normal(" has no title attribute.".into())
            ])
        )
    }

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
            Parser::from_string("This text is ___really important___.").parse(),
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
            Parser::from_string("This text is __*really important*__.").parse(),
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

        assert_eq!(
            Parser::from_string("This is really***very***important text.").parse(),
            Node::TextRun(vec![
                Node::Normal("This is really".into()),
                Node::Bold(Box::new(
                    Node::Italic(Box::new(
                        Node::Normal("very".into())
                    ))
                )),
                Node::Normal("important text.".into())
            ])
        );
    }

    #[test]
    fn emphasis_bold() {
        assert_eq!(
            Parser::from_string("I just love **bold text**.").parse(),
            Node::TextRun(vec![
                Node::Normal("I just love ".into()),
                Node::Bold(Box::new(
                    Node::Normal("bold text".into())
                )),
                Node::Normal(".".into())
            ])
        );

        assert_eq!(
            Parser::from_string("I just love __bold text__.").parse(),
            Node::TextRun(vec![
                Node::Normal("I just love ".into()),
                Node::Bold(Box::new(
                    Node::Normal("bold text".into())
                )),
                Node::Normal(".".into())
            ])
        );

        assert_eq!(
            Parser::from_string("Love**is**bold").parse(),
            Node::TextRun(vec![
                Node::Normal("Love".into()),
                Node::Bold(Box::new(
                    Node::Normal("is".into())
                )),
                Node::Normal("bold".into())
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

    #[test]
    fn emphasis_code() {
        assert_eq!(
            Parser::from_string("At the command prompt, type `nano`.").parse(),
            Node::TextRun(vec![
                Node::Normal("At the command prompt, type ".into()),
                Node::Code(Box::new(
                    Node::Normal("nano".into())
                )),
                Node::Normal(".".into())
            ]),
        );

        assert_eq!(
            Parser::from_string("Code: `nano *is* **great**`.").parse(),
            Node::TextRun(vec![
                Node::Normal("Code: ".into()),
                Node::Code(Box::new(
                    Node::Normal("nano *is* **great**".into())
                )),
                Node::Normal(".".into())
            ]),
        );

        assert_eq!(
            Parser::from_string("``There is a literal backtick (`) here.``").parse(),
            Node::Code(Box::new(
                Node::Normal("There is a literal backtick (`) here.".into())
            )),
        );

        assert_eq!(
            Parser::from_string("A single backtick in a code span: `` ` ``").parse(),
            Node::TextRun(vec![
                Node::Normal("A single backtick in a code span: ".into()),
                Node::Code(Box::new(
                    Node::Normal(" ` ".into())
                ))
            ])
        );

        assert_eq!(
            Parser::from_string("A backtick-delimited string in a code span: `` `foo` ``").parse(),
            Node::TextRun(vec![
                Node::Normal("A backtick-delimited string in a code span: ".into()),
                Node::Code(Box::new(
                    Node::Normal(" `foo` ".into())
                ))
            ])
        );
    }

    #[test]
    fn emphasis_strikethrough() {
        assert_eq!(
            Parser::from_string("~~The world is flat.~~").parse(),
            Node::Strikethrough(Box::new(
                Node::Normal("The world is flat.".into())
            ))
        );

        assert_eq!(
            Parser::from_string("~Not a strikethrough~").parse(),
            Node::Normal("~Not a strikethrough~".into())
        );
    }
}