#![deny(dead_code)]
#![allow(unused)]

use lexer::{
    lexer::Lexer,
    tokens::Token
};
use parser::parser::{Parser, Statement, Expression};
use codegen::Codegen;
fn main() {
    let mut args = std::env::args().skip(1);
    let filepath: String = args.next().unwrap();
    if !filepath.ends_with(".real") {
        panic!("File must end with .real");
    }
    let source: String = std::fs::read_to_string(filepath.clone()).unwrap();

    let mut lexer: Lexer = Lexer::new(source.clone());
    let tokens: Vec<Token> = lexer.lex();

    let mut parser: Parser = Parser::new(tokens);
    let statements: Vec<Statement> = parser.parse();

    let mut codegen: Codegen = Codegen::new(filepath.clone().split('/').last().unwrap().to_string(), statements.clone());
    let cpp_code: String = codegen.codegen_cpp();
    let header_code: String = codegen.codegen_header();
    std::fs::write(filepath.clone().replace(".real", ".cpp"), cpp_code).unwrap();
    std::fs::write(filepath.clone().replace(".real", ".h"), header_code).unwrap();
}