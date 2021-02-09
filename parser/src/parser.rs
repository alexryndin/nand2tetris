use crate::token::{Token, TokenType};
use std::io::Write;



macro_rules! expect_and_write {
    ($self:ident, $( $tt:pat ),+ ) => {
        match $self.t.next() {
            None => Err(format!("EOF while parsing")),
            Some(t) => {
                match t.t {
                    $(
                        $tt => $self.write_term(t),
                    )+
                    _ => Err(format!("'{:?}' expected {}", t, stringify!($($tt)+))),
                }
            },
        }
    }
}

pub struct Parser<T: Iterator<Item=Token>, W: Write> {
    t: itertools::MultiPeek<T>,
    w: W,
}

impl<T: Iterator<Item=Token>, W: Write> Parser<T, W> {
    pub fn new(t: T, w: W) -> Parser<T, W> {
        Parser {
            t: itertools::multipeek(t),
            w: w,
        }
    }
    fn write_term(&mut self, t: Token) -> Result<(), String> {
        let ret = match t.t {
            TokenType::LPAREN         => Ok(format!("<symbol>(</symbol>")),
            TokenType::RPAREN         => Ok(format!("<symbol>)</symbol>")),
            TokenType::LCPAREN        => Ok(format!("<symbol>{{</symbol>")),
            TokenType::RCPAREN        => Ok(format!("<symbol>}}</symbol>")),
            TokenType::LBRACKET       => Ok(format!("<symbol>[</symbol>")),
            TokenType::RBRACKET       => Ok(format!("<symbol>]</symbol>")),
            TokenType::COMMA          => Ok(format!("<symbol>,</symbol>")),
            TokenType::SEMICOLON      => Ok(format!("<symbol>;</symbol>")),
            TokenType::EQ             => Ok(format!("<symbol>=</symbol>")),
            TokenType::DOT            => Ok(format!("<symbol>.</symbol>")),
            TokenType::PLUS           => Ok(format!("<symbol>+</symbol>")),
            TokenType::MINUS          => Ok(format!("<symbol>-</symbol>")),
            TokenType::MUL            => Ok(format!("<symbol>*</symbol>")),
            TokenType::DIV            => Ok(format!("<symbol>/</symbol>")),
            TokenType::AMP            => Ok(format!("<symbol>&amp;</symbol>")),
            TokenType::PIPE           => Ok(format!("<symbol>|</symbol>")),
            TokenType::TILDA          => Ok(format!("<symbol>~</symbol>")),
            TokenType::LT             => Ok(format!("<symbol>&lt;</symbol>")),
            TokenType::GT             => Ok(format!("<symbol>&gt;</symbol>")),
            TokenType::IDENT(s)       => Ok(format!("<identifier>{}</identifier>", s)),
            TokenType::STRING(s)      => Ok(format!("<stringConstant>{}</stringConstant>", s)),
            TokenType::NUM(n)         => Ok(format!("<integerConstant>{}</integerConstant>", n)),
            TokenType::CLASS          => Ok(format!("<keyword>class</keyword>")),
            TokenType::CONSTRUCTOR    => Ok(format!("<keyword>constructor</keyword>")),
            TokenType::METHOD         => Ok(format!("<keyword>method</keyword>")),
            TokenType::FUNCTION       => Ok(format!("<keyword>function</keyword>")),
            TokenType::INT            => Ok(format!("<keyword>int</keyword>")),
            TokenType::BOOL           => Ok(format!("<keyword>boolean</keyword>")),
            TokenType::CHAR           => Ok(format!("<keyword>char</keyword>")),
            TokenType::VOID           => Ok(format!("<keyword>void</keyword>")),
            TokenType::VAR            => Ok(format!("<keyword>var</keyword>")),
            TokenType::STATIC         => Ok(format!("<keyword>static</keyword>")),
            TokenType::FIELD          => Ok(format!("<keyword>field</keyword>")),
            TokenType::LET            => Ok(format!("<keyword>let</keyword>")),
            TokenType::DO             => Ok(format!("<keyword>do</keyword>")),
            TokenType::IF             => Ok(format!("<keyword>if</keyword>")),
            TokenType::ELSE           => Ok(format!("<keyword>else</keyword>")),
            TokenType::WHILE          => Ok(format!("<keyword>while</keyword>")),
            TokenType::RETURN         => Ok(format!("<keyword>return</keyword>")),
            TokenType::TRUE           => Ok(format!("<keyword>true</keyword>")),
            TokenType::FALSE          => Ok(format!("<keyword>false</keyword>")),
            TokenType::NULL           => Ok(format!("<keyword>null</keyword>")),
            TokenType::THIS           => Ok(format!("<keyword>this</keyword>")),
            _                         => Err(format!("Unknown token to write -- {:?}", t.t)),

        }?;
        match write!(self.w, "{}\n", ret) {
            Err(e) => Err(e.to_string()),
            Ok(_) => Ok(()),
        }
    }

