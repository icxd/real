use std::collections::HashMap;
use lexer::{
    span::Span,
    tokens::{Token, TokenKind}
};

#[derive(Debug, Clone)] pub struct MatchCase {
    pub condition: Vec<Expression>,
    pub body: Vec<Statement>,
    pub span: Span,
}
#[derive(Debug, Clone)] pub struct Parser {
    tokens: Vec<Token>,
    statements: Vec<Statement>,
    current: usize,

    current_generic_parameters: Vec<Type>,

    data_enums: HashMap<String, Vec<EnumVarient>>,
    data_structs: HashMap<String, Vec<(String, Type)>>,
    aliases: HashMap<String, Type>,
    objects: HashMap<String, (Vec<Type>, Vec<Statement>)>,
    procedures: HashMap<String, (Vec<(String, Type)>, Type, Expression)>,
    consts: HashMap<String, (Type, Expression)>,

    generic_data_enums: HashMap<String, Vec<(Type, GenericType, Vec<Type>)>>,
    generic_data_structs: HashMap<String, Vec<(Type, GenericType, Vec<Type>)>>,
    generic_aliases: HashMap<String, Vec<(Type, GenericType, Vec<Type>)>>,
    generic_objects: HashMap<String, Vec<(Type, GenericType, Vec<Type>)>>,
    generic_procedures: HashMap<String, Vec<(Type, GenericType, Vec<Type>)>>,
}

