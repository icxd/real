use crate::{
    span::Span,
    tokens::{
        TokenKind,
        Token,
    },
};

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    end: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            current: 0,
            start: 0,
            end: 0,
        }
    }
    pub fn lex(&mut self) -> Vec<Token> {
        while self.current < self.source.len() {
            match self.current().clone() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                }
                '\n' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = start;
                    self.tokens.push(Token { kind: TokenKind::Newline, literal: None, span: Span { start, end: self.end }})
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut literal: String = String::new();
                    let start: usize = self.start;
                    while self.current().is_alphanumeric() || self.current() == '_' {
                        literal.push(self.current());
                        self.start += 1;
                        self.advance();
                    }
                    self.end = self.start;
                    let kind: TokenKind = match literal.clone().as_str() {
                        "Unit" => TokenKind::Unit,
                        "Int" => TokenKind::Int,
                        "Char" => TokenKind::Char,
                        "Bool" => TokenKind::Bool,
                        "data" => TokenKind::Data,
                        "alias" => TokenKind::Alias,
                        "object" => TokenKind::Object,
                        "const" => TokenKind::Const,
                        "procedure" => TokenKind::Procedure,
                        "trait" => TokenKind::Trait,
                        "of" => TokenKind::Of,
                        "module" => TokenKind::Module,
                        "import" => TokenKind::Import,
                        "exposing" => TokenKind::Exposing,
                        "external" => TokenKind::External,
                        "internal" => TokenKind::Internal,
                        "public" => TokenKind::Public,
                        "private" => TokenKind::Private,
                        "virtual" => TokenKind::Virtual,
                        "override" => TokenKind::Override,
                        "unsafe" => TokenKind::Unsafe,
                        "match" => TokenKind::Match,
                        "with" => TokenKind::With,
                        "else" => TokenKind::Else,
                        "cpp" => TokenKind::Cpp,
                        _ => TokenKind::Identifier,
                    };
                    self.tokens.push(Token { kind, literal: Some(literal), span: Span { start, end: self.end }})
                }
                '0'..='9' => {
                    let mut literal: String = String::new();
                    let start: usize = self.start;
                    while self.current().is_numeric() {
                        literal.push(self.current());
                        self.start += 1;
                        self.advance();
                    }
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::IntegerLiteral, literal: Some(literal), span: Span { start, end: self.end }})
                }
                '"' => {
                    let mut literal: String = String::new();
                    self.advance();
                    self.start += 1;
                    let start: usize = self.start;
                    while self.current() != '"' {
                        literal.push(self.current());
                        if self.current() == '\\' {
                            self.start += 1;
                            self.advance();
                            literal.push(self.current());
                        }
                        self.start += 1;
                        self.advance();
                    }
                    self.end = self.start;
                    self.advance();
                    self.start += 1;
                    self.tokens.push(Token { kind: TokenKind::StringLiteral, literal: Some(literal), span: Span { start, end: self.end }})
                }
                '(' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::OpenParenthesis, literal: None, span: Span { start, end: self.end }})
                }
                ')' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::CloseParenthesis, literal: None, span: Span { start, end: self.end }})
                }
                '[' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::OpenBracket, literal: None, span: Span { start, end: self.end }})
                }
                ']' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::CloseBracket, literal: None, span: Span { start, end: self.end }})
                }
                '{' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::OpenBrace, literal: None, span: Span { start, end: self.end }})
                }
                '}' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::CloseBrace, literal: None, span: Span { start, end: self.end }})
                }
                ':' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::Colon, literal: None, span: Span { start, end: self.end }})
                }
                ';' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::Semicolon, literal: None, span: Span { start, end: self.end }})
                }
                '.' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::Dot, literal: None, span: Span { start, end: self.end }})
                }
                ',' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::Comma, literal: None, span: Span { start, end: self.end }})
                }
                '|' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::Pipe, literal: None, span: Span { start, end: self.end }})
                }
                '?' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::QuestionMark, literal: None, span: Span { start, end: self.end }})
                }
                '=' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::Equal, literal: None, span: Span { start, end: self.end }})
                }
                '<' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == ':' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token { kind: TokenKind::LessColon, literal: None, span: Span { start, end: self.end }})
                    } else {
                        panic!("Unexpected token '{}'", self.current())
                    }
                }
                '>' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == ':' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token { kind: TokenKind::GreaterColon, literal: None, span: Span { start, end: self.end }})
                    } else {
                        panic!("Unexpected token '{}'", self.current())
                    }
                }
                '+' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::Plus, literal: None, span: Span { start, end: self.end }})
                }
                '-' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == '>' {
                        self.advance();
                        self.start += 1;
                        self.end = self.start;
                        self.tokens.push(Token { kind: TokenKind::Arrow, literal: None, span: Span { start, end: self.end }})
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token { kind: TokenKind::Minus, literal: None, span: Span { start, end: self.end }})
                    }
                }
                '*' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::Asterisk, literal: None, span: Span { start, end: self.end }})
                }
                '/' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    if self.current() == '/' {
                        self.advance();
                        self.start += 1;
                        while self.current < self.source.len() && self.current() != '\n' {
                            self.advance();
                            self.start += 1;
                        }
                    } else {
                        self.end = self.start;
                        self.tokens.push(Token { kind: TokenKind::Slash, literal: None, span: Span { start, end: self.end }})
                    }
                }
                '%' => {
                    let start: usize = self.start;
                    self.advance();
                    self.start += 1;
                    self.end = self.start;
                    self.tokens.push(Token { kind: TokenKind::Percent, literal: None, span: Span { start, end: self.end }})
                }
                _ => panic!("Unexpected token '{}'", self.current())
            }
        }
        self.tokens.clone()
    }

    fn current(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }
}