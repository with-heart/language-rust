use input_stream::InputStream;
use regex::Regex;

#[derive(Clone)]
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
        let mut token: Token;
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

        match self.iter.peek() {
            // comment
            Some('#') => {
                self.skip_comment();
                return self.read_next();
            }

            // string
            Some('"') => {
                return self.read_string();
            }

            // digit
            Some(c) if TokenStream::is_digit(c) => {
                return self.read_number();
            }

            // identifier
            Some(c) if TokenStream::is_id_start(c) => return self.read_ident(),

            Some(c) if TokenStream::is_punc(c) => return Some(
                Token::Punc(self.iter.next().unwrap())),

            _ => {}
        }
        None
    }

    pub fn next(&mut self) -> Option<Token> {
        let cur = self.cur.clone();
        self.cur = self.read_next();
        if cur.is_none() { self.cur.clone() } else { cur }
    }

    pub fn peek(&self) -> Option<Token> {
        self.cur.clone()
    }

    pub fn croak(&self, msg: String) {
        self.iter.croak(msg);
    }

    fn is_whitespace(c: char) -> bool {
        " \t\n".contains(c)
    }

    fn is_keyword(s: String) -> bool {
        let KEYWORDS: Vec<String> = vec!["if".to_string(), "then".into(),
            "else".into(), "lambda".into(), "true".into(), "false".into()];
        KEYWORDS.iter().any(|x| x == s.trim())
    }

    fn is_digit(c: char) -> bool {
        let sliced: &str = &c.to_string();
        Regex::new(r"[0-9]/i").unwrap().is_match(sliced)
    }

    fn is_id_start(c: char) -> bool {
        let sliced: &str = &c.to_string();
        Regex::new(r"[a-z_]/i").unwrap().is_match(sliced)
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
