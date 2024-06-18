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
        // TODO: Escape HTML characters
        self.content.clone()
    }
}
