use markdown::Markdown;

fn main() {
    dbg!(Markdown::compile("hello **world *italics*** it's me".to_string()));
}
