use crate::token::{Token, TokenType};
use std::fs::File;
use std::io::Read;

pub trait CharSource: Iterator<Item=char> {
    fn get_source_name(&self) -> &str;
    fn get_pos(&mut self) -> (usize, usize);
    fn peek(&mut self) -> Option<&char>;
}

pub struct Tokenizer<T: CharSource> {
    t: T,
}

impl Tokenizer<CharStream> {
    pub fn new(filename: String) -> Self {
        Tokenizer {
            t: CharStream::new(filename)
        }

    }
    pub fn new_from_string(contents: String) -> Self {
        Tokenizer {
            t: CharStream::new_from_string(contents)
        }

    }
}
impl<T: CharSource> Tokenizer<T> {

    fn read_ident(&mut self, mut n: String) -> Token {
        let start_pos = self.t.get_pos();
        while let Some(s) = self.t.peek() {
            match s {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_'  => n.push(self.t.next().unwrap()),
                _ => break,
            }
        }
        self.string_to_ident(n, start_pos)
    }

    fn read_number(&mut self, mut n: String) -> Token {
        while let Some(_n) = self.t.peek() {
            match _n {
                '0'..='9' => n.push(self.t.next().unwrap()),
                _ => break,
            }
        }
        Token::new(self.t.get_pos(), TokenType::NUM(n.parse().unwrap())) 
    }

    fn read_string(&mut self) -> Token {
        let mut ret = String::new();
        while let Some(c) = self.t.next() {
            match c {
                ' ' | '!' | '#'..='~' => ret.push(c),
                '"' => break,
                _ => return Token::new(self.t.get_pos(), TokenType::INVALID)
            }
        }
        Token::new(self.t.get_pos(), TokenType::STRING(ret))
    }

    fn string_to_ident(&mut self, s: String, pos: (usize, usize)) -> Token {
        match s.as_str() {
            "class"       => Token::new(pos, TokenType::CLASS),
            "constructor" => Token::new(pos, TokenType::CONSTRUCTOR),
            "method"      => Token::new(pos, TokenType::METHOD),
            "function"    => Token::new(pos, TokenType::FUNCTION),
            "int"         => Token::new(pos, TokenType::INT),
            "boolean"     => Token::new(pos, TokenType::BOOL),
            "char"        => Token::new(pos, TokenType::CHAR),
            "void"        => Token::new(pos, TokenType::VOID),
            "var"         => Token::new(pos, TokenType::VAR),
            "static"      => Token::new(pos, TokenType::STATIC),
            "field"       => Token::new(pos, TokenType::FIELD),
            "let"         => Token::new(pos, TokenType::LET),
            "do"          => Token::new(pos, TokenType::DO),
            "if"          => Token::new(pos, TokenType::IF),
            "else"        => Token::new(pos, TokenType::ELSE),
            "while"       => Token::new(pos, TokenType::WHILE),
            "return"      => Token::new(pos, TokenType::RETURN),
            "true"        => Token::new(pos, TokenType::TRUE),
            "false"       => Token::new(pos, TokenType::FALSE),
            "null"        => Token::new(pos, TokenType::NULL),
            "this"        => Token::new(pos, TokenType::THIS),
            _             => Token::new(pos, TokenType::IDENT(s)),
        }
    }

    fn read_comment_literal(&mut self, oneline: bool) -> Option<String> {
        let mut ret = String::new();
        while let Some(ch) = self.t.next() {
            match oneline {
                true => match ch {
                    '\n' => return Some(ret),
                    _ => ret.push(ch),
                }
                false => match ch {
                    '*' => match self.t.peek() {
                        Some('/') => { self.t.next(); return Some(ret); },
                        Some(_) => { ret.push(ch); },
                        None => return None,
                    }
                    _ => ret.push(ch),
                }

            }
        }
    None
    }

