use crate::token::{Token, TokenType};
use crate::symbol_table::{SymbolTable, Kind, Type, Ident, Symbol};
use std::io::Write;
use std::clone::Clone;
use std::convert::TryInto;


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

enum FnKind {
    FUNCTION,
    METHOD,
    CONSTRUCTOR,
}

struct Label(i32);

enum PP {
    PUSH,
    POP,
}

enum Segment {
    CONST,
    ARG,
    LOCAL,
    STATIC,
    THIS,
    THAT,
    POINTER,
    TEMP,
}

enum Op {
    ADD,
    SUB,
    NEG,
    EQ,
    GT,
    LT,
    AND,
    OR,
    NOT,
}

pub struct Parser<T: Iterator<Item=Token>, W: Write> {
    t: itertools::MultiPeek<T>,
    w: W,
    src: String,
    idx: i32,
    pub st: SymbolTable,
}

impl<T: Iterator<Item=Token>, W: Write> Parser<T, W> {
    pub fn new(t: T, w: W, src: String) -> Parser<T, W> {
        Parser {
            t: itertools::multipeek(t),
            w: w,
            src: src,
            idx: -1,
            st: SymbolTable::new(),
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
        Ok(())
    }

    fn write_tag(&mut self, t: &str, open: bool) -> Result<(), String> {
        let tag = format!("<{}{}>", if open {""} else {"/"}, t);
        Ok(())
    }

    fn write_all(&mut self, s: &str) -> Result<(), String> {
        match self.w.write_all(s.as_bytes()) {
            Err(e) => Err(e.to_string()),
            Ok(_) => Ok(()),
        }
    }

    fn get_idx(&mut self) -> i32 {
        self.idx += 1;
        self.idx
    }

    fn get_label(&mut self) -> Label {
        Label(self.get_idx())
    }

    pub fn parse(&mut self) -> Result<(), String> {
        self.parse_class()?;
        self.st.show();
        Ok(())
    }

    fn parse_class_var_dec(&mut self) -> Result<(), String> {
        self.write_tag("classVarDec", true)?;
        let kind = Kind::from_token(&self.t.next().unwrap()).unwrap();
        let typ = Type::from_token(self.t.peek().unwrap()).unwrap();
        expect_and_write!(self, TokenType::INT,
                                TokenType::CHAR,
                                TokenType::BOOL,
                                TokenType::IDENT(_))?;
        self.st.define(Ident::from_token(self.t.peek().unwrap()).unwrap(), &typ, &kind);
        expect_and_write!(self, TokenType::IDENT(_))?;
        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::COMMA => {
                    expect_and_write!(self, TokenType::COMMA)?;
                    self.st.define(Ident::from_token(self.t.peek().unwrap()).unwrap(), &typ, &kind);
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

        let kind = Kind::VAR;

        let typ = Type::from_token(
            &self.t.next().ok_or(String::from("vartype expected"))?
        ).unwrap();

        let ident = Ident::from_token(
            &self.t.next().ok_or(String::from("vartype expected"))?
        ).unwrap();

        self.st.define(ident, &typ, &kind);

        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::COMMA => {
                    expect_and_write!(self, TokenType::COMMA)?;
                    let ident = Ident::from_token(
                        &self.t.next().ok_or(String::from("vartype expected"))?
                    ).unwrap();
                    self.st.define(ident, &typ, &kind);
                }
                _ => break,
            }
        };

        expect_and_write!(self, TokenType::SEMICOLON)?;

        self.write_tag("varDec", false)?;
        Ok(())
    }

