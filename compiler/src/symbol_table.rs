use std::collections::HashMap;
use crate::token::{Token, TokenType};

#[derive(PartialEq, Hash, Eq, Debug, Clone)]
pub struct Ident(pub String);


impl Ident {
    pub fn from_token(t: &Token) -> Option<Ident> {
        match &t.t {
            TokenType::IDENT(s) => Some(Ident(s.clone())),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Ident {
        Ident(s.to_string())
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Kind {
    STATIC,
    FIELD,
    ARG,
    VAR,
}

impl Kind {
    pub fn from_token(t: &Token) -> Option<Kind> {
        match &t.t {
            TokenType::STATIC => Some(Kind::STATIC),
            TokenType::FIELD => Some(Kind::FIELD),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    VOID,
    INT,
    CHAR,
    BOOL,
    OFCLASS(Ident),
}

impl Type {
    pub fn from_token(t: &Token) -> Option<Type> {
        match &t.t {
            TokenType::VOID => Some(Type::VOID),
            TokenType::INT => Some(Type::INT),
            TokenType::CHAR => Some(Type::CHAR),
            TokenType::BOOL => Some(Type::BOOL),
            TokenType::IDENT(s) => Some(Type::OFCLASS(Ident(s.clone()))),
            _ => None,
        }
    }
    pub fn get_ident(&self) -> Option<&Ident> {
        match self {
            Type::OFCLASS(id) => Some(&id),
            _ => None,
        }
    }
}


#[derive(Debug, Clone)]
pub struct Symbol {
    pub k: Kind,
    pub t: Type,
    pub n: i32,
}

pub struct SymbolTable {
    cs: HashMap<Ident, Symbol>,
    ss: HashMap<Ident, Symbol>,
    si: i32,
    fi: i32,
    ai: i32,
    vi: i32,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            cs: HashMap::new(),
            ss: HashMap::new(),
            si: -1,
            fi: -1,
            ai: -1,
            vi: -1,
        }
    }

    pub fn inc_ai(&mut self) {
        self.ai += 1;
    }

    pub fn new_subroutine(&mut self) {
        self.ss = HashMap::new();
        self.ai = -1;
        self.vi = -1;
    }

    fn idx(&mut self, k: &Kind) -> i32 {
        match k {
            Kind::STATIC => {
                self.si+=1;
                self.si
            }
            Kind::FIELD => {
                self.fi+=1;
                self.fi
            }
            Kind::ARG => {
                self.ai+=1;
                self.ai
            }
            Kind::VAR => {
                self.vi+=1;
                self.vi
            }
        }
    }

    pub fn define(&mut self, name: Ident, t: &Type, k: &Kind) {
        let idx = self.idx(&k);
        let s = Symbol {
            k: k.clone(),
            t: t.clone(),
            n: idx,
        };
        match s.k {
            Kind::STATIC | Kind::FIELD => {
                self.cs.insert(name, s);
            },
            Kind::ARG | Kind::VAR => {
                self.ss.insert(name, s);
            },
        }
    }

    pub fn var_count(&self, k: &Kind) -> i32 {
        match k {
            Kind::STATIC | Kind::FIELD => {
                self.cs.values().fold(0, |sum, s| if s.k == *k { sum + 1 } else { sum } )
            },
            Kind::ARG | Kind::VAR => {
                self.ss.values().fold(0, |sum, s| if s.k == *k { sum + 1 } else { sum } )
            },
        }
    }

    pub fn get_mut(&mut self, i: &Ident) -> Option<&mut Symbol> {
        match self.ss.get_mut(i) {
            Some(s) => Some(s),
            None => self.cs.get_mut(i),
        }

    }

    pub fn get(&self, i: &Ident) -> Option<&Symbol> {
        self.ss.get(i).or_else( ||
        self.cs.get(i))
    }

    pub fn type_of(&self, i: &Ident) -> Option<&Type> {
        self.ss.get(i).and_then(|s| Some(&s.t)).or_else( ||
        self.cs.get(i).and_then(|s| Some(&s.t)))
    }

    pub fn kind_of(&self, i: &Ident) -> Option<&Kind> {
        self.ss.get(i).and_then(|s| Some(&s.k)).or_else( ||
        self.cs.get(i).and_then(|s| Some(&s.k)))
    }

    pub fn index_of(&self, i: &Ident) -> Option<i32> {
        self.ss.get(i).and_then(|s| Some(s.n)).or_else( ||
        self.cs.get(i).and_then(|s| Some(s.n)))
    }

    pub fn show(&self) {
        for (k, v) in &self.cs {
            println!("{:?}: \"{:?}\"", k, v);
        }
        for (k, v) in &self.ss {
            println!("{:?}: \"{:?}\"", k, v);
        }
    }
}

