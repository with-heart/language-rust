use std::str;

pub struct InputStream<'a> {
    line: i32,
    col: i32,
    cur: Option<char>,
    iter: str::Chars<'a>,
}

impl<'a> Iterator for InputStream<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let cur = self.cur.clone();
        let next = self.iter.next();

        if next == Some('\n') {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }

        self.cur = next;
        cur
    }
}

impl<'a> InputStream<'a> {
    pub fn new(s: &'a String) -> InputStream<'a> {
        let mut stream = InputStream {
            line: 1,
            col: 1,
            cur: None,
            iter: s.chars(),
        };

        stream.cur = stream.iter.next();
        stream
    }

    pub fn peek(&self) -> Option<char> {
        self.cur
    }

    pub fn eof(&self) -> bool {
        self.peek().is_none()
    }

    pub fn croak(&self, msg: String) {
        panic!("{} ({}:{})", msg, self.line, self.col);
    }
}

#[cfg(test)]
mod tests {

    use input_stream::InputStream;

    #[test]
    fn test_input_stream_next() {
        let string = String::from("test");
        let mut stream = InputStream::new(&string);

        assert_eq!(stream.next(), Some('t'));
        assert_eq!(stream.next(), Some('e'));
        assert_eq!(stream.next(), Some('s'));
        assert_eq!(stream.next(), Some('t'));
        assert_eq!(stream.next(), None);
    }

    #[test]
    fn test_input_stream_peek() {
        let string = String::from("test");
        let mut stream = InputStream::new(&string);

        assert_eq!(stream.peek(), Some('t'));
        assert_eq!(stream.peek(), Some('t'));
        assert_eq!(stream.next(), Some('t'));
        assert_eq!(stream.peek(), Some('e'));
        assert_eq!(stream.peek(), Some('e'));
        assert_eq!(stream.next(), Some('e'));
        assert_eq!(stream.peek(), Some('s'));
        assert_eq!(stream.peek(), Some('s'));
        assert_eq!(stream.next(), Some('s'));
        assert_eq!(stream.peek(), Some('t'));
        assert_eq!(stream.next(), Some('t'));
        assert_eq!(stream.peek(), None);
    }

    #[test]
    fn test_input_stream_eof() {
        let string = String::from("t");
        let mut stream = InputStream::new(&string);

        assert!(!stream.eof());
        stream.next();
        assert!(stream.eof());
    }

    #[test]
    #[should_panic]
    fn test_input_stream_croak_panic() {
        let string = String::from("test");
        let stream = InputStream::new(&string);
        stream.croak("test".to_string());
    }

    #[test]
    fn test_input_stream_col_and_line() {
        let string = String::from("te\nst");
        let mut stream = InputStream::new(&string);

        assert_eq!(stream.col, 1);
        assert_eq!(stream.line, 1);
        stream.next();
        assert_eq!(stream.col, 2);
        assert_eq!(stream.line, 1);
        stream.next();
        assert_eq!(stream.col, 1);
        assert_eq!(stream.line, 2);
        stream.next();
        assert_eq!(stream.col, 2);
        assert_eq!(stream.line, 2);
    }
}