    fn parse_parameter_list(&mut self) -> Result<i32, String> {
        self.write_tag("parameterList", true)?;
        let mut ret = 0;
        let kind = Kind::ARG;

        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::INT |
                TokenType::CHAR|
                TokenType::BOOL|
                TokenType::IDENT(_) => {
                    ret += 1;
                    let typ = Type::from_token(
                        &self.t.next().ok_or(String::from("vartype expected"))?
                    ).unwrap();

                    let ident = Ident::from_token(
                        &self.t.next().ok_or(String::from("vartype expected"))?
                    ).unwrap();
                    self.st.define(ident, &typ, &kind);
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

        self.write_tag("parameterList", false)?;
        Ok(ret)
    }

    fn get_subroutine_kind(&mut self) -> Result<FnKind, String> {
        match self.t.next().unwrap().t {
            TokenType::CONSTRUCTOR => Ok(FnKind::CONSTRUCTOR),
            TokenType::FUNCTION => Ok(FnKind::FUNCTION),
            TokenType::METHOD => Ok(FnKind::METHOD),
            s => Err(format!("expected func type, got {:?}", s)),
        }
    }

    fn parse_subroutine_var_dec(&mut self) -> Result<(), String> {
        self.write_tag("subroutineBody", true)?;
        expect_and_write!(self, TokenType::LCPAREN)?;

        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::VAR => self.parse_var_dec()?,
                _ => break,
            }
        };
        self.t.reset_peek();
        Ok(())
    }

    fn parse_subroutine_dec(&mut self) -> Result<(), String> {
        self.write_tag("subroutineDec", true)?;
        self.st.new_subroutine();
        let mut name = String::from(&self.src);
        name.push('.');
        let sk = self.get_subroutine_kind()?;

        if let FnKind::METHOD = sk {
            self.st.inc_ai();
        };

        let typ = Type::from_token(
            &self.t.next().ok_or(String::from("functype expected"))?
        ).unwrap();

        //expect_and_write!(self, TokenType::IDENT(_))?;
        let fname = self.t.next().unwrap().t;
        name.push_str(fname.get_str().ok_or(String::from("expected function name"))?);
        expect_and_write!(self, TokenType::LPAREN)?;
        let _nargs = self.parse_parameter_list()?;
        expect_and_write!(self, TokenType::RPAREN)?;
        self.parse_subroutine_var_dec()?;
        self.st.show();
        let nlocals = self.st.var_count(&Kind::VAR);
        self.write_func(&name, nlocals)?;
        match sk {
            FnKind::METHOD => {
                self.write_push_pop(&Segment::ARG, &PP::PUSH, 0)?;
                self.write_push_pop(&Segment::POINTER, &PP::POP, 0)?;
            },
            FnKind::CONSTRUCTOR => {
                let nfields = self.st.var_count(&Kind::FIELD);
                self.write_push_pop(&Segment::CONST, &PP::PUSH, nfields)?;
                self.write_call("Memory.alloc", 1)?;
                self.write_push_pop(&Segment::POINTER, &PP::POP, 0)?;
            },
            _ => (),
        };
        self.parse_subroutine_body()?;
        if typ == Type::VOID {
            self.write_push_pop(&Segment::CONST, &PP::PUSH, 0)? // void functions must return 0
        }
        self.write_return()?;
        self.write_tag("subroutineDec", false)
    }

    fn parse_expression_list(&mut self) -> Result<i32, String>  {
        self.write_tag("expressionList", true)?;
        let mut ret = 0;
        while let Some(t) = self.t.peek() {
            match t.t {
                TokenType::COMMA => {
                    expect_and_write!(self, TokenType::COMMA)?;
                    ret += 1;
                    self.parse_expression()?;
                },
                TokenType::RPAREN => break,
                _ => {
                    self.t.reset_peek();
                    ret += 1;
                    self.parse_expression()?
                },
            }
        };
        self.write_tag("expressionList", false)?;
        Ok(ret)
    }

    fn parse_subroutine_call(&mut self) -> Result<(), String>  {
        let mut f = String::new();
        let mut c = String::new();
        f.push_str(self.t.next().unwrap().t.get_str().ok_or(String::from("function name or class expected"))?);
        if let Some(t) = self.t.peek() {
            if t.t == TokenType::DOT {
                std::mem::swap(&mut c, &mut f);
                f.clear();
                self.t.next().unwrap();
                f.push_str(self.t.next().unwrap().t.get_str().ok_or(String::from("method expected"))?);
            }
        };
        let mut kind = FnKind::FUNCTION;
        let obj_name = self.st.get(&Ident::from_str(&c));
        if let Some(_) = obj_name {
            kind = FnKind::METHOD;
            let obj = c.clone();
            c.clear();
            c.push_str(&obj_name.unwrap().t.get_ident().unwrap().0);
            self.write_variable(&obj)?;
        };
        if c.len() == 0 {
            kind = FnKind::METHOD;
            self.write_push_pop(&Segment::POINTER, &PP::PUSH, 0)?;
        };
        expect_and_write!(self, TokenType::LPAREN)?;
        let mut nargs = 0;
        if let FnKind::METHOD = kind {
            nargs+=1;
        }
        let callname = if c.len() > 0 {
            format!("{}.{}", c, f)
        } else {
            format!("{}.{}", self.src, f)
        };
        nargs += self.parse_expression_list()?;
        self.write_call(&callname, nargs)?;
        expect_and_write!(self, TokenType::RPAREN)
    }

    fn parse_term(&mut self) -> Result<(), String>  {
        self.write_tag("term", true)?;
        if let Some(t) = self.t.peek() {
            match &t.t {
                TokenType::NUM(n) => {
                    let n = *n;
                    self.t.next();
                    self.write_push_pop(&Segment::CONST, &PP::PUSH, n.into())?
                },
                TokenType::STRING(s) => {
                    let s = s.clone(); // TODO: Can I make it without clone?
                    self.t.next();
                    self.write_string(&s.clone())?;
                },
                TokenType::THIS => {
                    self.t.next();
                    self.write_push_pop(&Segment::POINTER, &PP::PUSH, 0)?;
                },
                TokenType::NULL => {
                    self.t.next();
                    self.write_push_pop(&Segment::CONST, &PP::PUSH, 0)?;
                },
                TokenType::TRUE => {
                    self.t.next();
                    self.write_push_pop(&Segment::CONST, &PP::PUSH, 0)?;
                    self.write_arithmetic(&TokenType::TILDA, true)?;
                },
                TokenType::FALSE => {
                    self.t.next();
                    self.write_push_pop(&Segment::CONST, &PP::PUSH, 0)?;
                },
                TokenType::IDENT(_) => {
                    if let Some(t) = self.t.peek() {
                        match t.t {
                            TokenType::LBRACKET => {
                                let var = &self.t.next().unwrap().t;
                                self.write_variable_from_token(var)?;
                                expect_and_write!(self, TokenType::LBRACKET)?;
                                self.parse_expression()?;
                                self.write_arithmetic(&TokenType::PLUS, false)?;
                                self.write_push_pop(&Segment::POINTER, &PP::POP, 1)?;
                                self.write_push_pop(&Segment::THAT, &PP::PUSH, 0)?;
                                expect_and_write!(self, TokenType::RBRACKET)?;
                            }
                            TokenType::LPAREN | TokenType::DOT => self.parse_subroutine_call()?,
                            _ => {
                                let var = &self.t.next().unwrap().t;
                                self.write_variable_from_token(var)?;
                            },
                        }
                    };
                }
                TokenType::LPAREN => {
                    expect_and_write!(self, TokenType::LPAREN)?;
                    self.parse_expression()?;
                    expect_and_write!(self, TokenType::RPAREN)?;
                }
                TokenType::TILDA | TokenType::MINUS => {
                    let tt = &self.t.next().unwrap().t;
                    self.parse_term()?;
                    self.write_arithmetic(tt, true)?;
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
                    let tt = &self.t.next().unwrap().t;
                    self.parse_term()?;
                    self.write_arithmetic(tt, false)?;
                }
                _ => break,
            }
        }
        self.t.reset_peek();
        self.write_tag("expression", false)
    }

    fn parse_let_statement(&mut self) -> Result<(), String>  {
        self.write_tag("letStatement", true)?;
        let mut let_array = false;
        expect_and_write!(self, TokenType::LET)?;
        let tt = &self.t.next().unwrap();
        let ident = Ident::from_token(tt).ok_or(String::from("ident expected"))?;
        let symbol = (*self.st.get(&ident).ok_or(format!("variable {:?} not found", &ident))?).clone();
        let idx = symbol.n;
        let segment = match symbol.k {
            Kind::VAR => Segment::LOCAL,
            Kind::ARG => Segment::ARG,
            Kind::FIELD => Segment::THIS,
            Kind::STATIC => Segment::STATIC,

        };
        if let Some(t) = self.t.peek() {
            if t.t == TokenType::LBRACKET {
                let_array = true;
                expect_and_write!(self, TokenType::LBRACKET)?;
                self.write_symbol(&symbol)?;
                self.parse_expression()?;
                self.write_arithmetic(&TokenType::PLUS, false)?;
                self.write_push_pop(&Segment::TEMP, &PP::POP, 1)?; // save array address
                expect_and_write!(self, TokenType::RBRACKET)?;
            }
        };

        expect_and_write!(self, TokenType::EQ)?;
        self.parse_expression()?;
        match let_array {
            false => self.write_push_pop(&segment, &PP::POP, idx)?,
            true => {
                self.write_push_pop(&Segment::TEMP, &PP::PUSH, 1)?; // retrieve saved array address
                self.write_push_pop(&Segment::POINTER, &PP::POP, 1)?;
                self.write_push_pop(&Segment::THAT, &PP::POP, 0)?;
            },
        };
        expect_and_write!(self, TokenType::SEMICOLON)?;
        self.write_tag("letStatement", false)
    }

    fn parse_while_statement(&mut self) -> Result<(), String>  {
        self.write_tag("whileStatement", true)?;
        let (l1, l2) = (self.get_label(), self.get_label());
        self.write_label(&l1)?;

        expect_and_write!(self, TokenType::WHILE)?;
        expect_and_write!(self, TokenType::LPAREN)?;
        self.parse_expression()?;
        self.write_arithmetic(&TokenType::TILDA, true)?;
        self.write_if_goto(&l2)?;
        expect_and_write!(self, TokenType::RPAREN)?;
        expect_and_write!(self, TokenType::LCPAREN)?;
        self.parse_statements()?;

        self.write_goto(&l1)?;

        self.write_label(&l2)?;

        expect_and_write!(self, TokenType::RCPAREN)?;

        self.write_tag("whileStatement", false)
    }

    fn parse_if_statement(&mut self) -> Result<(), String>  {
        self.write_tag("ifStatement", true)?;
        let (l1, l2) = (self.get_label(), self.get_label());
        expect_and_write!(self, TokenType::IF)?;
        expect_and_write!(self, TokenType::LPAREN)?;
        self.parse_expression()?;
        self.write_arithmetic(&TokenType::TILDA, true)?;
        self.write_if_goto(&l1)?;
        expect_and_write!(self, TokenType::RPAREN)?;
        expect_and_write!(self, TokenType::LCPAREN)?;
        self.parse_statements()?;
        self.write_goto(&l2)?;
        self.write_label(&l1)?;
        expect_and_write!(self, TokenType::RCPAREN)?;
        if let Some(t) = self.t.peek() {
            if t.t == TokenType::ELSE {
                expect_and_write!(self, TokenType::ELSE)?;
                expect_and_write!(self, TokenType::LCPAREN)?;
                self.parse_statements()?;
                expect_and_write!(self, TokenType::RCPAREN)?;
            }
        };
        self.write_label(&l2)?;
        self.t.reset_peek();

        self.write_tag("ifStatement", false)
    }

    fn parse_do_statement(&mut self) -> Result<(), String>  {
        self.write_tag("doStatement", true)?;

        expect_and_write!(self, TokenType::DO)?;

        self.parse_subroutine_call()?;

        expect_and_write!(self, TokenType::SEMICOLON)?;
        self.write_push_pop(&Segment::TEMP, &PP::POP, 0)?; // Ignore returned value

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

    fn write_push_pop(&mut self, seg: &Segment, p: &PP, idx: i32) -> Result<(), String> {
        let mut s = String::new();
        match p {
            PP::POP => s.push_str("pop "),
            PP::PUSH => s.push_str("push "),
        };
        match seg {
            Segment::CONST => s.push_str("constant "),
            Segment::ARG => s.push_str("argument "),
            Segment::LOCAL => s.push_str("local "),
            Segment::STATIC => s.push_str("static "),
            Segment::THIS => s.push_str("this "),
            Segment::THAT => s.push_str("that "),
            Segment::POINTER => s.push_str("pointer "),
            Segment::TEMP => s.push_str("temp "),
        }

        s.push_str(&idx.to_string());
        s.push('\n');
        self.write_all(&s)
    }

    fn write_arithmetic(&mut self, op: &TokenType, unary: bool) -> Result<(), String> {
        let mut s = String::new();
        match op {
            TokenType::PLUS => s.push_str("add"),
            TokenType::MINUS => match unary {
                true => s.push_str("neg"),
                false => s.push_str("sub"),
            }
            TokenType::TILDA => s.push_str("not"),
            TokenType::EQ => s.push_str("eq"),
            TokenType::GT => s.push_str("gt"),
            TokenType::LT => s.push_str("lt"),
            TokenType::AMP => s.push_str("and"),
            TokenType::PIPE => s.push_str("or"),
            TokenType::MUL => s.push_str("call Math.multiply 2"),
            TokenType::DIV => s.push_str("call Math.divide 2"),
            _ => return Err(format!("'{:?}' is not an operator", op)),
        }
        s.push('\n');
        self.write_all(&s)
    }

    fn write_call(&mut self, name: &str, nargs: i32) -> Result<(), String> {
        self.write_all(&format!("call {} {}\n", name, nargs))
    }

    fn write_func(&mut self, name: &str, nargs: i32) -> Result<(), String> {
        self.write_all(&format!("function {} {}\n", name, nargs))
    }

    fn write_return(&mut self) -> Result<(), String> {
        self.write_all(&format!("return\n"))
    }

    fn write_symbol(&mut self, s: &Symbol) -> Result<(), String> {
        let n = s.n; match s.k {
            Kind::STATIC => self.write_push_pop(&Segment::STATIC, &PP::PUSH, n)
,
            Kind::FIELD => self.write_push_pop(&Segment::THIS, &PP::PUSH, n),
            Kind::ARG => self.write_push_pop(&Segment::ARG, &PP::PUSH, n),
            Kind::VAR => self.write_push_pop(&Segment::LOCAL, &PP::PUSH, n),
        }
    }

    fn write_variable(&mut self, var: &str) -> Result<(), String> {
        match self.st.get_mut(&Ident::from_str(var)) {
            Some(s) => {
                let symbol = s.clone();
                self.write_symbol(&symbol)
            }
            None => return Err(format!("'{}' is not a variable!", var)),
        }
    }

    fn write_variable_from_token(&mut self, var: &TokenType) -> Result<(), String> {
        match var {
            TokenType::IDENT(var) => self.write_variable(var),
            _ => Err(String::from("Not a variable!")),
        }
    }

    fn write_if_goto(&mut self, l: &Label) -> Result<(), String> {
        self.write_all(&format!("if-goto LABEL_{}\n", l.0))
    }

    fn write_goto(&mut self, l: &Label) -> Result<(), String> {
        self.write_all(&format!("goto LABEL_{}\n", l.0))
    }

    fn write_label(&mut self, l: &Label) -> Result<(), String> {
        self.write_all(&format!("label LABEL_{}\n", l.0))
    }

    fn write_string(&mut self, s: &str) -> Result<(), String> {
        self.write_push_pop(&Segment::CONST, &PP::PUSH, s.len().try_into().unwrap())?;
        self.write_call("String.new", 1)?;
        for c in s.chars() {
            self.write_push_pop(&Segment::CONST, &PP::PUSH, c as i32)?;
            self.write_call("String.appendChar", 2)?;
        }
        Ok(())
    }
}



