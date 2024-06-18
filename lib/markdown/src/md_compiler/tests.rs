#[cfg(test)]
mod tests {
    use crate::md_compiler::Compiler;
    use crate::md_ast::Node;
    use crate::md_parse::Parser;

    #[test]
    fn block_heading() {
        assert_eq!(
            Parser::from_string("# Heading").parse(),
            Node::Heading(1, Box::new(Node::Normal(
                "Heading".into()
            )))
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("# Heading").parse()
            ),
            "<h1>Heading</h1>"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("#Heading").parse()
            ),
            "<h1>Heading</h1>"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("# Heading with *emphasis*").parse()
            ),
            "<h1>Heading with <i>emphasis</i></h1>"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("## Heading h2").parse()
            ),
            "<h2>Heading h2</h2>"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("## # Heading").parse()
            ),
            "<h2># Heading</h2>"
        );
    }

    #[test]
    fn span_link() {
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("[This link](http://example.net/) has no title attribute.").parse()
            ),
            "<a href=\"http://example.net\">This link</a> has no title attribute."
        );
    }

    #[test]
    fn emphasis_mixed() {
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("This text is ***really important***.").parse()
            ),
            "This text is <b><i>really important</i></b>."
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("This text is ___really important___.").parse()
            ),
            "This text is <b><i>really important</i></b>."
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("This text is __*really important*__.").parse()
            ),
            "This text is <b><i>really important</i></b>."
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("This text is **_really important_**.").parse()
            ),
            "This text is <b><i>really important</i></b>."
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("This is really***very***important text.").parse()
            ),
            "This is really<b><i>very</i></b>important text."
        );
    }

    #[test]
    fn emphasis_bold() {
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("I just love **bold text**.").parse()
            ),
            "I just love <b>bold text</b>."
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("I just love __bold text__.").parse()
            ),
            "I just love <b>bold text</b>."
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("Love**is**bold").parse()
            ),
            "Love<b>is</b>bold"
        );
    }

    #[test]
    fn emphasis_italic() {
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("Italicized text is the *cat's meow*.").parse()
            ),
            "Italicized text is the <i>cat's meow</i>."
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("Italicized text is the _cat's meow_.").parse()
            ),
            "Italicized text is the <i>cat's meow</i>."
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("A*cat*meow").parse()
            ),
            "A<i>cat</i>meow"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("*italic*").parse()
            ),
            "<i>italic</i>"
        );
    }

    #[test]
    fn emphasis_code() {
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("At the command prompt, type `nano`.").parse()
            ),
            "At the command prompt, type <code>nano</code>."
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("Code: `nano *is* **great**`.").parse()
            ),
            "Code: <code>nano *is* **great**</code>."
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("``There is a literal backtick (`) here.``").parse()
            ),
            "<code>There is a literal backtick (`) here.</code>"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("A single backtick in a code span: `` ` ``").parse()
            ),
            "A single backtick in a code span: <code> ` </code>"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("A backtick-delimited string in a code span: `` `foo` ``").parse()
            ),
            "A backtick-delimited string in a code span: <code> `foo` </code>"
        );
    }

    #[test]
    fn emphasis_strikethrough() {
        assert_eq!(
            Compiler::compile(&Parser::from_string("~~The world is flat.~~").parse()),
            "<s>The world is flat.</s>"
        );
    }
}