use std::collections::HashMap;

use lexer::tokens::TokenKind;
use parser::parser::{
    Statement,
    Expression,
    Type,
    AccessFlag,
    EnumVarient,
    GenericType,
    MatchCase
};

#[derive(Debug, Clone)] pub struct Codegen {
    pub filename: String,
    pub statements: Vec<Statement>,
    pub imports: Vec<String>,

    pub enums: Vec<String>,
    pub types: HashMap<String, Type>,

    pub current_class: Option<String>,
}

impl Codegen {
    pub fn new(filename: String, statements: Vec<Statement>) -> Codegen {
        Codegen {
            filename,
            statements,
            imports: Vec::new(),

            enums: Vec::new(),
            types: HashMap::new(),

            current_class: None,
        }
    }

    pub fn codegen_cpp(&mut self) -> String {
        let mut code = String::new();
        code.push_str(&format!("#include \"{}\"\n", self.filename.replace(".real", ".h")));
        let mut namespace_count = 0;
        if let Statement::Module(path, _) = self.statements[0].clone() {
            let path = self.get_module_path(path);
            let split: Vec<&str> = path.split(".").collect();
            for i in 0..split.len() {
                code.push_str(&format!("namespace {} {{\n", split[i]));
                namespace_count += 1;
            }
        }

        for statement in self.statements.clone() {
            code.push_str(&self.get_cpp_statement(statement));
        }

        for _ in 0..namespace_count {
            code.push_str("}\n");
        }
        code
    }

    pub fn codegen_header(&mut self) -> String {
        let mut code = String::new();
        code.push_str(&format!("#ifndef {}_H\n", self.filename.replace(".", "_").to_uppercase()));
        code.push_str(&format!("#define {}_H\n", self.filename.replace(".", "_").to_uppercase()));

        let mut imports: String = String::new();
        let mut namespaces: String = String::new();
        let mut usings: String = String::new();
        for statement in self.statements.clone() {
            match statement {
                Statement::Import(path, _) => {
                    let path = self.get_module_path(path);
                    let last: &str = path.split(".").last().unwrap();
                    imports.push_str(&format!("#include \"{}.h\"\n", last));
                    namespaces.push_str(&format!("using namespace {};\n", path.replace(".", "::")));
                }
                Statement::ImportExposing(path, exposing, _) => {
                    let path = self.get_module_path(path);
                    let last: &str = path.split(".").last().unwrap();
                    imports.push_str(&format!("#include \"{}.h\"\n", last));
                    namespaces.push_str(&format!("using namespace {};\n", path.replace(".", "::")));
                    for expose in exposing {
                        usings.push_str(&format!("using {}::{};\n", path.replace(".", "::"), expose));
                    }
                }
                _ => {}
            }
        }
        code.push_str(&imports);
        code.push_str(&namespaces);

        let mut namespace_count = 0;
        if let Statement::Module(path, _) = self.statements[0].clone() {
            let path = self.get_module_path(path);
            let split: Vec<&str> = path.split(".").collect();
            for i in 0..split.len() {
                code.push_str(&format!("namespace {} {{\n", split[i]));
                namespace_count += 1;
            }
        }
        code.push_str(&usings);

        for statement in self.statements.clone() {
            code.push_str(&self.get_header_statement(statement));
        }

        for _ in 0..namespace_count {
            code.push_str("}\n");
        }
        code.push_str(&format!("#endif // {}_H\n", self.filename.replace(".", "_").to_uppercase()));
        code
    }

    fn get_statement(&mut self, statement: Statement) -> String {
        match statement {
            _ => String::new()
        }
    }

