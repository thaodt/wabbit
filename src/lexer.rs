//! Lexical analyzer (tokenizer) for the Wabbit compiler
//!
//! This module handles the conversion of source text into a stream of tokens.
//! It implements a state machine that processes characters sequentially to recognize:
//! - Keywords and identifiers
//! - Numeric literals (integers and floats)
//! - Character literals
//! - Operators and punctuation
//! - Comments (single-line and multi-line)
//!
//! The main entry point is the `Lexer::tokenize()` function.

use crate::{
    error::{SyntaxError, TokenError},
    input::{ErrorContext, Input},
    location::{Loc, Span},
    token::{Token, TokenKind},
};

/// A lexer is a state machine that takes a string and converts it into a stream of tokens.
/// This struct describes the state of the lexer.
#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a Input<'a>,

    /// current position in the input, updated by [`next()`]
    pos: usize,

    /// current location in the input, updated by [`next()`]
    loc: Loc,

    /// starting location of the current token, updated by the lexer loop
    start_loc: Loc,

    /// current stream of token
    tokens: Vec<Token>,
}

/// type alias for the lexer result.
pub type Result<T> = std::result::Result<T, TokenError>;

/// Impls.
impl<'a> Lexer<'a> {
    /// Tokenize an input string
    pub fn tokenize(input: &'a Input<'a>) -> Result<Vec<Token>> {
        let mut lexer = Self::new(input);
        lexer.run()?;
        Ok(lexer.tokens)
    }

    /// Create a new lexer.
    fn new(input: &'a Input<'a>) -> Self {
        Self {
            input,
            pos: 0,
            loc: Loc::default(),
            start_loc: Loc::default(),
            tokens: Vec::new(),
        }
    }

