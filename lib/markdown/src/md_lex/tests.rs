#[cfg(test)]
mod tests {
    use crate::md_lex::Cursor;

    #[test]
    fn utf8_slice() {
        let cursor = Cursor::from_string("aὐv");
        assert_eq!(
            &cursor[1..2],
            "ὐ"
        );
    }
}