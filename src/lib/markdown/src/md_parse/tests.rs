#[cfg(test)]
mod tests {
    use crate::md_parse::Node;
    use create::md_parse::Parser;

    #[test]
    fn emphasis_italic() {
        assert_eq!(
            Parser::parse("Italicized text is the *cat's meow*."),
            Node::TextRun(vec![
                Node::Normal("Italicized text is the ".into()),
                Node::Italic(Box::new(
                    Node::Normal("cat's meow".into())
                )),
                Node::Normal(".".into())
            ])
        );
    }
}