    /// Push a token into the token stream.
    fn push(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            kind,
            span: Span::new(self.start_loc, self.loc),
        });
    }

    /// Return the next character in the input stream and update the current location.
    ///
    /// Returns `None` if the end of the input is reached.
    fn next(&mut self) -> Option<char> {
        let c = self.input.source.chars().nth(self.pos);
        if let Some(c) = c {
            self.pos += 1;
            if c == '\n' {
                self.loc.line += 1;
                self.loc.col = 0;
            } else {
                self.loc.col += 1;
            }
        }

        c
    }

    /// Return the next character in the input stream without updating the current location.
    ///
    /// Returns `None` if the end of the input is reached.
    fn peek(&self) -> Option<char> {
        self.input.source.chars().nth(self.pos)
    }

    /// Return the next character in the input stream if it matches `c` and update the current
    /// location.
    fn accept(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.next();
            true
        } else {
            false
        }
    }

    /// Build an [`TokenizerError`] from a [`SyntaxError`] and a [`Span`] and return it as a
    /// [`Result`].
    ///
    /// This function is intended as a shorthand for returning an error that will be displayed with
    /// suitable context of the user.
    fn err<T>(&self, err: SyntaxError) -> std::result::Result<T, TokenError> {
        let err = TokenError::SyntaxErr(
            Box::new(err),
            Box::new(ErrorContext::new(self.input, Span::new(self.loc, self.loc))),
        );

        Err(err)
    }

    /// Run the tokenizer on the input stream.
    fn run(&mut self) -> Result<()> {
        while let Some(c) = self.next() {
            self.start_loc = self.loc;

            match c {
                // whitespace
                c if c.is_whitespace() => continue,
                // integer/float
                c if c.is_ascii_digit() => {
                    let mut num = c.to_string();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_digit() {
                            num.push(c);
                            self.next();
                        } else {
                            break;
                        }
                    }
                    if let Some(c) = self.peek() {
                        if c == '.' {
                            num.push(c);
                            self.next();
                            while let Some(c) = self.peek() {
                                if c.is_ascii_digit() {
                                    num.push(c);
                                    self.next();
                                } else {
                                    break;
                                }
                            }
                            self.push(TokenKind::Float(num.parse().unwrap()));
                        } else {
                            self.push(TokenKind::Int(num.parse().unwrap()));
                        }
                    } else {
                        self.push(TokenKind::Int(num.parse().unwrap()));
                    }
                }
                // character literal
                '\'' => {
                    let character = match self.next() {
                        Some('\\') => match self.next() {
                            Some('n') => '\n',
                            Some('t') => '\t',
                            Some('r') => '\r',
                            Some('\\') => '\\',
                            Some('\'') => '\'',
                            Some(c) => {
                                return self.err(SyntaxError::UnexpectedChar(c));
                            }
                            None => return self.err(SyntaxError::UnexpectedEOF),
                        },
                        Some(c) if c != '\'' => c,
                        Some(c) => {
                            return self.err(SyntaxError::UnexpectedChar(c));
                        }
                        None => {
                            return self.err(SyntaxError::UnexpectedEOF);
                        }
                    };

                    // closing quote
                    match self.next() {
                        Some('\'') => (),
                        Some(c) => {
                            return self.err(SyntaxError::UnexpectedChar(c));
                        }
                        None => {
                            return self.err(SyntaxError::UnexpectedEOF);
                        }
                    }
                    self.push(TokenKind::Char(character));
                }
                // names/keywords
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let mut name = c.to_string();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_alphanumeric() || c == '_' {
                            name.push(c);
                            self.next();
                        } else {
                            break;
                        }
                    }
                    match name.as_str() {
                        // keywords
                        "var" => self.push(TokenKind::Var),
                        "const" => self.push(TokenKind::Const),
                        "print" => self.push(TokenKind::Print),
                        "break" => self.push(TokenKind::Break),
                        "continue" => self.push(TokenKind::Continue),
                        "if" => self.push(TokenKind::If),
                        "else" => self.push(TokenKind::Else),
                        "while" => self.push(TokenKind::While),
                        "func" => self.push(TokenKind::Func),
                        "return" => self.push(TokenKind::Return),
                        "true" => self.push(TokenKind::Bool(true)),
                        "false" => self.push(TokenKind::Bool(false)),
                        _ => self.push(TokenKind::Name(name)),
                    }
                }
                // misc
                ';' => self.push(TokenKind::Semi),
                ',' => self.push(TokenKind::Comma),
                '(' => self.push(TokenKind::LParen),
                ')' => self.push(TokenKind::RParen),
                '{' => self.push(TokenKind::LBrace),
                '}' => self.push(TokenKind::RBrace),
                '=' => {
                    if self.accept('=') {
                        self.push(TokenKind::Equal);
                    } else {
                        self.push(TokenKind::Assign);
                    }
                }
                '!' => {
                    if self.accept('=') {
                        self.push(TokenKind::NotEqual);
                    } else {
                        self.push(TokenKind::Not);
                    }
                }
                '+' => self.push(TokenKind::Plus),
                '-' => self.push(TokenKind::Minus),
                '*' => self.push(TokenKind::Star),
                '/' => {
                    if self.accept('/') {
                        while let Some(c) = self.next() {
                            if c == '\n' {
                                break;
                            }
                        }
                    } else if self.accept('*') {
                        while let Some(c) = self.next() {
                            if c == '*' && self.peek() == Some('/') {
                                self.next();
                                break;
                            }
                        }
                    } else {
                        self.push(TokenKind::Slash);
                    }
                }
                '<' => {
                    if self.accept('=') {
                        self.push(TokenKind::LessEqual);
                    } else {
                        self.push(TokenKind::Less);
                    }
                }
                '>' => {
                    if self.accept('=') {
                        self.push(TokenKind::GreaterEqual);
                    } else {
                        self.push(TokenKind::Greater);
                    }
                }
                '&' => {
                    if self.accept('&') {
                        self.push(TokenKind::And);
                    } else {
                        return self.err(SyntaxError::UnexpectedChar(c));
                    }
                }
                '|' => {
                    if self.accept('|') {
                        self.push(TokenKind::Or);
                    } else {
                        return self.err(SyntaxError::UnexpectedChar(c));
                    }
                }

                c => return self.err(SyntaxError::UnexpectedChar(c)),
            }
        }

        Ok(())
    }
}
