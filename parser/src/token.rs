#[derive(Debug, PartialEq)]
pub struct Token {
    pos: (usize, usize),
    pub t: TokenType,
}

impl Token {
    pub fn new(pos: (usize, usize), t: TokenType) -> Self {
        Token {pos, t}
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ICOMMENT(String),
    COMMENT(String),
    APICOMMENT(String),
    LPAREN,
    RPAREN,
    LCPAREN,
    RCPAREN,
    LBRACKET,
    RBRACKET,
    COMMA,
    SEMICOLON,
    EQ,
    DOT,
    PLUS,
    MINUS,
    MUL,
    DIV,
    AMP,
    PIPE,
    TILDA,
    LT,
    GT,
    IDENT(String),
    STRING(String),
    NUM(i16),
    CLASS,
    CONSTRUCTOR,
    METHOD,
    FUNCTION,
    INT,
    BOOL,
    CHAR,
    VOID,
    VAR,
    STATIC,
    FIELD,
    LET,
    DO,
    IF,
    ELSE,
    WHILE,
    RETURN,
    TRUE,
    FALSE,
    NULL,
    THIS,
    INVALID,
    EOF,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Tokenizer;
    #[test]
    fn tokenizer_test() {
        let input = String::from("class var     let = { test hi \"string\" \"\",,..[1,22*2+1-4~21] }
            // comment
            // /* not a comment */
            /* a comment */
            /** api comment */
            (+ 2 2          ~(1-2))
            function void main() {}");

        let mut t = Tokenizer::new_from_string(input);
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 1),  t:TokenType::CLASS});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 7),  t:TokenType::VAR});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 15), t:TokenType::LET});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 19), t:TokenType::EQ});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 21), t:TokenType::LCPAREN});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 23), t:TokenType::IDENT(String::from("test"))});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 28), t:TokenType::IDENT(String::from("hi"))});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 38), t:TokenType::STRING(String::from("string"))});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 41), t:TokenType::STRING(String::from(""))});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 42), t:TokenType::COMMA});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 43), t:TokenType::COMMA});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 44), t:TokenType::DOT});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 45), t:TokenType::DOT});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 46), t:TokenType::LBRACKET});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 47), t:TokenType::NUM(1)});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 48), t:TokenType::COMMA});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 50), t:TokenType::NUM(22)});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 51), t:TokenType::MUL});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 52), t:TokenType::NUM(2)});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 53), t:TokenType::PLUS});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 54), t:TokenType::NUM(1)});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 55), t:TokenType::MINUS});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 56), t:TokenType::NUM(4)});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 57), t:TokenType::TILDA});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 59), t:TokenType::NUM(21)});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 60), t:TokenType::RBRACKET});
        assert_eq!(t.next().unwrap(), Token{ pos:(1, 62), t:TokenType::RCPAREN});
        assert_eq!(t.next().unwrap(), Token{ pos:(2, 13),  t:TokenType::ICOMMENT(String::from(" comment"))});
        assert_eq!(t.next().unwrap(), Token{ pos:(3, 13),  t:TokenType::ICOMMENT(String::from(" /* not a comment */"))});
        assert_eq!(t.next().unwrap(), Token{ pos:(4, 13),  t:TokenType::COMMENT(String::from(" a comment "))});
        assert_eq!(t.next().unwrap(), Token{ pos:(5, 13),  t:TokenType::APICOMMENT(String::from(" api comment "))});
        assert_eq!(t.next().unwrap(), Token{ pos:(6, 13),  t:TokenType::LPAREN});
        assert_eq!(t.next().unwrap(), Token{ pos:(6, 14),  t:TokenType::PLUS});
        assert_eq!(t.next().unwrap(), Token{ pos:(6, 16),  t:TokenType::NUM(2)});
        assert_eq!(t.next().unwrap(), Token{ pos:(6, 18),  t:TokenType::NUM(2)});
        assert_eq!(t.next().unwrap(), Token{ pos:(6, 29),  t:TokenType::TILDA});
        assert_eq!(t.next().unwrap(), Token{ pos:(6, 30),  t:TokenType::LPAREN});
        assert_eq!(t.next().unwrap(), Token{ pos:(6, 31),  t:TokenType::NUM(1)});
        assert_eq!(t.next().unwrap(), Token{ pos:(6, 32),  t:TokenType::MINUS});
        assert_eq!(t.next().unwrap(), Token{ pos:(6, 33),  t:TokenType::NUM(2)});
        assert_eq!(t.next().unwrap(), Token{ pos:(6, 34),  t:TokenType::RPAREN});
        assert_eq!(t.next().unwrap(), Token{ pos:(6, 35),  t:TokenType::RPAREN});
        assert_eq!(t.next().unwrap(), Token{ pos:(7, 13),  t:TokenType::FUNCTION});
        assert_eq!(t.next().unwrap(), Token{ pos:(7, 22),  t:TokenType::VOID});
        assert_eq!(t.next().unwrap(), Token{ pos:(7, 27),  t:TokenType::IDENT(String::from("main"))});
        assert_eq!(t.next().unwrap(), Token{ pos:(7, 31),  t:TokenType::LPAREN});
        assert_eq!(t.next().unwrap(), Token{ pos:(7, 32),  t:TokenType::RPAREN});
        assert_eq!(t.next().unwrap(), Token{ pos:(7, 34),  t:TokenType::LCPAREN});
        assert_eq!(t.next().unwrap(), Token{ pos:(7, 35),  t:TokenType::RCPAREN});
        assert_eq!(t.next(), None);
        assert_eq!(t.next(), None);
    }

    #[test]
    fn tokenizer_comment_test() {
        let input = String::from("
class var // this is a comment
class /** multiline api 
      comment */
let /* inline comment */ x
/* wrong coment");

        let mut t = Tokenizer::new_from_string(input);
        assert_eq!(t.next().unwrap(), Token{ pos:(2, 1),  t:TokenType::CLASS});
        assert_eq!(t.next().unwrap(), Token{ pos:(2, 7),  t:TokenType::VAR});
        assert_eq!(t.next().unwrap(), Token{ pos:(2, 11),  t:TokenType::ICOMMENT(String::from(" this is a comment"))});
        assert_eq!(t.next().unwrap(), Token{ pos:(3, 1),  t:TokenType::CLASS});
        assert_eq!(t.next().unwrap(), Token{ pos:(3, 7),  t:TokenType::APICOMMENT(String::from(" multiline api \n      comment "))});
        assert_eq!(t.next().unwrap(), Token{ pos:(5, 1),  t:TokenType::LET});
        assert_eq!(t.next().unwrap(), Token{ pos:(5, 5),  t:TokenType::COMMENT(String::from(" inline comment "))});
        assert_eq!(t.next().unwrap(), Token{ pos:(5, 26),  t:TokenType::IDENT(String::from("x"))});
        assert_eq!(t.next().unwrap(), Token{ pos:(6, 1),  t:TokenType::INVALID});
        assert_eq!(t.next(), None);
        assert_eq!(t.next(), None);
        assert_eq!(t.next(), None);
        assert_eq!(t.next(), None);
    }
}