    fn get_cpp_statement(&mut self, statement: Statement) -> String {
        let mut code: String = String::new();
        match statement {
            Statement::Object(name, _, _, members, _) => {
                self.current_class = Some(name.clone());
                for member in members {
                    code.push_str(&self.get_cpp_statement(member));
                }
                self.current_class = None;
            }
            Statement::GenericObject(name, _, generics, _, members, _) => {
                self.current_class = Some(name.clone());
                for member in members {
                    code.push_str(&self.get_cpp_statement(member));
                }
                self.current_class = None;
            }
            Statement::Procedure(name, _, args, return_type, expression, _) => {
                let mut args_string = String::new();
                for (name, t) in args.iter() {
                    self.types.insert(name.clone(), t.clone());
                    args_string.push_str(&format!("{} {}, ", self.get_type(t.clone()), name));
                }
                if args.len() > 0 {
                    args_string.pop();
                    args_string.pop();
                }
                code.push_str(&format!("{} {}{}({}) {{\n", self.get_type(return_type.clone()), if self.current_class.is_some() { format!("{}::", self.current_class.clone().unwrap()) } else { String::new() }, name, args_string));
                if let Expression::Match(_, _, _, _) = expression {
                    code.push_str(self.get_expression(expression).as_str());
                    code.push_str(";\n");
                } else if let Type::Unit(_) = return_type.clone() {
                    code.push_str(&format!("{};\n", self.get_expression(expression)));
                } else {
                    code.push_str(&format!("return {};\n", self.get_expression(expression)));
                }
                code.push_str("}\n");
                for (name, _) in args.iter() {
                    self.types.remove(name);
                }
            }
            Statement::GenericProcedure(name, _, generics, args, return_type, expression, _) => {
                let mut args_string = String::new();
                for (name, t) in args.iter() {
                    self.types.insert(name.clone(), t.clone());
                    args_string.push_str(&format!("{} {}, ", self.get_type(t.clone()), name));
                }
                if args.len() > 0 {
                    args_string.pop();
                    args_string.pop();
                }
                code.push_str(&format!("{} {}{}({}) {{\n", self.get_type(return_type.clone()), if self.current_class.is_some() { format!("{}::", self.current_class.clone().unwrap()) } else { String::new() }, name, args_string));
                if let Expression::Match(_, _, _, _) = expression {

                } else if let Type::Unit(_) = return_type.clone() {
                    code.push_str(&format!("{};\n", self.get_expression(expression)));
                } else {
                    code.push_str(&format!("return {};\n", self.get_expression(expression)));
                }
                code.push_str("}\n");
                for (name, _) in args.iter() {
                    self.types.remove(name);
                }
            }
            _ => {}
        }
        code
    }

