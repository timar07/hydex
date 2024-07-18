#[cfg(test)]
mod tests {
    use crate::md_compiler::Compiler;
    use crate::md_ast::Node;
    use crate::md_parse::Parser;

    #[test]
    fn block_paragraph() {
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("aaa\n\nbbb")
                    .parse()
            ),
            "<p>aaa</p>\n<p>bbb</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("aaa\nbbb\n\nccc\nddd")
                    .parse()
            ),
            "<p>aaa\nbbb</p>\n<p>ccc\nddd</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("aaa\n\n\nbbb")
                    .parse()
            ),
            "<p>aaa</p>\n<p>bbb</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("  aaa\n bbb")
                    .parse()
            ),
            "<p>aaa\nbbb</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string(
                    "aaa\n\
                                 bbb\n\
                                                           ccc"
                )
                    .parse()
            ),
            "<p>aaa\nbbb\nccc</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string(
                    "   aaa\nbbb"
                )
                    .parse()
            ),
            "<p>aaa\nbbb</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string(
                    "aaa    \nbbb     "
                )
                    .parse()
            ),
            "<p>aaa<br />\nbbb</p>\n"
        );
    }

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
            "<h1>Heading</h1>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("#Heading").parse()
            ),
            "<h1>Heading</h1>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("# Heading with *emphasis*").parse()
            ),
            "<h1>Heading with <em>emphasis</em></h1>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("## Heading h2").parse()
            ),
            "<h2>Heading h2</h2>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("## # Heading").parse()
            ),
            "<h2># Heading</h2>\n"
        );
    }

    #[test]
    fn span_link() {
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("[This link](http://example.net/) has no title attribute.").parse()
            ),
            "<p><a href=\"http://example.net/\">This link</a> has no title attribute.</p>\n"
        );
    }

    #[test]
    fn emphasis_mixed() {
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("This text is ***really important***.").parse()
            ),
            "<p>This text is <b><em>really important</em></b>.</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("This text is ___really important___.").parse()
            ),
            "<p>This text is <b><em>really important</em></b>.</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("This text is __*really important*__.").parse()
            ),
            "<p>This text is <b><em>really important</em></b>.</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("This text is **_really important_**.").parse()
            ),
            "<p>This text is <b><em>really important</em></b>.</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("This is really***very***important text.").parse()
            ),
            "<p>This is really<b><em>very</em></b>important text.</p>\n"
        );
    }

    #[test]
    fn emphasis_bold() {
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("I just love **bold text**.").parse()
            ),
            "<p>I just love <b>bold text</b>.</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("I just love __bold text__.").parse()
            ),
            "<p>I just love <b>bold text</b>.</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("Love**is**bold").parse()
            ),
            "<p>Love<b>is</b>bold</p>\n"
        );
    }

    #[test]
    fn emphasis_italic() {
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("Italicized text is the *cat's meow*.").parse()
            ),
            "<p>Italicized text is the <em>cat&#39;s meow</em>.</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("Italicized text is the _cat's meow_.").parse()
            ),
            "<p>Italicized text is the <em>cat&#39;s meow</em>.</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("A*cat*meow").parse()
            ),
            "<p>A<em>cat</em>meow</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("*italic*").parse()
            ),
            "<p><em>italic</em></p>\n"
        );
    }

    #[test]
    fn emphasis_code() {
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("At the command prompt, type `nano`.").parse()
            ),
            "<p>At the command prompt, type <code>nano</code>.</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("Code: `nano *is* **great**`.").parse()
            ),
            "<p>Code: <code>nano *is* **great**</code>.</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("``There is a literal backtick (`) here.``").parse()
            ),
            "<p><code>There is a literal backtick (`) here.</code></p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("A single backtick in a code span: `` ` ``").parse()
            ),
            "<p>A single backtick in a code span: <code> ` </code></p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("A backtick-delimited string in a code span: `` `foo` ``").parse()
            ),
            "<p>A backtick-delimited string in a code span: <code> `foo` </code></p>\n"
        );
    }

    #[test]
    fn emphasis_strikethrough() {
        assert_eq!(
            Compiler::compile(&Parser::from_string("~~The world is flat.~~").parse()),
            "<p><s>The world is flat.</s></p>\n"
        );
    }

    #[test]
    fn linebreaks() {
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("foo  \nbaz").parse()
            ),
            "<p>foo<br />\nbaz</p>\n"
        );
        assert_eq!(
            Compiler::compile(
                &Parser::from_string("foo\\\nbaz").parse()
            ),
            "<p>foo<br />\nbaz</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("foo       \nbaz").parse()
            ),
            "<p>foo<br />\nbaz</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("foo\\\n     bar").parse()
            ),
            "<p>foo<br />\nbar</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("foo  \n     bar").parse()
            ),
            "<p>foo<br />\nbar</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("*foo  \nbar*").parse()
            ),
            "<p><em>foo<br />\nbar</em></p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("*foo\\\nbar*").parse()
            ),
            "<p><em>foo<br />\nbar</em></p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("foo\\").parse()
            ),
            "<p>foo\\</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("foo  ").parse()
            ),
            "<p>foo</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("### foo\\").parse()
            ),
            "<h3>foo\\</h3>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("### foo  \n").parse()
            ),
            "<h3>foo</h3>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("### foo  \ntest").parse()
            ),
            "<h3>foo</h3>\n<p>test</p>\n"
        );


        assert_eq!(
            Compiler::compile(
                &Parser::from_string("# foo ~bar  \n~\nasdfasdf").parse()
            ),
            "<h1>foo ~bar</h1>\n<p>~\nasdfasdf</p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("`code  \nspan`").parse()
            ),
            "<p><code>code   span</code></p>\n"
        );

        assert_eq!(
            Compiler::compile(
                &Parser::from_string("`code\\\nspan`").parse()
            ),
            "<p><code>code\\ span</code></p>\n"
        );
    }

    #[test]
    fn normal_escapes() {
        assert_eq!(
            Compiler::compile(&Parser::from_string(r"\*not emphasized*").parse()),
            "<p>*not emphasized*</p>\n"
        );

        assert_eq!(
            Compiler::compile(&Parser::from_string(r"\`not a code`").parse()),
            "<p>`not a code`</p>\n"
        );

        assert_eq!(
            Compiler::compile(&Parser::from_string(r"1\. not a list").parse()),
            "<p>1. not a list</p>\n"
        );

        assert_eq!(
            Compiler::compile(&Parser::from_string(r"\* not a list").parse()),
            "<p>* not a list</p>\n"
        );

        assert_eq!(
            Compiler::compile(&Parser::from_string(r"\# not a heading").parse()),
            "<p># not a heading</p>\n"
        );

        assert_eq!(
            Compiler::compile(&Parser::from_string(r"\\*emphasis*").parse()),
            "<p>\\<em>emphasis</em></p>\n"
        );
    }
}