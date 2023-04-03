use std::collections::HashMap;

use lexer::{
    lexer::Lexer,
    tokens::Token, span::Span
};
use parser::parser::{Parser, Statement, Expression};
use vm::{
    ClassFile,
    Attribute,
    Constant,
    ConstantInfo,
    Field,
    Method
};
use codegen::Codegen;

fn hexdump(data: &[u8], width: usize, sep: char, offset: usize) -> String {
    let mut lines = Vec::new();
    for i in (0..data.len()).step_by(width) {
        let chunk = &data[i..std::cmp::min(i+width, data.len())];
        let hexa = chunk.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(&sep.to_string());
        let text = chunk.iter().map(|b| if 0x20 <= *b && *b < 0x7f { *b as char } else { '.' }).collect::<String>();
        lines.push(format!("{:08x}  {:<48}  {}", i+offset, hexa, text));
    }
    lines.join("\n")
}

fn get_module_path(expression: Expression) -> String {
    match expression {
        Expression::Member(expression, member, _) => {
            let mut module_path: String = get_module_path(*expression);
            module_path.push_str(".");
            module_path.push_str(&get_module_path(*member));
            module_path
        }
        Expression::Variable(id, _) => id,
        _ => panic!("Invalid module path"),
    }
}

#[derive(Debug, Clone)]
struct Constants {
    utf8: HashMap<String, u16>,
    class: HashMap<String, u16>,
    name_and_type: HashMap<(String, String), u16>,
    method_ref: HashMap<(String, String, String), u16>,
    field_ref: HashMap<(String, String, String), u16>,
}

impl Constants {
    fn new() -> Constants {
        Constants {
            utf8: HashMap::new(),
            class: HashMap::new(),
            name_and_type: HashMap::new(),
            method_ref: HashMap::new(),
            field_ref: HashMap::new(),
        }
    }
    fn add_utf8(&mut self, value: &str) -> u16 {
        if self.utf8.contains_key(value) {
            return *self.utf8.get(value).unwrap();
        }
        let index = self.utf8.len() as u16 + 1;
        self.utf8.insert(value.to_string(), index);
        index
    }
    fn add_class(&mut self, value: &str) -> u16 {
        if self.class.contains_key(value) {
            return *self.class.get(value).unwrap();
        }
        let index = self.class.len() as u16 + 1;
        self.class.insert(value.to_string(), index);
        index
    }
    fn add_name_and_type(&mut self, name: &str, descriptor: &str) -> u16 {
        if self.name_and_type.contains_key(&(name.to_string(), descriptor.to_string())) {
            return *self.name_and_type.get(&(name.to_string(), descriptor.to_string())).unwrap();
        }
        let index = self.name_and_type.len() as u16 + 1;
        self.name_and_type.insert((name.to_string(), descriptor.to_string()), index);
        index
    }
    fn add_method_ref(&mut self, class: &str, name: &str, descriptor: &str) -> u16 {
        if self.method_ref.contains_key(&(class.to_string(), name.to_string(), descriptor.to_string())) {
            return *self.method_ref.get(&(class.to_string(), name.to_string(), descriptor.to_string())).unwrap();
        }
        let index = self.method_ref.len() as u16 + 1;
        self.method_ref.insert((class.to_string(), name.to_string(), descriptor.to_string()), index);
        index
    }
    fn add_field_ref(&mut self, class: &str, name: &str, descriptor: &str) -> u16 {
        if self.field_ref.contains_key(&(class.to_string(), name.to_string(), descriptor.to_string())) {
            return *self.field_ref.get(&(class.to_string(), name.to_string(), descriptor.to_string())).unwrap();
        }
        let index = self.field_ref.len() as u16 + 1;
        self.field_ref.insert((class.to_string(), name.to_string(), descriptor.to_string()), index);
        index
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let filepath: String = args.next().unwrap();
    if !filepath.ends_with(".real") {
        panic!("File must end with .real");
    }
    let source: String = std::fs::read_to_string(filepath.clone()).unwrap();

    let mut lexer: Lexer = Lexer::new(source.clone());
    let tokens: Vec<Token> = lexer.lex();

    for token in tokens.clone() {
        let span: Span = token.span;
        println!("{:?} \"{}\"", token.kind, source[span.start..span.end].to_string());
    }

    let mut parser: Parser = Parser::new(tokens);
    let statements: Vec<Statement> = parser.parse();

    for statement in statements.clone() {
        println!("{:?}", statement);
    }

    let mut codegen: Codegen = Codegen::new(filepath.clone().split('/').last().unwrap().to_string(), statements.clone());
    let cpp_code: String = codegen.codegen_cpp();
    let header_code: String = codegen.codegen_header();
    std::fs::write(filepath.clone().replace(".real", ".cpp"), cpp_code).unwrap();
    std::fs::write(filepath.clone().replace(".real", ".h"), header_code).unwrap();

    // let mut module_path: String = String::new();
    // for statement in statements.clone() {
    //     match statement {
    //         Statement::Module(expression, _) => {
    //             module_path = get_module_path(expression);
    //         }
    //         _ => {}
    //     }
    // }

    // println!("Module path: {}", module_path);

    // const CLASS_ACC_PUBLIC: u16 = 0x0001;
    // const CLASS_ACC_FINAL: u16 = 0x0010;
    // const CLASS_ACC_SUPER: u16 = 0x0020;
    // const CLASS_ACC_INTERFACE: u16 = 0x0200;
    // const CLASS_ACC_ABSTRACT: u16 = 0x0400;
    // const CLASS_ACC_SYNTHETIC: u16 = 0x1000;
    // const CLASS_ACC_ANNOTATION: u16 = 0x2000;
    // const CLASS_ACC_ENUM: u16 = 0x4000;

    // let mut class_file: ClassFile = ClassFile {
    //     magic: 0xCAFEBABE,
    //     minor_version: 0,
    //     major_version: 52,
    //     constant_pool: vec![],
    //     access_flags: 0x00,
    //     this_class: 7,
    //     super_class: 2,
    //     interfaces: vec![],
    //     fields: vec![],
    //     methods: vec![],
    //     attributes: vec![],
    // };

    // class_file.add_method_ref(2, 3);
    // class_file.add_class(4);
    // class_file.add_name_and_type(5, 6);
    // class_file.add_utf8("java/lang/Object");
    // class_file.add_utf8("<init>");
    // class_file.add_utf8("()V");
    // class_file.add_class(8);
    // class_file.add_utf8(module_path.replace(".", "/").as_str());
    // class_file.add_utf8("Code");
    // class_file.add_utf8("LineNumberTable");
    // class_file.add_utf8("SourceFile");
    // class_file.add_utf8(filepath.as_str());

    // class_file.add_access_flags(CLASS_ACC_SUPER | CLASS_ACC_PUBLIC);

    // class_file.add_attribute(Attribute { attribute_name_index: 11, info: vec![0x00, 0x0c] });

    // class_file.add_method(Method {
    //     access_flags: CLASS_ACC_PUBLIC,
    //     name_index: 5,
    //     descriptor_index: 6,
    //     attributes: vec![Attribute { attribute_name_index: 9, info: vec![0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x05, 0x2a, 0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01] }]
    // });

    // println!("{}", hexdump(&class_file.to_bytes(), 16, ' ', 0));
    // std::fs::write(filepath.replace(".real", ".class"), class_file.to_bytes()).unwrap();
}