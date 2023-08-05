use crate::token::Token;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEYWORDS: Lazy<HashMap<String, Token>> = Lazy::new(|| {
    let mut keywords = HashMap::new();
    keywords.insert("break".to_string(), Token::Break);
    keywords.insert("const".to_string(), Token::Const);
    keywords.insert("continue".to_string(), Token::Continue);
    keywords.insert("if".to_string(), Token::If);
    keywords.insert("else".to_string(), Token::Else);
    keywords.insert("func".to_string(), Token::Func);
    keywords.insert("print".to_string(), Token::Print);
    keywords.insert("return".to_string(), Token::Return);
    keywords.insert("true".to_string(), Token::True);
    keywords.insert("false".to_string(), Token::False);
    keywords.insert("while".to_string(), Token::While);
    keywords.insert("var".to_string(), Token::Var);
    keywords
});

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    // start and current point to the start and end of the 'slice' of source
    // we're currently looking at.
    start: usize,
    current: usize,
    line: usize,
    //keywords: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::Eof);
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), LexerError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(Token::LeftParen),
            ')' => self.add_token(Token::RightParen),
            '{' => self.add_token(Token::LeftBrace),
            '}' => self.add_token(Token::RightBrace),
            '-' => self.add_token(Token::Minus),
            '+' => self.add_token(Token::Plus),
            '*' => self.add_token(Token::Mul),
            '/' => self.add_token(Token::Divide),
            ',' => self.add_token(Token::Comma),
            ';' => self.add_token(Token::Semicolon),
            '=' => {
                let token = if self.match_char('=') {
                    Token::EqualEqual
                } else {
                    Token::Assign
                };
                self.add_token(token);
            }
            '!' => {
                let token = if self.match_char('=') {
                    Token::NotEqual
                } else {
                    Token::LogicalNot
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.match_char('=') {
                    Token::GreaterEqual
                } else {
                    Token::Greater
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.match_char('=') {
                    Token::LessEqual
                } else {
                    Token::Less
                };
                self.add_token(token);
            }
            '&' => {
                if self.match_char('&') {
                    self.add_token(Token::LogicalAnd);
                } else {
                    return Err(LexerError::new(
                        "Unexpected character",
                        self.line,
                        self.current,
                    ));
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(Token::LogicalOr);
                } else {
                    return Err(LexerError::new(
                        "Unexpected character",
                        self.line,
                        self.current,
                    ));
                }
            }
            ' ' | '\t' | '\r' => {}
            '\'' => self.handle_char()?, // handle single quote character
            '\n' => self.line += 1,
            '0'..='9' => self.handle_number()?,
            'a'..='z' | 'A'..='Z' => self.handle_identifier()?,
            _ => {
                return Err(LexerError::new(
                    "Unexpected character",
                    self.line,
                    self.current,
                ))
            }
        }

        Ok(())
    }

    /*pub fn lex(&mut self) {
        // The lexer will repeatedly step through the source code one character at a time,
        // creating tokens as it goes.
        while !self.is_at_end() {
            self.start = self.current;
            self.tokenize_next();
        }
        // Once we're out of characters to read, we add an EOF token.
        self.tokens.push(Token::Eof);
    }

    fn tokenize_next(&mut self) {
        // Here is where you'd handle recognizing and creating the different tokens.
        // The implementation of this method will involve a lot of logic,
        // which I won't include in this skeleton.
        let identifier = self.identifier(); // reads an identifier
        let token = self
            .keywords
            .get(&identifier)
            .unwrap_or(&Token::Identifier(identifier)); // if it's a keyword, use the keyword token; if not, use an identifier token
        self.add_token(token.clone());
    }*/

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn handle_number(&mut self) -> Result<(), LexerError> {
        /*let mut is_float = false;

        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else if c == '.' && !is_float {
                is_float = true;
                self.advance();
            } else {
                break;
            }
        }

        let literal = self.source[self.start..self.current].to_string();
        let token = if is_float {
            let value = literal.parse::<f64>().unwrap();
            Token::Float(value)
        } else {
            let value = literal.parse::<i32>().unwrap();
            Token::Integer(value)
        };

        self.add_token(token);
        Ok(())*/
        let mut is_float = false;

        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                self.advance();
            } else if c == '.' && !is_float {
                is_float = true;
                self.advance();
            } else {
                break;
            }
        }

        let literal = self.source[self.start..self.current].to_string();
        let token = if is_float {
            Token::Float
        } else {
            Token::Integer
        };

        self.add_token(token);
        Ok(())
    }

    // handle single quote character in Wabbit
    fn handle_char(&mut self) -> Result<(), LexerError> {
        if self.is_at_end() {
            return Err(LexerError::new(
                "Unterminated character literal",
                self.line,
                self.current,
            ));
        }

        let c = self.advance();

        let c = if c == '\\' {
            self.advance(); // Consume the escape character
            match self.peek() {
                Some('n') => '\n',
                Some('t') => '\t',
                Some('\'') => '\'',
                Some('\\') => '\\',
                _ => {
                    return Err(LexerError::new(
                        "Invalid escape sequence",
                        self.line,
                        self.current,
                    ));
                }
            }
        } else {
            c
        };

        if self.peek() != Some('\'') {
            return Err(LexerError::new(
                "Unterminated character literal",
                self.line,
                self.current,
            ));
        }

        self.advance(); // Consume the closing single quote
        self.add_token(Token::Char);
        Ok(())
    }

    fn handle_identifier(&mut self) -> Result<(), LexerError> {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let identifier = self.source[self.start..self.current].to_string();
        let token = KEYWORDS
            .get(&identifier)
            .cloned()
            .unwrap_or(Token::Identifier(identifier));
        self.add_token(token);
        Ok(())
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }
}

pub struct LexerError {
    message: String,
    line: usize,
    column: usize,
}

impl LexerError {
    pub fn new(message: &str, line: usize, column: usize) -> Self {
        LexerError {
            message: message.to_string(),
            line,
            column,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}
