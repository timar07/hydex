use super::compilable::Compilable;

pub struct Enclosured {
    tag: String,
    content: String
}

impl Enclosured {
    pub fn new(tag: String, content: String) -> Self {
        Self {
            tag,
            content
        }
    }
}

impl Compilable for Enclosured {
    fn compile(&self) -> String {
        format!("<{}>{}</{}>", self.tag, self.content, self.tag)
    }
}