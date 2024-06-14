use crate::md_errors::DescribableError;
use super::token::Token;

#[derive(Debug)]
pub struct LexicalError {
    pub tag: LexicalErrorTag
}

#[derive(Debug)]
pub enum LexicalErrorTag {
    UnknownToken,
}

impl DescribableError for LexicalError {
    fn kind(&self) -> String {
        "LexicalError".into()
    }

    fn message(&self) -> String {
        match self.tag {
            LexicalErrorTag::UnknownToken => {
                format!("Unknown token")
            }
        }
    }
}