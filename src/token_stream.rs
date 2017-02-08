use input_stream::InputStream;
use regex::Regex;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Num(i32),
    Punc(char),
    Str(String),
    Kw(String),
    Var(String),
    Op(String),
    Empty,
}

pub struct TokenStream<'a> {
    cur: Option<Token>,
    iter: InputStream<'a>,
}

impl<'a> TokenStream<'a> {
    pub fn new(input: InputStream) -> TokenStream {
        TokenStream {
            cur: None,
            iter: input,
        }
    }

    fn read_number(&mut self) -> Option<Token> {
        let number = self.read_while(TokenStream::is_digit);
        Some(Token::Num(number.parse::<i32>().unwrap()))
    }

    fn read_ident(&mut self) -> Option<Token> {
        let id = self.read_while(TokenStream::is_id);
        let token: Token;
        if TokenStream::is_keyword(id.clone()) {
            token = Token::Kw(id);
        } else {
            token = Token::Var(id);
        }
        Some(token)
    }

    fn read_escaped(&mut self, end: char) -> String {
        let mut escaped = false;
        let mut string = String::from("");
        self.iter.next();

        while !self.iter.eof() {
            let ch = self.iter.next().unwrap();

            if escaped {
                string.push(ch);
            } else if ch == '\\' {
                escaped = true;
            } else if ch == end {
                break;
            } else {
                string.push(ch);
            }
        }

        string
    }

    fn read_string(&mut self) -> Option<Token> {
        return Some(Token::Str(self.read_escaped('"')));
    }

    fn skip_comment(&mut self) {
        self.read_while(|c| c != '\n');
        self.iter.next();
    }

    fn read_while<F>(&mut self, predicate: F) -> String
        where F: Fn(char) -> bool
    {
        let mut string: String = String::from("");
        while !self.iter.eof() && predicate(self.iter.peek().unwrap()) {
            string.push(self.iter.next().unwrap());
        }
        string
    }

    fn read_next(&mut self) -> Option<Token> {
        // skip whitespace
        self.read_while(TokenStream::is_whitespace);

        if self.iter.eof() {
            return None;
        }

        let result = match self.iter.peek() {
            // comment
            Some('#') => {
                println!("#");
                self.skip_comment();
                self.read_next()
            }

            // string
            Some('"') => self.read_string(),

            // digit
            Some(c) if TokenStream::is_digit(c) => {
                println!("is digit!!!");
                self.read_number()
            }

            // identifier
            Some(c) if TokenStream::is_id_start(c) => self.read_ident(),

            // punc
            Some(c) if TokenStream::is_punc(c) => Some(Token::Punc(self.iter.next().unwrap())),

            // operator
            Some(c) if TokenStream::is_op_char(c) => {
                Some(Token::Op(self.read_while(TokenStream::is_op_char)))
            }

            _ => None,
        };

        if result.is_none() {
            let peek = self.peek().unwrap();
            self.iter
                .croak(format!("Can't handle character: {:?}", peek));
        }
        result
    }

    pub fn next(&mut self) -> Option<Token> {
        let tok = self.cur.clone();
        self.cur = None;
        if tok.is_none() { self.read_next() } else { tok }
    }

    pub fn peek(&mut self) -> Option<Token> {
        if self.cur.is_none() {
            self.cur = self.read_next();
            self.cur.clone()
        } else {
            self.cur.clone()
        }
    }

    pub fn eof(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn croak(&self, msg: String) {
        self.iter.croak(msg);
    }

    fn is_whitespace(c: char) -> bool {
        " \t\n".contains(c)
    }

    fn is_keyword(s: String) -> bool {
        let keywords: Vec<String> = vec!["if".to_string(),
                                         "then".into(),
                                         "else".into(),
                                         "lambda".into(),
                                         "true".into(),
                                         "false".into()];
        keywords.iter().any(|x| x == s.trim())
    }

    fn is_digit(c: char) -> bool {
        let sliced: &str = &c.to_string();
        Regex::new(r"[0-9]").unwrap().is_match(sliced)
    }

    fn is_id_start(c: char) -> bool {
        let sliced: &str = &c.to_string();
        Regex::new(r"[a-zA-Z_]").unwrap().is_match(sliced)
    }

    fn is_id(c: char) -> bool {
        TokenStream::is_id_start(c) || "?!-<>=0123456789".contains(c)
    }

    fn is_op_char(c: char) -> bool {
        "+-*/%=&|<>!".contains(c)
    }

    fn is_punc(c: char) -> bool {
        ",;(){}[]".contains(c)
    }
}

#[cfg(test)]
mod tests {
    use token_stream::{TokenStream, Token};
    use input_stream::InputStream;

    #[test]
    fn test_next() {
        let string = String::from("\"test\"\"test2\"");
        let mut ts = stream(&string);
        let token = Token::Str("test".to_string());
        let token2 = Token::Str("test2".to_string());

        assert_eq!(ts.next().unwrap(), token);
        assert_eq!(ts.next().unwrap(), token2);
    }

    #[test]
    fn test_peek() {
        let string = String::from("\"test\"\"test2\"");
        let mut ts = stream(&string);
        let token = Token::Str("test".to_string());
        let token2 = Token::Str("test2".to_string());

        assert_eq!(ts.peek().unwrap(), token);
        assert_eq!(ts.peek().unwrap(), token);
        ts.next();
        assert_eq!(ts.peek().unwrap(), token2);
    }

    #[test]
    fn test_eof() {
        let string = String::from("a");
        let mut ts = stream(&string);

        assert!(!ts.eof());
        ts.next();
        assert!(ts.eof());
    }

    #[test]
    fn test_num() {
        let string = String::from("1");
        let mut ts = stream(&string);
        let token = Token::Num(1);

        assert_eq!(ts.next().unwrap(), token);
    }

    #[test]
    fn test_string() {
        let string = String::from("\"test\"");
        let mut ts = stream(&string);
        let token = Token::Str("test".to_string());

        assert_eq!(ts.next().unwrap(), token);
    }

    #[test]
    fn test_punc() {
        let string = String::from("(");
        let mut ts = stream(&string);
        let token = Token::Punc('(');

        assert_eq!(ts.next().unwrap(), token);
    }

    #[test]
    fn test_kw() {
        let string = String::from("true");
        let mut ts = stream(&string);
        let token = Token::Kw("true".to_string());

        assert_eq!(ts.next().unwrap(), token);
    }

    #[test]
    fn test_var() {
        let string = String::from("alpha");
        let mut ts = stream(&string);
        let token = Token::Var("alpha".to_string());

        assert_eq!(ts.next().unwrap(), token);
    }

    #[test]
    fn test_op() {
        let string = String::from("!=");
        let mut ts = stream(&string);
        let token = Token::Op("!=".to_string());

        assert_eq!(ts.next().unwrap(), token);
    }

    #[test]
    fn test_skip_whitespace() {
        let string = String::from("   1");
        let mut ts = stream(&string);
        let token = Token::Num(1);

        assert_eq!(ts.next().unwrap(), token);
    }

    fn stream<'a>(s: &'a String) -> TokenStream<'a> {
        let is = InputStream::new(&s);
        TokenStream::new(is)
    }
}
