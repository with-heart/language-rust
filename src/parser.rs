use token_stream::{TokenStream, Token};

const FALSE: Node = Node::Bool{ value: false };

#[derive(Clone, Debug)]
pub enum Node {
    Num { value: i32 },
    Str { value: String },
    Bool { value: bool },
    Var { value: String },
    Lambda { vars: Vec<String>, body: Box<Node> },
    Call { func: Box<Node>, args: Vec<Node> },
    If { cond: Box<Node>, then: Box<Node>, elsethen: Box<Node> },
    Assign { operator: String, left: Box<Node>, Right: Box<Node> },
    Binary { operator: String, left: Box<Node>, right: Box<Node> },
    Prog { prog: Vec<Node> },
    Empty
}

pub struct Parser<'b> {
    input: TokenStream<'b>
}

impl<'b> Parser<'b> {
    pub fn new(input: TokenStream) -> Node {
        Node::Empty
    }

    fn is_punc(&mut self, c: char) -> bool {
        let tok = self.input.peek();
        match tok {
            Some(Token::Punc(_)) => true,
            _ => false,
        }
    }
}

