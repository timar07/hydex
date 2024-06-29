#[cfg(test)]
mod tests {
    use crate::md_lex::Cursor;

    #[test]
    fn consume_while() {
        let mut cursor = Cursor::from_string("~~~~~~~");
        cursor.consume_while("~");
        assert_eq!(
            cursor.current(),
            None
        );
    }

    #[test]
    fn skip_n() {
        let mut cursor = Cursor::from_string("asdf");
        cursor.skip_n(100);
        assert_eq!(
            cursor.current(),
            None
        );
    }

    #[test]
    fn match_curr() {
        let mut cursor = Cursor::from_string("a");
        assert!(cursor.match_curr("a"));
    }

    #[test]
    fn utf8_slice() {
        let mut cursor = Cursor::from_string("aὐv");
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