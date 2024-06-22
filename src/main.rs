use markdown::Markdown;

fn main() {
    dbg!(
        Markdown::parse(
            "This is just a\n\nsimple paragraph".to_string()
        )
    );
}
