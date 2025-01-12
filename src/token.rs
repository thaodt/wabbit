//! Token definitions for the Wabbit compiler
//!
//! Defines the complete set of tokens recognized by the lexer:
//! - Language keywords
//! - Operators and punctuation
//! - Literals (numbers, characters, booleans)
//! - Identifiers
//!
//! Each token includes its type and source location information.
use crate::location::Span;
use crate::opts_handle::{BinOpKind, CompOpKind, UnaryOpKind};

use std::fmt::Display;

/// define the possible kinds of tokens.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // primitive
    Name(String),
    Int(i32),
    Float(f64),
    Char(char),
    Bool(bool),

    // misc
    Semi,
    Comma,
    Assign,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // operators
    Not,
    Plus,
    Minus,
    Star,
    Slash,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    Equal,
    NotEqual,
    And,
    Or,

    // keywords
    Var,
    Const,
    Print,
    Break,
    Continue,
    If,
    Else,
    While,
    Func,
    Return,
}

/// A token is a single unit of code.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

/// implement formatter for Token to display it in a readable way.
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenKind::*;
        match self.kind {
            Name(ref s) => write!(f, "'{}'", s),
            Int(i) => write!(f, "'{}'", i),
            Float(fl) => write!(f, "'{:?}'", fl),
            Bool(b) => write!(f, "'{}'", b),
            Char(c) => write!(f, "'{}'", c),
            Semi => write!(f, "';'"),
            Comma => write!(f, "','"),
            Assign => write!(f, "'='"),
            LParen => write!(f, "'('"),
            RParen => write!(f, "')'"),
            LBrace => write!(f, "'{{'"),
            RBrace => write!(f, "'}}'"),
            Not => write!(f, "'!'"),
            Plus => write!(f, "'+'"),
            Minus => write!(f, "'-'"),
            Star => write!(f, "'*'"),
            Slash => write!(f, "'/'"),
            Less => write!(f, "'<'"),
            LessEqual => write!(f, "'<='"),
            Greater => write!(f, "'>'"),
            GreaterEqual => write!(f, "'>='"),
            Equal => write!(f, "'=='"),
            NotEqual => write!(f, "'!='"),
            And => write!(f, "'&&'"),
            Or => write!(f, "'||'"),
            Var => write!(f, "'var'"),
            Const => write!(f, "'const'"),
            Print => write!(f, "'print'"),
            Break => write!(f, "'break'"),
            Continue => write!(f, "'continue'"),
            If => write!(f, "'if'"),
            Else => write!(f, "'else'"),
            While => write!(f, "'while'"),
            Func => write!(f, "'func'"),
            Return => write!(f, "'return'"),
        }
    }
}

/// Apply newtype pattern over a [`Token`] just for the purpose of pretty printing the token stream.
pub struct DisplayToken(pub Token);

impl Display for DisplayToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let span = format!("{}", self.0.span);
        let kind = match self.0.kind {
            TokenKind::Name(ref s) => format!("{:10} {:?}", "Name", s),
            TokenKind::Int(i) => format!("{:10} {}", "Int", i),
            TokenKind::Float(fl) => format!("{:10} {:?}", "Float", fl),
            TokenKind::Bool(b) => format!("{:10} {}", "Bool", b),
            _ => format!("{:?}", self.0.kind),
        };

        write!(f, "{:15} {}", span, kind)
    }
}
/// Define the conversion from TokenKind to BinOpKind.
impl From<TokenKind> for BinOpKind {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Plus => Self::Add,
            TokenKind::Minus => Self::Sub,
            TokenKind::Star => Self::Mul,
            TokenKind::Slash => Self::Div,
            TokenKind::And => Self::And,
            TokenKind::Or => Self::Or,
            _ => panic!("Invalid token kind: {:?}", value),
        }
    }
}

/// Define the conversion from TokenKind to UnaryOpKind.
impl From<TokenKind> for UnaryOpKind {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Plus => Self::Pos,
            TokenKind::Minus => Self::Neg,
            TokenKind::Not => Self::Not,
            _ => panic!("Invalid token kind: {:?}", value),
        }
    }
}

/// Define the conversion from TokenKind to CompOpKind.
impl From<TokenKind> for CompOpKind {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::Less => Self::Lt,
            TokenKind::LessEqual => Self::Le,
            TokenKind::Greater => Self::Gt,
            TokenKind::GreaterEqual => Self::Ge,
            TokenKind::Equal => Self::Eq,
            TokenKind::NotEqual => Self::Ne,
            _ => panic!("Invalid token kind: {:?}", value),
        }
    }
}