    fn get_header_statement(&mut self, statement: Statement) -> String {
        let mut code = String::new();
        match statement {
            Statement::Alias(name, _, t, _) => {
                code.push_str(&format!("using {} = {};\n", name, self.get_type(t)));
            }
            Statement::GenericAlias(name, _, generics, t, _) => {
                let mut generics_string = String::new();
                for generic in generics {
                    match generic.1 {
                        GenericType::Extends => {
                            generics_string.push_str(&format!("typename {} = ", self.get_type(generic.0)));
                            let extends_type = self.get_type(generic.2.clone()[0].clone());
                            generics_string.push_str(&format!("{}, ", extends_type));
                        }
                        GenericType::Implements => panic!("Implementing generics is not supported yet"),
                        GenericType::None => {
                            generics_string.push_str(&format!("typename {}, ", self.get_type(generic.0)));
                        }
                    }
                }
                generics_string.pop();
                generics_string.pop();
                code.push_str(&format!("template <{}>\n", generics_string));
                code.push_str(&format!("using {} = {};\n", name, self.get_type(t)));
            }
            Statement::DataEnum(name, _, variants, _) => {
                self.enums.push(name.clone());
                code.push_str(&format!("namespace {}_Variants {{\n", name));
                let mut value_names = Vec::new();
                for variant in variants {
                    match variant {
                        EnumVarient::Tuple(name, types, _) => {
                            let mut args = String::new();
                            for (i, t) in types.iter().enumerate() {
                                args.push_str(&format!("{} __{}; ", self.get_type(t.clone()), i));
                            }
                            code.push_str(&format!("struct {} {{ {}}};\n", name, args));
                            value_names.push(name);
                        }
                        EnumVarient::Unit(name, _) => {
                            code.push_str(&format!("struct {} {{ }};\n", name));
                            value_names.push(name);
                        }
                    }
                }
                code.push_str("}\n");
                let names = value_names.iter().map(|x| format!("{}_Variants::{}", name, x)).collect::<Vec<String>>().join(", ");
                code.push_str(&format!("using {} = Enum<{}>;\n", name, names));
            }
            Statement::GenericDataEnum(name, _, generics, variants, _) => {
                self.enums.push(name.clone());
                code.push_str(&format!("namespace {}_Variants {{\n", name));
                let mut value_names = Vec::new();
                for variant in variants {
                    match variant {
                        EnumVarient::Tuple(name, types, _) => {
                            let mut args = String::new();
                            let mut generic_names = Vec::new();
                            for t in types.iter() {
                                match t {
                                    Type::Generic(generic, _) => {
                                        generic_names.push(generic.clone());
                                    }
                                    _ => {}
                                }
                            }
                            code.push_str(&format!("template <{}>\n", generic_names.iter().map(|x| format!("typename {}", x)).collect::<Vec<String>>().join(", ")));
                            for (i, t) in types.iter().enumerate() {
                                args.push_str(&format!("{} __{}; ", self.get_type(t.clone()), i));
                            }
                            code.push_str(&format!("struct {} {{ {}}};\n", name, args));
                            value_names.push(name);
                        }
                        EnumVarient::Unit(name, _) => {
                            code.push_str(&format!("struct {} {{ }};\n", name));
                            value_names.push(name);
                        }
                    }
                }
                code.push_str("}\n");
                let mut generics_string = String::new();
                for generic in generics {
                    match generic.1 {
                        GenericType::Extends => {
                            generics_string.push_str(&format!("typename {} = ", self.get_type(generic.0)));
                            let extends_type = self.get_type(generic.2.clone()[0].clone());
                            generics_string.push_str(&format!("{}, ", extends_type));
                        }
                        GenericType::Implements => panic!("Implementing generics is not supported yet"),
                        GenericType::None => {
                            generics_string.push_str(&format!("typename {}, ", self.get_type(generic.0)));
                        }
                    }
                }
                generics_string.pop();
                generics_string.pop();
                code.push_str(&format!("template <{}>\n", generics_string));
                let names = value_names.iter().map(|x| format!("{}_Variants::{}", name, x)).collect::<Vec<String>>().join(", ");
                code.push_str(&format!("using {} = Enum<{}>;\n", name, names));
            }
            Statement::DataStruct(name, _, fields, _) => {
                code.push_str(&format!("struct {} {{\n", name));
                for field in fields {
                    code.push_str(&format!("{} {};\n", self.get_type(field.1), field.0));
                }
                code.push_str("};\n");
            }
            Statement::GenericDataStruct(name, _, generics, fields, _) => {
                let mut generics_string = String::new();
                for generic in generics {
                    match generic.1 {
                        GenericType::Extends => {
                            generics_string.push_str(&format!("typename {} = ", self.get_type(generic.0)));
                            let extends_type = self.get_type(generic.2.clone()[0].clone());
                            generics_string.push_str(&format!("{}, ", extends_type));
                        }
                        GenericType::Implements => panic!("Implementing generics is not supported yet"),
                        GenericType::None => {
                            generics_string.push_str(&format!("typename {}, ", self.get_type(generic.0)));
                        }
                    }
                }
                generics_string.pop();
                generics_string.pop();
                code.push_str(&format!("template <{}>\n", generics_string));
                code.push_str(&format!("struct {} {{\n", name));
                for field in fields {
                    code.push_str(&format!("{} {};\n", self.get_type(field.1), field.0));
                }
                code.push_str("};\n");
            }
            Statement::Object(name, _, parent, members, _) => {
                self.current_class = Some(name.clone());
                code.push_str(&format!("class {} {}{{\n", name, if parent.is_some() { format!(": public {}", self.get_type(parent.unwrap())) } else { String::new() }));
                let mut public_members = Vec::new();
                let mut private_members = Vec::new();
                for member in members {
                    match member {
                        Statement::Procedure(_, ref flags, _, _, _, _) => {
                            if flags.contains(&AccessFlag::Public) {
                                public_members.push(member);
                            } else {
                                private_members.push(member);
                            }
                        }
                        _ => panic!("Invalid member"),
                    }
                }
                code.push_str("public:\n");
                code.push_str(&format!("{}() = default;\n", name));
                code.push_str(&format!("~{}() = default;\n", name));

                for member in public_members {
                    code.push_str(&self.get_header_statement(member));
                }

                if private_members.len() > 0 {
                    code.push_str("private:\n");
                    for member in private_members {
                        code.push_str(&self.get_header_statement(member));
                    }
                }

                code.push_str("};\n");
                self.current_class = None;
            }
            Statement::GenericObject(name, _, generics, parent, members, _) => {
                self.current_class = Some(name.clone());
                let mut generics_string = String::new();
                for generic in generics {
                    match generic.1 {
                        GenericType::Extends => {
                            generics_string.push_str(&format!("typename {} = ", self.get_type(generic.0)));
                            let extends_type = self.get_type(generic.2.clone()[0].clone());
                            generics_string.push_str(&format!("{}, ", extends_type));
                        }
                        GenericType::Implements => panic!("Implementing generics is not supported yet"),
                        GenericType::None => {
                            generics_string.push_str(&format!("typename {}, ", self.get_type(generic.0)));
                        }
                    }
                }
                generics_string.pop();
                generics_string.pop();
                code.push_str(&format!("template <{}>\n", generics_string));
                code.push_str(&format!("class {} {}{{\n", name, if parent.is_some() { format!(": public {}", self.get_type(parent.unwrap())) } else { String::new() }));
                let mut public_members = Vec::new();
                let mut private_members = Vec::new();
                for member in members {
                    match member {
                        Statement::Procedure(_, ref flags, _, _, _, _) => {
                            if flags.contains(&AccessFlag::Public) {
                                public_members.push(member);
                            } else {
                                private_members.push(member);
                            }
                        }
                        _ => panic!("Invalid member"),
                    }
                }
                code.push_str("public:\n");
                code.push_str(&format!("{}() = default;\n", name));
                code.push_str(&format!("~{}() = default;\n", name));

                for member in public_members {
                    code.push_str(&self.get_header_statement(member));
                }

                if private_members.len() > 0 {
                    code.push_str("private:\n");
                    for member in private_members {
                        code.push_str(&self.get_header_statement(member));
                    }
                }

                code.push_str("};\n");
                self.current_class = None;
            }
            Statement::Const(name, _, t, value, _) => {
                code.push_str(&format!("constexpr {} {} = {};\n", self.get_type(t), name, self.get_expression(value)));
            }
            Statement::Procedure(name, flags, args, return_type, _, _) => {
                if self.current_class.is_none() {
                    return String::new();
                }
                let mut args_string = String::new();
                for arg in args {
                    args_string.push_str(&format!("{} {}, ", self.get_type(arg.1), arg.0));
                }
                if args_string.len() > 0 {
                    args_string.pop();
                    args_string.pop();
                }
                if flags.contains(&AccessFlag::Virtual) {
                    code.push_str(&format!("virtual {} {}({}) = 0;\n", self.get_type(return_type), name, args_string));
                } else if flags.contains(&AccessFlag::Override) {
                    code.push_str(&format!("virtual {} {}({}) override = 0;\n", self.get_type(return_type), name, args_string));
                } else {
                    code.push_str(&format!("{} {}({});\n", self.get_type(return_type), name, args_string));
                }
            }
            Statement::GenericProcedure(name, flags, generics, args, return_type, _, _) => {
                let mut generics_string = String::new();
                for generic in generics {
                    match generic.1 {
                        GenericType::Extends => {
                            generics_string.push_str(&format!("typename {} = ", self.get_type(generic.0)));
                            let extends_type = self.get_type(generic.2.clone()[0].clone());
                            generics_string.push_str(&format!("{}, ", extends_type));
                        }
                        GenericType::Implements => panic!("Implementing generics is not supported yet"),
                        GenericType::None => {
                            generics_string.push_str(&format!("typename {}, ", self.get_type(generic.0)));
                        }
                    }
                }
                generics_string.pop();
                generics_string.pop();
                let mut args_string = String::new();
                for arg in args {
                    args_string.push_str(&format!("{} {}, ", self.get_type(arg.1), arg.0));
                }
                if args_string.len() > 0 {
                    args_string.pop();
                    args_string.pop();
                }
                if flags.contains(&AccessFlag::Virtual) {
                    code.push_str(&format!("virtual {} {}<{}>({}) = 0;\n", self.get_type(return_type), name, generics_string, args_string));
                } else if flags.contains(&AccessFlag::Override) {
                    code.push_str(&format!("virtual {} {}<{}>({}) override = 0;\n", self.get_type(return_type), name, generics_string, args_string));
                } else {
                    code.push_str(&format!("{} {}<{}>({});\n", self.get_type(return_type), name, generics_string, args_string));
                }
            }
            _ => {}
        }
        code
    }