    fn write_tag(&mut self, t: &str, open: bool) -> Result<(), String> {
        let tag = format!("<{}{}>", if open {""} else {"/"}, t);
        match write!(self.w, "{}\n", tag) {
            Err(e) => Err(e.to_string()),
            Ok(_) => Ok(()),
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.parse_class()?;
        Ok(())

    }

    fn parse_class_var_dec(&mut self) -> Result<(), String> {
        self.write_tag("classVarDec", true)?;
        expect_and_write!(self, TokenType::STATIC, TokenType::FIELD)?;
        expect_and_write!(self, TokenType::INT,
                                TokenType::CHAR,
                                TokenType::BOOL,
                                TokenType::IDENT(_))?;
        expect_and_write!(self, TokenType::IDENT(_))?;
        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::COMMA => {
                    expect_and_write!(self, TokenType::COMMA)?;
                    expect_and_write!(self, TokenType::IDENT(_))?;
                }
                _ => break,
            }
        };
        expect_and_write!(self, TokenType::SEMICOLON)?;
        self.write_tag("classVarDec", false)?;
        Ok(())
    }

    fn parse_var_dec(&mut self) -> Result<(), String> {
        self.write_tag("varDec", true)?;
        expect_and_write!(self, TokenType::VAR)?;

        expect_and_write!(self, TokenType::INT,
                                TokenType::CHAR,
                                TokenType::BOOL,
                                TokenType::IDENT(_))?;

        expect_and_write!(self, TokenType::IDENT(_))?;

        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::COMMA => {
                    expect_and_write!(self, TokenType::COMMA)?;
                    expect_and_write!(self, TokenType::IDENT(_))?;
                }
                _ => break,
            }
        };

        expect_and_write!(self, TokenType::SEMICOLON)?;

        self.write_tag("varDec", false)?;
        Ok(())
    }

    fn parse_parameter_list(&mut self) -> Result<(), String> {
        self.write_tag("parameterList", true)?;

        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::INT |
                TokenType::CHAR|
                TokenType::BOOL|
                TokenType::IDENT(_) => {
                    expect_and_write!(self, TokenType::INT,
                                            TokenType::CHAR,
                                            TokenType::BOOL,
                                            TokenType::IDENT(_))?;
                    expect_and_write!(self, TokenType::IDENT(_))?;
                    if let Some(t) = self.t.peek() {
                        match t.t {
                            TokenType::COMMA => {
                                expect_and_write!(self, TokenType::COMMA)?;
                                continue
                            }
                            _ => continue
                        }
                    }
                }
                _ => break,
            }
        };

        self.write_tag("parameterList", false)
    }

    fn parse_subroutine_dec(&mut self) -> Result<(), String> {
        self.write_tag("subroutineDec", true)?;
        expect_and_write!(self, TokenType::CONSTRUCTOR,
                                TokenType::FUNCTION,
                                TokenType::METHOD)?;

        expect_and_write!(self, TokenType::VOID,
                                TokenType::INT,
                                TokenType::CHAR,
                                TokenType::BOOL,
                                TokenType::IDENT(_))?;

        expect_and_write!(self, TokenType::IDENT(_))?;
        expect_and_write!(self, TokenType::LPAREN)?;
        self.parse_parameter_list()?;
        expect_and_write!(self, TokenType::RPAREN)?;
        self.parse_subroutine_body()?;
        self.write_tag("subroutineDec", false)
    }

    fn parse_expression_list(&mut self) -> Result<(), String>  {
        self.write_tag("expressionList", true)?;
        while let Some(t) = self.t.peek() {
            match t.t { 
                TokenType::COMMA => {
                    expect_and_write!(self, TokenType::COMMA)?;
                    self.parse_expression()?;
                },
                TokenType::RPAREN => break,
                _ => {
                    self.t.reset_peek();
                    self.parse_expression()?
                },
            }
        };
        self.write_tag("expressionList", false)
    }

    fn parse_subroutine_call(&mut self) -> Result<(), String>  {
        expect_and_write!(self, TokenType::IDENT(_))?;
        if let Some(t) = self.t.peek() {
            if t.t == TokenType::DOT {
                expect_and_write!(self, TokenType::DOT)?;
                expect_and_write!(self, TokenType::IDENT(_))?;
            }
        }
        expect_and_write!(self, TokenType::LPAREN)?;
        self.parse_expression_list()?;
        expect_and_write!(self, TokenType::RPAREN)
    }

    fn parse_term(&mut self) -> Result<(), String>  {
        self.write_tag("term", true)?;
        if let Some(t) = self.t.peek() {
            match t.t {
                TokenType::NUM(_) => expect_and_write!(self, TokenType::NUM(_))?,
                TokenType::STRING(_) => expect_and_write!(self, TokenType::STRING(_))?,
                TokenType::THIS => expect_and_write!(self, TokenType::THIS)?,
                TokenType::NULL => expect_and_write!(self, TokenType::NULL)?,
                TokenType::TRUE => expect_and_write!(self, TokenType::TRUE)?,
                TokenType::FALSE => expect_and_write!(self, TokenType::FALSE)?,
                TokenType::IDENT(_) => {
                    if let Some(t) = self.t.peek() {
                        match t.t {
                            TokenType::LBRACKET => {
                                expect_and_write!(self, TokenType::IDENT(_))?;
                                expect_and_write!(self, TokenType::LBRACKET)?;
                                self.parse_expression()?;
                                expect_and_write!(self, TokenType::RBRACKET)?;
                            }
                            TokenType::LPAREN | TokenType::DOT => self.parse_subroutine_call()?,
                            _ => expect_and_write!(self, TokenType::IDENT(_))?
                        }
                    };
                }
                TokenType::LPAREN => {
                    expect_and_write!(self, TokenType::LPAREN)?;
                    self.parse_expression()?;
                    expect_and_write!(self, TokenType::RPAREN)?;
                }
                TokenType::TILDA | TokenType::MINUS => {
                    expect_and_write!(self, TokenType::TILDA, TokenType::MINUS)?;
                    self.parse_term()?;
                }

                _ => return Err(format!("Expected term, found {:?}", t))
            }
        }
        self.t.reset_peek();
        self.write_tag("term", false)
    }

    fn parse_expression(&mut self) -> Result<(), String>  {
        self.write_tag("expression", true)?;
        self.parse_term()?;
        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::PLUS | 
                TokenType::MINUS |
                TokenType::MUL |
                TokenType::DIV |
                TokenType::AMP |
                TokenType::PIPE |
                TokenType::LT |
                TokenType::GT |
                TokenType::EQ => {
                    expect_and_write!(self, TokenType::PLUS,
                                            TokenType::MINUS,
                                            TokenType::MUL,
                                            TokenType::DIV,
                                            TokenType::AMP,
                                            TokenType::PIPE,
                                            TokenType::LT,
                                            TokenType::GT,
                                            TokenType::EQ)?;
                    self.parse_term()?;
                }
                _ => break,
            }
        }
        self.t.reset_peek();
        self.write_tag("expression", false)
    }

    fn parse_let_statement(&mut self) -> Result<(), String>  {
        self.write_tag("letStatement", true)?;
        expect_and_write!(self, TokenType::LET)?;
        expect_and_write!(self, TokenType::IDENT(_))?;
        if let Some(t) = self.t.peek() {
            if t.t == TokenType::LBRACKET {
                expect_and_write!(self, TokenType::LBRACKET)?;
                self.parse_expression()?;
                expect_and_write!(self, TokenType::RBRACKET)?;
            }
        };

        expect_and_write!(self, TokenType::EQ)?;
        self.parse_expression()?;
        expect_and_write!(self, TokenType::SEMICOLON)?;
        self.write_tag("letStatement", false)
    }

    fn parse_while_statement(&mut self) -> Result<(), String>  {
        self.write_tag("whileStatement", true)?;

        expect_and_write!(self, TokenType::WHILE)?;
        expect_and_write!(self, TokenType::LPAREN)?;
        self.parse_expression()?;
        expect_and_write!(self, TokenType::RPAREN)?;
        expect_and_write!(self, TokenType::LCPAREN)?;
        self.parse_statements()?;
        expect_and_write!(self, TokenType::RCPAREN)?;

        self.write_tag("whileStatement", false)
    }

    fn parse_if_statement(&mut self) -> Result<(), String>  {
        self.write_tag("ifStatement", true)?;

        expect_and_write!(self, TokenType::IF)?;
        expect_and_write!(self, TokenType::LPAREN)?;
        self.parse_expression()?;
        expect_and_write!(self, TokenType::RPAREN)?;
        expect_and_write!(self, TokenType::LCPAREN)?;
        self.parse_statements()?;
        expect_and_write!(self, TokenType::RCPAREN)?;
        if let Some(t) = self.t.peek() {
            if t.t == TokenType::ELSE {
                expect_and_write!(self, TokenType::ELSE)?;
                expect_and_write!(self, TokenType::LCPAREN)?;
                self.parse_statements()?;
                expect_and_write!(self, TokenType::RCPAREN)?;
            }
        };
        self.t.reset_peek();

        self.write_tag("ifStatement", false)
    }

    fn parse_do_statement(&mut self) -> Result<(), String>  {
        self.write_tag("doStatement", true)?;

        expect_and_write!(self, TokenType::DO)?;

        self.parse_subroutine_call()?;

        expect_and_write!(self, TokenType::SEMICOLON)?;

        self.write_tag("doStatement", false)
    }

    fn parse_return_statement(&mut self) -> Result<(), String>  {
        self.write_tag("returnStatement", true)?;

        expect_and_write!(self, TokenType::RETURN)?;

        if let Some(t) = self.t.peek() {
            if t.t == TokenType::SEMICOLON {
                expect_and_write!(self, TokenType::SEMICOLON)?;
            } else {
                self.t.reset_peek();
                self.parse_expression()?;
                expect_and_write!(self, TokenType::SEMICOLON)?;
            }
        };

        self.write_tag("returnStatement", false)
    }

    fn parse_statements(&mut self) -> Result<(), String> {
        self.write_tag("statements", true)?;

        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::LET => self.parse_let_statement()?,
                TokenType::IF => self.parse_if_statement()?,
                TokenType::WHILE => self.parse_while_statement()?,
                TokenType::DO => self.parse_do_statement()?,
                TokenType::RETURN => self.parse_return_statement()?,
                TokenType::RCPAREN => break,
                _ => return Err(format!("Expected statement, found {:?}", t)),
            }
        };
        self.t.reset_peek();

        self.write_tag("statements", false)
    }

    fn parse_subroutine_body(&mut self) -> Result<(), String> {
        self.write_tag("subroutineBody", true)?;
        expect_and_write!(self, TokenType::LCPAREN)?;

        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::VAR => self.parse_var_dec()?,
                _ => break,
            }
        };
        self.t.reset_peek();
        self.parse_statements()?;
        expect_and_write!(self, TokenType::RCPAREN)?;
        self.write_tag("subroutineBody", false)?;
        Ok(())
    }

    fn parse_class(&mut self) -> Result<(), String> {
        self.write_tag("class", true)?;
        expect_and_write!(self, TokenType::CLASS)?;
        expect_and_write!(self, TokenType::IDENT(_))?;
        expect_and_write!(self, TokenType::LCPAREN)?;
        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::STATIC | TokenType::FIELD => self.parse_class_var_dec()?,
                _ => break,

            }
        };
        self.t.reset_peek();
        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::CONSTRUCTOR |
                TokenType::FUNCTION    |
                TokenType::METHOD => self.parse_subroutine_dec()?,
                _ => break,

            }
        };
        expect_and_write!(self, TokenType::RCPAREN)?;
        self.write_tag("class", false)?;
        Ok(())
    }
}



