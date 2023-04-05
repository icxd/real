#![deny(dead_code)]
#![allow(unused)]

use parser::parser::Parser;

#[derive(Debug, Clone)]
pub struct TypeChecker {
    parser: Parser,
    current: usize,
}

impl TypeChecker {
    pub fn new(parser: Parser) -> Self {
        Self {
            parser,
            current: 0,
        }
    }
}