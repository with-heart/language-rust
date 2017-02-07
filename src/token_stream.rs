use input_stream::InputStream;
use regex::Regex;

const KEYWORDS: &'static str = " if then else lambda true false ";
lazy_static! {
    static ref IS_ID_START: Regex = Regex::new(r"[a-z_]/i").unwrap();
}

#[derive(Clone)]
pub enum Token {
    Num(i32),
    Punc(String),
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
        let number = self.read_while(|c| c.is_digit(10));
        Some(Token::Num(number.parse::<i32>().unwrap()))
    }

    fn read_ident(&mut self) -> Option<Token> {
        let id = self.read_while(|c| "?!-<>=0123456789".contains(c));
        let mut token: Token;
        if KEYWORDS.contains(id) {
            token = Token::Kw(id)
        } else {
            token = Token::Var(id)
        }
        Some(token);
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
        self.read_while(|c| " \t\n".contains(c));

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
            Some(c) if c.is_digit(10) => {
                return self.read_number();
            }

            // identifier
            Some(c) if IS_ID_START.is_match(&c.to_string()) => return self.read_ident(),

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
}