#[derive(Debug, Clone, PartialEq)] pub enum EnumVarient {
    Unit(String, Span),
    Tuple(String, Vec<Type>, Span),
}
#[derive(Debug, Clone, PartialEq)] pub enum GenericType {
    // I haven't figured out what the difference between them is yet.
    Implements,
    Extends,
    None,
}
#[derive(Debug, Clone, PartialEq)] pub enum AccessFlag {
    External,
    Internal,
    Public,
    Private,
    Virtual,
    Override,
}
#[derive(Debug, Clone, PartialEq)] pub enum Type {
    Unit(Span),
    Int(Span),
    Char(Span),
    Bool(Span),

    GenericParameter(String, Span),
    Generic(String, Span),

    DataEnum(String, Span),
    DataStruct(String, Span),
    Alias(String, Span),
    Object(String, Span),

    Array(Box<Type>, Span),

    Function(Vec<Type>, Box<Type>, Span),

    Unknown(String, Span),
}
#[derive(Debug, Clone)] pub enum Statement {
    DataEnum(String, Vec<AccessFlag>, Vec<EnumVarient>, Span),
    DataStruct(String, Vec<AccessFlag>, Vec<(String, Type)>, Span),
    Alias(String, Vec<AccessFlag>, Type, Span),
    Object(String, Vec<AccessFlag>, Vec<Type>, Vec<(String, Type)>, Vec<Statement>, Span),
    Procedure(String, Vec<AccessFlag>, Vec<(String, Type)>, Type, Expression, Span),
    Const(String, Vec<AccessFlag>, Type, Expression, Span),
    Trait(String, Vec<AccessFlag>, Span),
    
    GenericDataEnum(String, Vec<AccessFlag>, Vec<(Type, GenericType, Vec<Type>)>, Vec<EnumVarient>, Span),
    GenericDataStruct(String, Vec<AccessFlag>, Vec<(Type, GenericType, Vec<Type>)>, Vec<(String, Type)>, Span),
    GenericAlias(String, Vec<AccessFlag>, Vec<(Type, GenericType, Vec<Type>)>, Type, Span),
    GenericObject(String, Vec<AccessFlag>, Vec<(Type, GenericType, Vec<Type>)>, Vec<Type>, Vec<(String, Type)>, Vec<Statement>, Span),
    GenericProcedure(String, Vec<AccessFlag>, Vec<(Type, GenericType, Vec<Type>)>, Vec<(String, Type)>, Type, Expression, Span),
    GenericTrait(String, Vec<AccessFlag>, Vec<(Type, GenericType, Vec<Type>)>, Span),

    Module(Expression, Span),
    Import(Expression, Span),
    ImportExposing(Expression, Vec<String>, Span),

    Of(Type, Span),

    Expression(Expression, Span),
}
#[derive(Debug, Clone)] pub enum Expression {
    Member(Box<Expression>, Box<Expression>, Span),
    Call(String, Vec<Expression>, Span),
    Variable(String, Span),
    String(String, Span),
    Integer(i64, Span),
    Match(Box<Expression>, Vec<MatchCase>, Option<MatchCase>, Span),
    Binary(Box<Expression>, Box<Expression>, TokenKind, Span),
    Unsafe(Box<Expression>, Span),
    Cpp(String, Span),
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            statements: vec![],
            current: 0,

            current_generic_parameters: vec![],

            data_enums: HashMap::new(),
            data_structs: HashMap::new(),
            aliases: HashMap::new(),
            objects: HashMap::new(),
            procedures: HashMap::new(),
            consts: HashMap::new(),

            generic_data_enums: HashMap::new(),
            generic_data_structs: HashMap::new(),
            generic_aliases: HashMap::new(),
            generic_objects: HashMap::new(),
            generic_procedures: HashMap::new(),
        }
    }
    pub fn parse(&mut self) -> Vec<Statement> {
        while self.current < self.tokens.len() {
            if self.current().kind == TokenKind::Newline {
                self.advance();
                continue;
            }
            let statement: Statement = self.parse_statement();
            self.statements.push(statement);
        }
        self.statements.clone()
    }

    fn parse_statement(&mut self) -> Statement {
        let access_flags: Vec<AccessFlag> = self.parse_access_flags();
        match self.current().kind {
            TokenKind::Data => self.parse_data(access_flags),
            TokenKind::Alias => self.parse_alias(access_flags),
            TokenKind::Object => self.parse_object(access_flags),
            TokenKind::Const => self.parse_const(access_flags),
            TokenKind::Procedure => self.parse_procedure(access_flags),
            TokenKind::Module => self.parse_module(),
            TokenKind::Import => self.parse_import(),
            _ => {
                let span: Span = self.current().span;
                let expression: Expression = self.parse_expression();
                Statement::Expression(expression, span)
            }
        }
    }
    fn parse_data(&mut self, flags: Vec<AccessFlag>) -> Statement {
        self.expect(TokenKind::Data);
        let name_span: Span = self.current().span;
        let identifier: String = self.expect(TokenKind::Identifier).literal.unwrap();
        let mut generic_parameters: Vec<(Type, GenericType, Vec<Type>)> = vec![];
        if self.current().kind == TokenKind::OpenBracket {
            self.expect(TokenKind::OpenBracket);
            while self.current < self.tokens.len() && self.current().kind != TokenKind::CloseBracket {
                let t: Type = self.parse_type();
                let generic_type: GenericType = match self.current().kind {
                    TokenKind::LessColon => {
                        self.expect(TokenKind::LessColon);
                        GenericType::Implements
                    },
                    TokenKind::GreaterColon => {
                        self.expect(TokenKind::GreaterColon);
                        GenericType::Extends
                    },
                    _ => GenericType::None
                };
                let mut types: Vec<Type> = vec![];
                if generic_type != GenericType::None {
                    types.push(self.parse_type());
                    while self.current().kind == TokenKind::Pipe {
                        self.expect(TokenKind::Pipe);
                        types.push(self.parse_type());
                    }
                }
                generic_parameters.push((match t {
                    Type::Unknown(id, span) => Type::GenericParameter(id, span),
                    _ => panic!("Generic parameters must be unknown types")
                }, generic_type, types));
                if self.current().kind == TokenKind::Comma {
                    self.expect(TokenKind::Comma);
                }
            }
            self.expect(TokenKind::CloseBracket);
        }
        if self.current().kind == TokenKind::OpenParenthesis {
            self.expect(TokenKind::OpenParenthesis);
            for (generic_type, _, _) in generic_parameters.clone() {
                self.current_generic_parameters.push(generic_type);
            }
            let mut parameters: Vec<(String, Type)> = vec![];
            while self.current < self.tokens.len() && self.current().kind != TokenKind::CloseParenthesis {
                let name: String = self.expect(TokenKind::Identifier).literal.unwrap();
                self.expect(TokenKind::Colon);
                let t: Type = self.parse_type();
                parameters.push((name, t));
                if self.current().kind == TokenKind::Comma {
                    self.expect(TokenKind::Comma);
                }
            }
            self.expect(TokenKind::CloseParenthesis);
            self.expect(TokenKind::Newline);
            for _ in 0..generic_parameters.len() {
                self.current_generic_parameters.pop();
            }
            if generic_parameters.len() > 0 {
                self.data_structs.insert(identifier.clone(), parameters.clone());
                self.generic_data_structs.insert(identifier.clone(), generic_parameters.clone());
                Statement::GenericDataStruct(identifier, flags, generic_parameters, parameters, name_span)
            } else {
                self.data_structs.insert(identifier.clone(), parameters.clone());
                Statement::DataStruct(identifier, flags, parameters, name_span)
            }
        } else {
            self.expect(TokenKind::Equal);
            self.expect(TokenKind::Newline);
            for (generic_type, _, _) in generic_parameters.clone() {
                self.current_generic_parameters.push(generic_type);
            }
            let mut variants: Vec<EnumVarient> = vec![];
            while self.current < self.tokens.len() && self.current().kind == TokenKind::Pipe {
                self.expect(TokenKind::Pipe);
                let variant: EnumVarient = self.parse_enum_variant();
                variants.push(variant);
                self.expect(TokenKind::Newline);
            }
            for _ in 0..generic_parameters.len() {
                self.current_generic_parameters.pop();
            }
            if generic_parameters.len() > 0 {
                self.data_enums.insert(identifier.clone(), variants.clone());
                self.generic_data_enums.insert(identifier.clone(), generic_parameters.clone());
                Statement::GenericDataEnum(identifier, flags, generic_parameters, variants, name_span)
            } else {
                self.data_enums.insert(identifier.clone(), variants.clone());
                Statement::DataEnum(identifier, flags, variants, name_span)
            }
        }
    }
    fn parse_alias(&mut self, flags: Vec<AccessFlag>) -> Statement {
        self.expect(TokenKind::Alias);
        let name_span: Span = self.current().span;
        let identifier: String = self.expect(TokenKind::Identifier).literal.unwrap();
        let mut generic_parameters: Vec<(Type, GenericType, Vec<Type>)> = vec![];
        if self.current().kind == TokenKind::OpenBracket {
            self.expect(TokenKind::OpenBracket);
            while self.current < self.tokens.len() && self.current().kind != TokenKind::CloseBracket {
                let t: Type = self.parse_type();
                let generic_type: GenericType = match self.current().kind {
                    TokenKind::LessColon => {
                        self.expect(TokenKind::LessColon);
                        GenericType::Implements
                    },
                    TokenKind::GreaterColon => {
                        self.expect(TokenKind::GreaterColon);
                        GenericType::Extends
                    },
                    _ => GenericType::None
                };
                let mut types: Vec<Type> = vec![];
                if generic_type != GenericType::None {
                    types.push(self.parse_type());
                    while self.current().kind == TokenKind::Pipe {
                        self.expect(TokenKind::Pipe);
                        types.push(self.parse_type());
                    }
                }
                generic_parameters.push((match t {
                    Type::Unknown(id, span) => Type::GenericParameter(id, span),
                    _ => panic!("Generic parameters must be unknown types")
                }, generic_type, types));
                if self.current().kind == TokenKind::Comma {
                    self.expect(TokenKind::Comma);
                }
            }
            self.expect(TokenKind::CloseBracket);
        }
        for (generic_type, _, _) in generic_parameters.clone() {
            self.current_generic_parameters.push(generic_type);
        }
        self.expect(TokenKind::Equal);
        let t: Type = self.parse_type();
        self.expect(TokenKind::Newline);
        for _ in 0..generic_parameters.len() {
            self.current_generic_parameters.pop();
        }
        if generic_parameters.len() > 0 {
            self.aliases.insert(identifier.clone(), t.clone());
            self.generic_aliases.insert(identifier.clone(), generic_parameters.clone());
            Statement::GenericAlias(identifier, flags, generic_parameters, t, name_span)
        } else {
            self.aliases.insert(identifier.clone(), t.clone());
            Statement::Alias(identifier, flags, t, name_span)
        }
    }
    fn parse_object(&mut self, flags: Vec<AccessFlag>) -> Statement {
        self.expect(TokenKind::Object);
        let name_span: Span = self.current().span;
        let identifier: String = self.expect(TokenKind::Identifier).literal.unwrap();
        let mut generic_parameters: Vec<(Type, GenericType, Vec<Type>)> = vec![];
        if self.current().kind == TokenKind::OpenBracket {
            self.expect(TokenKind::OpenBracket);
            while self.current < self.tokens.len() && self.current().kind != TokenKind::CloseBracket {
                let t: Type = self.parse_type();
                let generic_type: GenericType = match self.current().kind {
                    TokenKind::LessColon => {
                        self.expect(TokenKind::LessColon);
                        GenericType::Implements
                    },
                    TokenKind::GreaterColon => {
                        self.expect(TokenKind::GreaterColon);
                        GenericType::Extends
                    },
                    _ => GenericType::None
                };
                let mut types: Vec<Type> = vec![];
                if generic_type != GenericType::None {
                    types.push(self.parse_type());
                    while self.current().kind == TokenKind::Pipe {
                        self.expect(TokenKind::Pipe);
                        types.push(self.parse_type());
                    }
                }
                generic_parameters.push((match t {
                    Type::Unknown(id, span) => Type::GenericParameter(id, span),
                    _ => panic!("Generic parameters must be unknown types")
                }, generic_type, types));
                if self.current().kind == TokenKind::Comma {
                    self.expect(TokenKind::Comma);
                }
            }
            self.expect(TokenKind::CloseBracket);
        }
        let mut parameters: Vec<(String, Type)> = vec![];
        if self.current().kind == TokenKind::OpenParenthesis {
            self.expect(TokenKind::OpenParenthesis);
            while self.current().kind != TokenKind::CloseParenthesis {
                let n: String = self.expect(TokenKind::Identifier).literal.unwrap();
                self.expect(TokenKind::Colon);
                let t: Type = self.parse_type();
                parameters.push((n, t));
                if self.current().kind == TokenKind::Comma {
                    self.expect(TokenKind::Comma);
                }
            }
            self.expect(TokenKind::CloseParenthesis);
        }
        let mut parents: Vec<Type> = vec![];
        if self.current().kind == TokenKind::Colon {
            self.expect(TokenKind::Colon);
            while self.current().kind != TokenKind::Newline {
                parents.push(self.parse_type());
                if self.current().kind == TokenKind::Comma {
                    self.expect(TokenKind::Comma);
                }
            }
        }
        if self.current().kind == TokenKind::Equal {
            self.expect(TokenKind::Equal);
            self.expect(TokenKind::Newline);
            let mut statements: Vec<Statement> = vec![];
            while self.current().kind == TokenKind::Pipe {
                self.expect(TokenKind::Pipe);
                let access_flags: Vec<AccessFlag> = self.parse_access_flags();
                statements.push(self.parse_procedure(access_flags));
            }
            if self.current().kind == TokenKind::Newline {
                self.expect(TokenKind::Newline);
            }
            if generic_parameters.len() > 0 {
                self.objects.insert(identifier.clone(), (parents.clone(), statements.clone()));
                self.generic_objects.insert(identifier.clone(), generic_parameters.clone());
                Statement::GenericObject(identifier, flags, generic_parameters, parents, parameters, statements, name_span)
            } else {
                self.objects.insert(identifier.clone(), (parents.clone(), statements.clone()));
                Statement::Object(identifier, flags, parents, parameters, statements, name_span)
            }
        } else {
            self.expect(TokenKind::Newline);
            if generic_parameters.len() > 0 {
                self.objects.insert(identifier.clone(), (parents.clone(), vec![]));
                self.generic_objects.insert(identifier.clone(), generic_parameters.clone());
                Statement::GenericObject(identifier, flags, generic_parameters, parents, parameters, vec![], name_span)
            } else {
                self.objects.insert(identifier.clone(), (parents.clone(), vec![]));
                Statement::Object(identifier, flags, parents, parameters, vec![], name_span)
            }
        }
    }
    fn parse_const(&mut self, flags: Vec<AccessFlag>) -> Statement {
        self.expect(TokenKind::Const);
        let name_span: Span = self.current().span;
        let identifier: String = self.expect(TokenKind::Identifier).literal.unwrap();
        self.expect(TokenKind::Colon);
        let t: Type = self.parse_type();
        self.expect(TokenKind::Equal);
        let expr: Expression = self.parse_expression();
        self.expect(TokenKind::Newline);
        self.consts.insert(identifier.clone(), (t.clone(), expr.clone()));
        Statement::Const(identifier, flags, t, expr, name_span)
    }
    fn parse_procedure(&mut self, flags: Vec<AccessFlag>) -> Statement {
        self.expect(TokenKind::Procedure);
        let name_span: Span = self.current().span;
        let identifier: String = self.expect(TokenKind::Identifier).literal.unwrap();
        let mut generic_parameters: Vec<(Type, GenericType, Vec<Type>)> = vec![];
        if self.current().kind == TokenKind::OpenBracket {
            self.expect(TokenKind::OpenBracket);
            while self.current < self.tokens.len() && self.current().kind != TokenKind::CloseBracket {
                let t: Type = self.parse_type();
                let generic_type: GenericType = match self.current().kind {
                    TokenKind::LessColon => {
                        self.expect(TokenKind::LessColon);
                        GenericType::Implements
                    },
                    TokenKind::GreaterColon => {
                        self.expect(TokenKind::GreaterColon);
                        GenericType::Extends
                    },
                    _ => GenericType::None
                };
                let mut types: Vec<Type> = vec![];
                if generic_type != GenericType::None {
                    types.push(self.parse_type());
                    while self.current().kind == TokenKind::Pipe {
                        self.expect(TokenKind::Pipe);
                        types.push(self.parse_type());
                    }
                }
                generic_parameters.push((match t {
                    Type::Unknown(id, span) => Type::GenericParameter(id, span),
                    _ => panic!("Generic parameters must be unknown types")
                }, generic_type, types));
                if self.current().kind == TokenKind::Comma {
                    self.expect(TokenKind::Comma);
                }
            }
            self.expect(TokenKind::CloseBracket);
        }
        for (generic_type, _, _) in generic_parameters.clone() {
            self.current_generic_parameters.push(generic_type);
        }
        self.expect(TokenKind::OpenParenthesis);
        let mut parameters: Vec<(String, Type)> = vec![];
        while self.current < self.tokens.len() && self.current().kind != TokenKind::CloseParenthesis {
            let name: String = self.expect(TokenKind::Identifier).literal.unwrap();
            self.expect(TokenKind::Colon);
            let t: Type = self.parse_type();
            parameters.push((name, t));
            if self.current().kind == TokenKind::Comma {
                self.expect(TokenKind::Comma);
            }
        }
        self.expect(TokenKind::CloseParenthesis);
        self.expect(TokenKind::Arrow);
        let return_type: Type = self.parse_type();
        for _ in 0..generic_parameters.len() {
            self.current_generic_parameters.pop();
        }
        if self.current().kind == TokenKind::Equal {
            self.expect(TokenKind::Equal);
            if self.current().kind == TokenKind::Newline {
                self.expect(TokenKind::Newline);
            }
            let expr: Expression = self.parse_expression();
            self.expect(TokenKind::Newline);
            if generic_parameters.len() > 0 {
                self.procedures.insert(identifier.clone(), (parameters.clone(), return_type.clone(), expr.clone()));
                self.generic_procedures.insert(identifier.clone(), generic_parameters.clone());
                Statement::GenericProcedure(identifier, flags, generic_parameters, parameters, return_type, expr, name_span)
            } else {
                self.procedures.insert(identifier.clone(), (parameters.clone(), return_type.clone(), expr.clone()));
                Statement::Procedure(identifier, flags, parameters, return_type, expr, name_span)
            }
        } else {
            panic!("Expected '=', block procedures are not yet supported");
        }
    }
    fn parse_module(&mut self) -> Statement {
        self.expect(TokenKind::Module);
        let span: Span = self.current().span;
        let expr: Expression = self.parse_expression();
        self.expect(TokenKind::Newline);
        Statement::Module(expr, span)
    }
    fn parse_import(&mut self) -> Statement {
        self.expect(TokenKind::Import);
        let span: Span = self.current().span;
        let expr: Expression = self.parse_expression();
        let mut exposed: Vec<String> = vec![];
        if self.current().kind == TokenKind::Exposing {
            self.expect(TokenKind::Exposing);
            if self.current().kind == TokenKind::OpenParenthesis {
                self.expect(TokenKind::OpenParenthesis);
                while self.current().kind != TokenKind::CloseParenthesis {
                    if self.current().kind == TokenKind::Newline {
                        self.expect(TokenKind::Newline);
                    }
                    exposed.push(self.expect(TokenKind::Identifier).literal.unwrap());
                    if self.current().kind == TokenKind::Comma {
                        self.expect(TokenKind::Comma);
                    }
                    if self.current().kind == TokenKind::Newline {
                        self.expect(TokenKind::Newline);
                    }
                }
                self.expect(TokenKind::CloseParenthesis);
            } else {
                exposed.push(self.expect(TokenKind::Identifier).literal.unwrap());
            }
        }
        self.expect(TokenKind::Newline);
        if exposed.len() > 0 {
            Statement::ImportExposing(expr, exposed, span)
        } else {
            Statement::Import(expr, span)
        }
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_addtitive()
    }
    fn parse_addtitive(&mut self) -> Expression {
        let mut expr: Expression = self.parse_multiplicative();
        while self.current().kind == TokenKind::Plus || self.current().kind == TokenKind::Minus {
            let span: Span = self.current().span;
            let op: TokenKind = self.current().kind;
            self.advance();
            let right: Expression = self.parse_expression();
            expr = Expression::Binary(Box::new(expr), Box::new(right), op, span);
        }
        expr
    }
    fn parse_multiplicative(&mut self) -> Expression {
        let mut expr: Expression = self.parse_call();
        while self.current().kind == TokenKind::Asterisk || self.current().kind == TokenKind::Slash {
            let span: Span = self.current().span;
            let op: TokenKind = self.current().kind;
            self.advance();
            let right: Expression = self.parse_expression();
            expr = Expression::Binary(Box::new(expr), Box::new(right), op, span);
        }
        expr
    }
    fn parse_call(&mut self) -> Expression {
        let span: Span = self.current().span;
        let mut expr: Expression = self.parse_member();
        while self.current().kind == TokenKind::OpenParenthesis {
            self.expect(TokenKind::OpenParenthesis);
            let mut args: Vec<Expression> = vec![];
            while self.current().kind != TokenKind::CloseParenthesis {
                args.push(self.parse_expression());
                if self.current().kind == TokenKind::Comma {
                    self.expect(TokenKind::Comma);
                }
            }
            self.expect(TokenKind::CloseParenthesis);
            expr = Expression::Call(match expr {
                Expression::Variable(name, _) => name,
                _ => panic!("unexpected expression: {:?}", expr)
            }, args, span.clone());
        }
        expr
    }
    fn parse_member(&mut self) -> Expression {
        let mut expr: Expression = self.parse_primary();
        while self.current().kind == TokenKind::Dot {
            self.expect(TokenKind::Dot);
            let span: Span = self.current().span;
            expr = Expression::Member(Box::new(expr), Box::new(self.parse_expression()), span);
        }
        expr
    }
    fn parse_primary(&mut self) -> Expression {
        let span: Span = self.current().span;
        match self.current().kind {
            TokenKind::Identifier => {
                let identifier: String = self.expect(TokenKind::Identifier).literal.unwrap();
                Expression::Variable(identifier, span)
            }
            TokenKind::StringLiteral => {
                let string: String = self.expect(TokenKind::StringLiteral).literal.unwrap();
                Expression::String(string, span)
            }
            TokenKind::IntegerLiteral => {
                let integer: i64 = self.expect(TokenKind::IntegerLiteral).literal.unwrap().parse::<i64>().unwrap();
                Expression::Integer(integer, span)
            }
            TokenKind::Match => {
                self.expect(TokenKind::Match);
                let expr: Expression = self.parse_expression();
                self.expect(TokenKind::With);
                self.expect(TokenKind::Newline);
                let mut cases: Vec<MatchCase> = vec![];
                let mut else_case: Option<MatchCase> = None;
                while self.current().kind == TokenKind::Pipe {
                    self.expect(TokenKind::Pipe);
                    if self.current().kind == TokenKind::Else {
                        self.expect(TokenKind::Else);
                        self.expect(TokenKind::Arrow);
                        let mut body: Vec<Statement> = vec![];
                        if self.current().kind == TokenKind::OpenBrace {
                            self.expect(TokenKind::OpenBrace);
                            while self.current().kind != TokenKind::CloseBrace {
                                body.push(self.parse_statement());
                            }
                            self.expect(TokenKind::CloseBrace);
                        } else {
                            body.push(Statement::Expression(self.parse_expression(), span.clone()));
                            if self.current().kind == TokenKind::Newline {
                                self.expect(TokenKind::Newline);
                            }
                        }
                        else_case = Some(MatchCase {
                            condition: vec![],
                            body,
                            span: self.current().span
                        });
                        break;
                    }
                    let mut condition: Vec<Expression> = vec![];
                    condition.push(self.parse_expression());
                    while self.current().kind == TokenKind::Pipe {
                        self.expect(TokenKind::Pipe);
                        condition.push(self.parse_expression());
                    }
                    self.expect(TokenKind::Arrow);
                    let mut body: Vec<Statement> = vec![];
                    if self.current().kind == TokenKind::OpenBrace {
                        self.expect(TokenKind::OpenBrace);
                        while self.current().kind != TokenKind::CloseBrace {
                            body.push(self.parse_statement());
                        }
                        self.expect(TokenKind::CloseBrace);
                    } else {
                        body.push(Statement::Expression(self.parse_expression(), span.clone()));
                        if self.current().kind == TokenKind::Newline {
                            self.expect(TokenKind::Newline);
                        }
                    }
                    cases.push(MatchCase {
                        condition,
                        body,
                        span: self.current().span
                    });
                }
                Expression::Match(Box::new(expr), cases, else_case, span)
            }
            TokenKind::Unsafe => {
                let span: Span = self.current().span;
                self.expect(TokenKind::Unsafe);
                let expression = self.parse_expression();
                Expression::Unsafe(Box::new(expression), span)
            }
            TokenKind::Cpp => {
                let span: Span = self.current().span;
                self.expect(TokenKind::Cpp);
                let cpp: String = self.expect(TokenKind::StringLiteral).literal.unwrap(); 
                Expression::Cpp(cpp, span)
            }
            _ => panic!("unexpected token: {:?}", self.current().kind)
        }
    }

    fn parse_access_flags(&mut self) -> Vec<AccessFlag> {
        let mut flags: Vec<AccessFlag> = vec![];
        while self.current().kind == TokenKind::Public || 
                self.current().kind == TokenKind::Private || 
                self.current().kind == TokenKind::External || 
                self.current().kind == TokenKind::Internal ||
                self.current().kind == TokenKind::Virtual ||
                self.current().kind == TokenKind::Override {
            let flag: AccessFlag = match self.current().kind {
                TokenKind::Public => AccessFlag::Public,
                TokenKind::Private => AccessFlag::Private,
                TokenKind::External => AccessFlag::External,
                TokenKind::Internal => AccessFlag::Internal,
                TokenKind::Virtual => AccessFlag::Virtual,
                TokenKind::Override => AccessFlag::Override,
                _ => panic!("unexpected token: {:?}", self.current().kind)
            };
            if flags.contains(&flag) {
                panic!("duplicate access flag: {:?}", flag);
            }
            flags.push(flag);
            self.advance();
        }
        flags
    }
    fn parse_type(&mut self) -> Type {
        let span: Span = self.current().span;
        match self.current().kind {
            TokenKind::Identifier => {
                let current: String = self.current().literal.unwrap();
                if self.data_enums.contains_key(&current) {
                    let identifier: String = self.expect(TokenKind::Identifier).literal.unwrap();
                    Type::DataEnum(identifier, span)
                } else if self.data_structs.contains_key(&current) {
                    let identifier: String = self.expect(TokenKind::Identifier).literal.unwrap();
                    Type::DataStruct(identifier, span)
                } else if self.aliases.contains_key(&current) {
                    let identifier: String = self.expect(TokenKind::Identifier).literal.unwrap();
                    Type::Alias(identifier, span)
                } else if self.objects.contains_key(&current) {
                    let identifier: String = self.expect(TokenKind::Identifier).literal.unwrap();
                    Type::Object(identifier, span)
                } else {
                    for t in self.current_generic_parameters.clone() {
                        if let Type::GenericParameter(name, _) = t.clone() {
                            if name == current {
                                self.advance();
                                return Type::Generic(name, span);
                            }
                        }
                    }
                    self.advance();
                    Type::Unknown(current, span)
                }
            }
            TokenKind::Unit => {
                self.expect(TokenKind::Unit);
                Type::Unit(span)
            }
            TokenKind::Int => {
                self.expect(TokenKind::Int);
                Type::Int(span)
            }
            TokenKind::Char => {
                self.expect(TokenKind::Char);
                Type::Char(span)
            }
            TokenKind::Bool => {
                self.expect(TokenKind::Bool);
                Type::Bool(span)
            }
            TokenKind::OpenBracket => {
                self.expect(TokenKind::OpenBracket);
                let t: Type = self.parse_type();
                self.expect(TokenKind::CloseBracket);
                Type::Array(Box::new(t), span)
            }
            TokenKind::OpenParenthesis => {
                self.expect(TokenKind::OpenParenthesis);
                let mut types: Vec<Type> = vec![];
                while self.current < self.tokens.len() && self.current().kind != TokenKind::CloseParenthesis {
                    types.push(self.parse_type());
                    if self.current().kind == TokenKind::Comma {
                        self.expect(TokenKind::Comma);
                    }
                }
                self.expect(TokenKind::CloseParenthesis);
                self.expect(TokenKind::Arrow);
                let t: Type = self.parse_type();
                Type::Function(types, Box::new(t), span)
            }
            _ => panic!("unexpected type: {:?}", self.current().kind)
        }
    }
    fn parse_enum_variant(&mut self) -> EnumVarient {
        let span: Span = self.current().span;
        let identifier: String = self.expect(TokenKind::Identifier).literal.unwrap();
        if self.current().kind == TokenKind::OpenParenthesis {
            self.expect(TokenKind::OpenParenthesis);
            let mut types: Vec<Type> = vec![];
            while self.current < self.tokens.len() && self.current().kind != TokenKind::CloseParenthesis {
                types.push(self.parse_type());
                if self.current().kind == TokenKind::Comma {
                    self.expect(TokenKind::Comma);
                }
            }
            self.expect(TokenKind::CloseParenthesis);
            EnumVarient::Tuple(identifier, types, span)
        } else {
            EnumVarient::Unit(identifier, span)
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Token {
        let token: Token = self.current();
        if token.kind != kind {
            panic!("unexpected token: {:?} ({:?}), expected: {:?}", token.kind, token.literal, kind);
        }
        self.advance();
        token
    }
    fn current(&mut self) -> Token {
        if self.current < self.tokens.len() {
            return self.tokens[self.current].clone();
        }
        panic!("unexpected end of file")
    }
    fn advance(&mut self) {
        if self.current >= self.tokens.len() {
            panic!("unexpected end of file")
        }
        self.current += 1;
    }
}