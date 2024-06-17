use markdown::Markdown;

fn main() {
    dbg!(Markdown::parse("[This link](http://example.net/) has no title attribute.".to_string()));
}
