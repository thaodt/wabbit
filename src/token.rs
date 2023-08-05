#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Minus,
    Plus,
    Mul,
    Divide,
    Comma,
    Semicolon,

    // One or two character tokens.
    Assign,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    NotEqual,
    LogicalAnd,
    LogicalOr,
    LogicalNot,

    // Identifiers.
    Identifier(String),

    // Literals.
    Integer,
    Float,
    Char,
    // Bool(bool),

    // Keywords.
    Break,
    Const,
    Continue,
    If,
    Else,
    Func,
    Print,
    Return,
    True,
    False,
    While,
    Var,

    // End of file
    Eof,
}
