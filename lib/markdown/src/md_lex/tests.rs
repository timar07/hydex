#[cfg(test)]
mod tests {
    use crate::md_lex::Cursor;

    #[test]
    fn utf8_slice() {
        let mut cursor = Cursor::from_string("aὐv");
        dbg!(cursor.len());
        assert_eq!(
            &cursor[1..2],
            "ὐ"
        );
        assert_eq!(
            cursor.consume(),
            Some('a')
        );
        assert_eq!(
            cursor.consume(),
            Some('ὐ')
        );
        assert_eq!(
            cursor.consume(),
            Some('v')
        );
        assert_eq!(
            cursor.consume(),
            None
        );
    }
}