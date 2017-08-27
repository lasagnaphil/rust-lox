use super::token::Token;
use super::token::Literal;

use std::fmt;

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>)
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            Binary(ref left, ref token, ref right) => write!(f, "({} {} {})", token.lexeme, left, right),
            Grouping(ref expression) => write!(f, "(group {})", expression),
            Literal(ref value) => value.fmt(f),
            Unary(ref operator, ref right) => write!(f, "({} {})", operator.lexeme, right),
        }
    }
}