    fn get_module_path(&self, expr: Expression) -> String {
        match expr {
            Expression::Member(expression, member, _) => {
                let mut module_path: String = self.get_module_path(*expression);
                module_path.push_str(".");
                module_path.push_str(&self.get_module_path(*member));
                module_path
            }
            Expression::Variable(id, _) => id,
            _ => panic!("Invalid module path"),
        }
    }

    fn get_type(&self, t: Type) -> String {
        match t {
            Type::Unit(_) => "void".to_string(),
            Type::Int(_) => "int".to_string(),
            Type::Bool(_) => "bool".to_string(),
            Type::GenericParameter(name, _) => name,
            Type::Generic(name, _) => name,
            Type::DataEnum(name, _) => name,
            Type::DataStruct(name, _) => name,
            Type::Alias(name, _) => name,
            Type::Object(name, _) => name,
            Type::Array(inner, _) => format!("List<{}>", self.get_type(*inner)),
            Type::Function(types, inner, _) => format!("Function<{}({})>", self.get_type(*inner), types.iter().map(|x| self.get_type(x.clone())).collect::<Vec<String>>().join(", ")),
            Type::Unknown(name, _) => name,
        }
    }

    fn get_expression(&mut self, expr: Expression) -> String {
        match expr {
            Expression::Variable(id, _) => id,
            Expression::Integer(value, _) => value.to_string(),
            Expression::String(value, _) => format!("\"{}\"", value),
            Expression::Member(expression, member, _) => {
                if let Expression::Variable(id, _) = *expression.clone() {
                    if let Expression::Call(callee, args, _) = *member.clone() {
                        if self.enums.contains(&id) {
                            let mut expr: String = format!("{} {{ {}_Variants::{} {{ ", id, id, callee);
                            expr.push_str(&args.iter().map(|x| self.get_expression(x.clone())).collect::<Vec<String>>().join(", "));
                            expr.push_str(" } }");
                            return expr;
                        }
                    }
                }
                let mut expr: String = self.get_expression(*expression);
                expr.push_str(".");
                expr.push_str(&self.get_expression(*member));
                expr
            }
            Expression::Call(name, args, _) => {
                let mut expr: String = name;
                expr.push_str("(");
                expr.push_str(&args.iter().map(|x| self.get_expression(x.clone())).collect::<Vec<String>>().join(", "));
                expr.push_str(")");
                expr
            }
            Expression::Match(expression, cases, else_case, _) => {
                let name: String = self.get_expression(*expression);
                let mut expr: String = format!("std::visit([this](auto&& {}) {{\n", name);
                expr.push_str(&format!("using T = std::decay_t<decltype({})>;\n", name));
                for case in cases {
                    let mut case_expr: String = String::new();
                    if let Expression::Call(callee, args, _) = case.condition[0].clone() {
                        let t: Type = self.types.get(&name).unwrap().clone();
                        let case_name: String = format!("{}_Variants::{}", self.get_type(t), callee);
                        case_expr.push_str(&format!("if constexpr (std::is_same_v<T, {}>) {{\n", case_name));
                        for (i, arg) in args.iter().enumerate() {
                            if let Expression::Variable(id, _) = arg {
                                case_expr.push_str(&format!("auto {} = {}.__{};\n", id, name, i));
                            } else {
                                panic!("Invalid match case");
                            }
                        }
                    } else if let Expression::Variable(id, _) = case.condition[0].clone() {
                        let t: Type = self.types.get(&name).unwrap().clone();
                        let case_name: String = format!("{}_Variants::{}", self.get_type(t), id);
                        case_expr.push_str(&format!("if constexpr (std::is_same_v<T, {}>) {{\n", case_name));
                    } else {
                        panic!("Invalid match case");
                    }
                    if case.body.len() == 1 {
                        if let Statement::Expression(expression, _) = case.body[0].clone() {
                            if let Expression::Call(_, _, _) = expression {
                                case_expr.push_str(&format!("{};\n", self.get_expression(expression)));
                                case_expr.push_str("return;\n");
                            } else {
                                case_expr.push_str(&format!("return {};\n", self.get_expression(expression)));
                            }
                        }
                    } else {
                        case_expr.push_str(&case.body.iter().map(|x| self.get_statement(x.clone())).collect::<Vec<String>>().join(""));
                    }
                    case_expr.push_str("}\n");
                    expr.push_str(&case_expr);
                }
                if let Some(else_case) = else_case {
                    if else_case.body.len() == 1 {
                        if let Statement::Expression(expression, _) = else_case.body[0].clone() {
                            expr.push_str(&format!("return {};\n", self.get_expression(expression)));
                        }
                    } else {
                        expr.push_str(&else_case.body.iter().map(|x| self.get_statement(x.clone())).collect::<Vec<String>>().join(""));
                    }
                }
                expr.push_str("}, ");
                expr.push_str(&name);
                expr.push_str(")");
                expr
            }
            Expression::Binary(left, right, op, _) => {
                let mut expr: String = self.get_expression(*left);
                expr.push_str(&format!(" {} ", match op {
                    TokenKind::Plus => "+",
                    TokenKind::Minus => "-",
                    TokenKind::Asterisk => "*",
                    TokenKind::Slash => "/",
                    TokenKind::Percent => "%",
                    _ => panic!("Invalid binary operator"),
                }));
                expr.push_str(&self.get_expression(*right));
                expr
            }
            _ => "".to_string(),
        }
    }
}