    fn read_comment(&mut self) -> Token {
        let pos = self.t.get_pos();
        match self.t.next() {
            Some('/') => match self.read_comment_literal(true) {
                None => Token::new(pos, TokenType::INVALID),
                Some(s) => Token::new(pos, TokenType::ICOMMENT(s)),
            }
            Some('*') => match self.t.peek() {
                Some('*') => { self.t.next(); match self.read_comment_literal(false) {
                    Some(s) => Token::new(pos, TokenType::APICOMMENT(s)),
                    None => Token::new(pos, TokenType::INVALID),
                }},
                Some(_) => match self.read_comment_literal(false) {
                    Some(s) => Token::new(pos, TokenType::COMMENT(s)),
                    None => Token::new(pos, TokenType::INVALID),
                }
                None => Token::new(pos, TokenType::INVALID)

            }
            Some(_) => Token::new(pos, TokenType::INVALID),
            None => Token::new(pos, TokenType::INVALID),
        }
    }
}

impl<T: CharSource + Iterator<Item=char>> Iterator for Tokenizer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut it = (&mut self.t).skip_while(|b| match b {
            ' ' | '\t' | '\n' | '\r' => true,
            _ => false,
        });
        match it.next() {
            None      => None,
            Some(ch) => match ch {
                '(' => Some(Token::new( self.t.get_pos(), TokenType::LPAREN)),
                ')' => Some(Token::new( self.t.get_pos(), TokenType::RPAREN)),
                '{' => Some(Token::new( self.t.get_pos(), TokenType::LCPAREN)),
                '}' => Some(Token::new( self.t.get_pos(), TokenType::RCPAREN)),
                '[' => Some(Token::new( self.t.get_pos(), TokenType::LBRACKET)),
                ']' => Some(Token::new( self.t.get_pos(), TokenType::RBRACKET)),
                ',' => Some(Token::new( self.t.get_pos(), TokenType::COMMA)),
                ';' => Some(Token::new( self.t.get_pos(), TokenType::SEMICOLON)),
                '=' => Some(Token::new( self.t.get_pos(), TokenType::EQ)),
                '.' => Some(Token::new( self.t.get_pos(), TokenType::DOT)),
                '+' => Some(Token::new( self.t.get_pos(), TokenType::PLUS)),
                '-' => Some(Token::new( self.t.get_pos(), TokenType::MINUS)),
                '*' => Some(Token::new( self.t.get_pos(), TokenType::MUL)),
                '&' => Some(Token::new( self.t.get_pos(), TokenType::AMP)),
                '|' => Some(Token::new( self.t.get_pos(), TokenType::PIPE)),
                '~' => Some(Token::new( self.t.get_pos(), TokenType::TILDA)),
                '<' => Some(Token::new( self.t.get_pos(), TokenType::LT)),
                '>' => Some(Token::new( self.t.get_pos(), TokenType::GT)),
                '/' => match self.t.peek() {
                    Some('*') | Some('/') => Some(self.read_comment()),
                    _ => Some(Token::new( self.t.get_pos(), TokenType::DIV))
                }
                '"' => Some(self.read_string()),
                'a'..='z' | 'A'..='Z' | '_' => Some(self.read_ident(String::from(ch))),
                '0'..='9' => Some(self.read_number(String::from(ch))),
                _ => Some(Token::new(self.t.get_pos(), TokenType::INVALID))
                }
            }
        }
    }

pub struct CharStream {
    input: std::iter::Peekable<std::vec::IntoIter<char>>,
    pos: (usize, usize),
    filename: String,
}

impl<'a> CharStream {
    pub fn new(filename: String) -> CharStream {
        let mut file = File::open(&filename).unwrap();
        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();
        CharStream {
            input: input.chars().collect::<Vec<_>>().into_iter().peekable(),
            pos: (1, 0),
            filename: filename,
        }
    }

    pub fn new_from_string(contents: String) -> CharStream {
        CharStream {
            input: contents.chars().collect::<Vec<_>>().into_iter().peekable(),
            pos: (1, 0),
            filename: String::from("_from_string"),
        }

    }
}

impl CharSource for CharStream {
    fn peek(&mut self) -> Option<&char>{
        return self.input.peek();
    }
    fn get_pos(&mut self) -> (usize, usize) {
        return self.pos;
    }
    fn get_source_name(&self) -> &str{
        return &self.filename;
    }
}

impl Iterator for CharStream {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.input.next();
        match ret {
            None => None,
            Some('\n') => {
                self.pos = (self.pos.0 + 1, 0);
                //println!("{:?}, {:?}", self.pos, ret);
                ret
            }
            _ => {
                self.pos.1 = self.pos.1 + 1;
                //println!("{:?}, {:?}", self.pos, ret);
                ret
            }
        }
    }
}

