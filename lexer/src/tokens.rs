use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Identifier,         // foo, bar, baz, etc.
    IntegerLiteral,     // 1234
    StringLiteral,      // "Hello, World!"

    // Keywords
    //   Types
    Unit,               // Unit (void)
    Int,                // Int (32-bit signed integer)
    Char,               // Char
    Bool,               // Bool (boolean)

    //   Objects
    Data,               // data (struct or enum)
    Object,             // object (i think this is a class?)
    Alias,              // alias (kind of like a type alias)

    //   Variables
    Const,              // const (a contant variable declaration, can only be declared in the global context)

    //   Functions
    Procedure,          // procedure (basically a function)

    // Traits
    Trait,              // trait (define a trait)
    Of,                 // of (implement a trait)

    //   Module Handling
    Module,             // module (defines a module)
    Import,             // import (imports a module)
    Exposing,           // exposing
    
    //   Descriptors
    External,           // external (used to define c++ bindings for a module or object)
    Internal,           // internal (makes the object only accessible within the directory of the module)
    Public,             // public (makes the object public to other modules)
    Private,            // private (makes the object only accessible within the module)
    Virtual,            // virtual (makes the object virtual, meaning it can be overridden by a child class)
    Override,           // override (overrides a virtual object)
    Unsafe,             // unsafe (allows you to write unsafe code blocks (not really blocks but whatever))

    //   Control Flow
    Match,              // match (pattern matching)
    With,               // with (pattern matching)
    Else,               // else (else statement)

    //   I don't even know
    Cpp,                // cpp

    // Punctuation
    OpenParenthesis,    // (
    CloseParenthesis,   // )
    OpenBracket,        // [
    CloseBracket,       // ]
    OpenBrace,          // {
    CloseBrace,         // }
    Colon,              // :
    Semicolon,          // ;
    Dot,                // .
    Comma,              // ,
    Pipe,               // |
    QuestionMark,       // ?
    Arrow,              // ->

    // Operators
    Equal,              // =
    LessColon,          // <:
    GreaterColon,       // >:
    Plus,               // +
    Minus,              // -
    Asterisk,           // *
    Slash,              // /
    Percent,            // %

    // Special
    Newline,
    EndOfLine,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: Option<String>,
    pub span: Span
}