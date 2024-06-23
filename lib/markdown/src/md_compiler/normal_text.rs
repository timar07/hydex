use super::compilable::Compilable;

pub struct NormalTextCompiler {
    content: String
}

impl NormalTextCompiler {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl Compilable for NormalTextCompiler {
    fn compile(&self) -> String {
        html_escape_string(self.content.clone())
    }
}

fn html_escape_string(s: String) -> String {
    let mut dest = String::new();

    s.chars().for_each(|c| {
        let raw_char = c.to_string();
        let esc_char = match c {
            '&' => "&amp;",
            '<' => "&lt;",
            '>' => "&gt;",
            '"' => "&quot",
            '\'' => "&#39",
            _ => &raw_char
        };

        dest.push_str(esc_char);
    });

    dest
}
