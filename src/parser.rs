use crate::token::{
    TokenKind,
    Token,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub kind: StatementKind,
    pub line: usize,
    pub column: usize,
}

impl Statement {
    pub fn new(kind: StatementKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementKind {
    // Statements
    Expression(Expression),
    VarDeclaration(VarDeclaration),
    ConstantDeclaration(ConstantDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    ClassDeclaration(ClassDeclaration),
    ConstructorDeclaration(ConstructorDeclaration),

    Return(Return),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDeclaration {
    pub name: String,
    pub type_: Type,
    pub initializer: Option<Expression>,
}

impl VarDeclaration {
    pub fn new(name: String, type_: Type, initializer: Option<Expression>) -> Self {
        Self { name, type_, initializer }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantDeclaration {
    pub statement: Box<Statement>,
}

impl ConstantDeclaration {
    pub fn new(statement: Box<Statement>) -> Self {
        Self { statement }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

impl FunctionDeclaration {
    pub fn new(name: String, parameters: Vec<Parameter>, return_type: Type, body: Vec<Statement>) -> Self {
        Self { name, parameters, return_type, body }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclaration {
    pub name: String,
    pub body: Vec<Statement>,
}

impl ClassDeclaration {
    pub fn new(name: String, body: Vec<Statement>) -> Self {
        Self { name, body }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub type_: Type,
}

impl Parameter {
    pub fn new(name: String, type_: Type) -> Self {
        Self { name, type_ }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub expression: Expression,
}

impl Return {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ConstructorDeclaration {
    pub parameters: Vec<Parameter>,
    pub body: Vec<Statement>,
}

impl ConstructorDeclaration {
    pub fn new(parameters: Vec<Parameter>, body: Vec<Statement>) -> Self {
        Self { parameters, body }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub line: usize,
    pub column: usize,
}

impl Expression {
    pub fn new(kind: ExpressionKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind {
    // Expressions
    Literal(Literal),
    Parenthesized(Parenthesized),
    Variable(Variable),
    Binary(Binary),
    Unary(Unary),
    Call(Box<Call>),
    Index(Index),
    Slice(Slice),
    Member(Member),
    Array(Array),
    Map(Map),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    // Types
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
    String,
    Char,
    Integer,
    Float,
    Bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub kind: TypeKind,
    pub line: usize,
    pub column: usize,
}

impl Type {
    pub fn new(kind: TypeKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub elements: Vec<Expression>,
}

impl Array {
    pub fn new(elements: Vec<Expression>) -> Self {
        Self { elements }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    pub entries: Vec<(Expression, Expression)>,
}

impl Map {
    pub fn new(entries: Vec<(Expression, Expression)>) -> Self {
        Self { entries }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parenthesized {
    pub expression: Box<Expression>,
}

impl Parenthesized {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    // Literals
    String(String),
    Char(char),
    Integer(i64),
    Float(f64),
    Bool(bool),
    Null,
    Map(Vec<(Expression, Expression)>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub kind: LiteralKind,
    pub line: usize,
    pub column: usize,
}

impl Literal {
    pub fn new(kind: LiteralKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub line: usize,
    pub column: usize,
}

impl Variable {
    pub fn new(name: String, line: usize, column: usize) -> Self {
        Self { name, line, column }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: String,
    pub arguments: Vec<Expression>,
}

impl Call {
    pub fn new(callee: String, arguments: Vec<Expression>) -> Self {
        Self { callee, arguments }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Index {
    pub callee: Box<Expression>,
    pub index: Box<Expression>,
}

impl Index {
    pub fn new(callee: Box<Expression>, index: Box<Expression>) -> Self {
        Self { callee, index }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Slice {
    pub callee: Box<Expression>,
    pub start: Box<Expression>,
    pub end: Box<Expression>,
}

impl Slice {
    pub fn new(callee: Box<Expression>, start: Box<Expression>, end: Box<Expression>) -> Self {
        Self { callee, start, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub callee: Box<Expression>,
    pub name: String,
}

impl Member {
    pub fn new(callee: Box<Expression>, name: String) -> Self {
        Self { callee, name }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: TokenKind,
    pub right: Box<Expression>,
}

impl Binary {
    pub fn new(left: Box<Expression>, operator: TokenKind, right: Box<Expression>) -> Self {
        Self { left, operator, right }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    pub operator: TokenKind,
    pub operand: Box<Expression>,
}

impl Unary {
    pub fn new(operator: TokenKind, operand: Box<Expression>) -> Self {
        Self { operator, operand }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut index = 0;
    while index < tokens.len() {
        let statement = parse_statement(&tokens, &mut index);
        statements.push(statement);
    }
    statements
}
pub fn parse_statement(tokens: &Vec<Token>, index: &mut usize) -> Statement {
    let token = &tokens[*index];
    match token.kind {
        TokenKind::VarKw => {
            let statement = parse_var_statement(tokens, index);
            statement
        }
        TokenKind::ConstKw => {
            expectc(tokens, index, TokenKind::ConstKw);
            let statement = parse_var_statement(tokens, index);
            return Statement::new(StatementKind::ConstantDeclaration(ConstantDeclaration::new(Box::new(statement))), token.line, token.column);
        }
        TokenKind::FnKw => {
            return parse_function_statement(tokens, index);
        }
        TokenKind::ConstructorKw => {
            return parse_constructor_statement(tokens, index);
        }
        TokenKind::ClassKw => {
            return parse_class_statement(tokens, index);
        }
        TokenKind::ReturnKw => {
            return parse_return_statement(tokens, index);
        }
        _ => {
            let expression = parse_expression(tokens, index);
            Statement::new(StatementKind::Expression(expression), token.line, token.column)
        }
    }
}
pub fn parse_function_statement(tokens: &Vec<Token>, index: &mut usize) -> Statement {
    let token = &tokens[*index];
    expectc(tokens, index, TokenKind::FnKw);
    let name = tokens[*index].lexeme.clone();
    *index += 1;
    expectc(tokens, index, TokenKind::LeftParen);
    let mut parameters = Vec::new();
    while tokens[*index].kind != TokenKind::RightParen {
        let name = tokens[*index].lexeme.clone();
        *index += 1;
        expectc(tokens, index, TokenKind::Colon);
        let type_ = parse_type(tokens, index);
        parameters.push(Parameter::new(name, type_));
        if tokens[*index].kind == TokenKind::Comma {
            expectc(tokens, index, TokenKind::Comma);
        }
    }
    expectc(tokens, index, TokenKind::RightParen);
    expectc(tokens, index, TokenKind::Colon);
    let return_type = parse_type(tokens, index);
    expectc(tokens, index, TokenKind::LeftBrace);
    let mut body = Vec::new();
    while tokens[*index].kind != TokenKind::RightBrace {
        let statement = parse_statement(tokens, index);
        body.push(statement);
    }
    expectc(tokens, index, TokenKind::RightBrace);
    Statement::new(StatementKind::FunctionDeclaration(FunctionDeclaration::new(name, parameters, return_type, body)), token.line, token.column)
}
pub fn parse_constructor_statement(tokens: &Vec<Token>, index: &mut usize) -> Statement {
    let token = &tokens[*index];
    expectc(tokens, index, TokenKind::ConstructorKw);
    expectc(tokens, index, TokenKind::LeftParen);
    let mut parameters = Vec::new();
    while tokens[*index].kind != TokenKind::RightParen {
        let name = tokens[*index].lexeme.clone();
        *index += 1;
        expectc(tokens, index, TokenKind::Colon);
        let type_ = parse_type(tokens, index);
        parameters.push(Parameter::new(name, type_));
        if tokens[*index].kind == TokenKind::Comma {
            expectc(tokens, index, TokenKind::Comma);
        }
    }
    expectc(tokens, index, TokenKind::RightParen);
    expectc(tokens, index, TokenKind::LeftBrace);
    let mut body = Vec::new();
    while tokens[*index].kind != TokenKind::RightBrace {
        let statement = parse_statement(tokens, index);
        body.push(statement);
    }
    expectc(tokens, index, TokenKind::RightBrace);
    Statement::new(StatementKind::ConstructorDeclaration(ConstructorDeclaration::new(parameters, body)), token.line, token.column)
}
pub fn parse_class_statement(tokens: &Vec<Token>, index: &mut usize) -> Statement {
    let token = &tokens[*index];
    expectc(tokens, index, TokenKind::ClassKw);
    let name = tokens[*index].lexeme.clone();
    *index += 1;
    expectc(tokens, index, TokenKind::LeftBrace);
    let mut body = Vec::new();
    while tokens[*index].kind != TokenKind::RightBrace {
        let statement = parse_statement(tokens, index);
        body.push(statement);
    }
    expectc(tokens, index, TokenKind::RightBrace);
    Statement::new(StatementKind::ClassDeclaration(ClassDeclaration::new(name, body)), token.line, token.column)
}
pub fn parse_var_statement(tokens: &Vec<Token>, index: &mut usize) -> Statement {
    let token = &tokens[*index];
    expectc(tokens, index, TokenKind::VarKw);
    let name = tokens[*index].lexeme.clone();
    *index += 1;
    expectc(tokens, index, TokenKind::Colon);
    let type_ = parse_type(tokens, index);
    expectc(tokens, index, TokenKind::Equal);
    let initializer = Some(parse_expression(tokens, index));
    expectc(tokens, index, TokenKind::Semicolon);
    Statement::new(
        StatementKind::VarDeclaration(VarDeclaration::new(name, type_, initializer)),
        token.line,
        token.column,
    )
}
pub fn parse_return_statement(tokens: &Vec<Token>, index: &mut usize) -> Statement {
    let token = &tokens[*index];
    expectc(tokens, index, TokenKind::ReturnKw);
    let expression = parse_expression(tokens, index);
    expectc(tokens, index, TokenKind::Semicolon);
    Statement::new(StatementKind::Return(Return::new(expression)), token.line, token.column)
}
pub fn parse_type(tokens: &Vec<Token>, index: &mut usize) -> Type {
    let token = &tokens[*index];
    match token.kind {
        TokenKind::ArrayKw => {
            expectc(tokens, index, TokenKind::ArrayKw);
            expectc(tokens, index, TokenKind::Less);
            let type_ = parse_type(tokens, index);
            expectc(tokens, index, TokenKind::Greater);
            Type::new(TypeKind::Array(Box::new(type_)), token.line, token.column)
        }
        TokenKind::MapKw => {
            expectc(tokens, index, TokenKind::LeftBrace);
            let key_type = parse_type(tokens, index);
            expectc(tokens, index, TokenKind::Colon);
            let value_type = parse_type(tokens, index);
            expectc(tokens, index, TokenKind::RightBrace);
            *index += 1;
            Type::new(TypeKind::Map(Box::new(key_type), Box::new(value_type)), token.line, token.column)
        }
        TokenKind::StringKw => {
            *index += 1;
            Type::new(TypeKind::String, token.line, token.column)
        }
        TokenKind::CharKw => {
            *index += 1;
            Type::new(TypeKind::Char, token.line, token.column)
        }
        TokenKind::IntegerKw => {
            *index += 1;
            Type::new(TypeKind::Integer, token.line, token.column)
        }
        TokenKind::FloatKw => {
            *index += 1;
            Type::new(TypeKind::Float, token.line, token.column)
        }
        TokenKind::BoolKw => {
            *index += 1;
            Type::new(TypeKind::Bool, token.line, token.column)
        }
        _ => {
            panic!("Expected type, found {:?}", token.kind);
        }
    }
}
pub fn parse_expression(tokens: &Vec<Token>, index: &mut usize) -> Expression {
    let token = &tokens[*index];
    let mut expression = parse_additive_expression(tokens, index);
    while tokens[*index].kind == TokenKind::EqualEqual || tokens[*index].kind == TokenKind::BangEqual {
        let operator = tokens[*index].kind.clone();
        *index += 1;
        let right = parse_additive_expression(tokens, index);
        expression = Expression::new(
            ExpressionKind::Binary(Binary::new(Box::new(expression), operator, Box::new(right))),
            token.line,
            token.column,
        );
    }
    expression
}
pub fn parse_additive_expression(tokens: &Vec<Token>, index: &mut usize) -> Expression {
    let mut expression = parse_multiplicative_expression(tokens, index);
    while tokens[*index].kind == TokenKind::Plus || tokens[*index].kind == TokenKind::Minus {
        let token = &tokens[*index];
        let operator = token.kind.clone();
        *index += 1;
        let right = parse_multiplicative_expression(tokens, index);
        expression = Expression::new(
            ExpressionKind::Binary(Binary::new(Box::new(expression), operator, Box::new(right))),
            token.line,
            token.column,
        );
    }
    expression
}
pub fn parse_multiplicative_expression(tokens: &Vec<Token>, index: &mut usize) -> Expression {
    let mut expression = parse_unary_expression(tokens, index);
    while tokens[*index].kind == TokenKind::Star || tokens[*index].kind == TokenKind::Slash {
        let token = &tokens[*index];
        let operator = token.kind.clone();
        *index += 1;
        let right = parse_unary_expression(tokens, index);
        expression = Expression::new(
            ExpressionKind::Binary(Binary::new(Box::new(expression), operator, Box::new(right))),
            token.line,
            token.column,
        );
    }
    expression
}
pub fn parse_unary_expression(tokens: &Vec<Token>, index: &mut usize) -> Expression {
    let token = &tokens[*index];
    if tokens[*index].kind == TokenKind::Bang || tokens[*index].kind == TokenKind::Minus {
        let operator = tokens[*index].kind.clone();
        *index += 1;
        let right = parse_unary_expression(tokens, index);
        Expression::new(
            ExpressionKind::Unary(Unary::new(operator, Box::new(right))),
            token.line,
            token.column,
        )
    } else {
        parse_primary_expression(tokens, index)
    }
}
pub fn parse_primary_expression(tokens: &Vec<Token>, index: &mut usize) -> Expression {
    let token = &tokens[*index];
    match token.kind.clone() {
        TokenKind::Identifier(identifier) => {
            *index += 1;
            if tokens[*index].kind == TokenKind::LeftParen {
                *index += 1;
                let expression = parse_call_expression(tokens, index);
                expectc(tokens, index, TokenKind::RightParen);
                expression
            } else {
                Expression::new(ExpressionKind::Variable(Variable::new(identifier, token.line, token.column)), token.line, token.column)
            }
        }
        TokenKind::Integer(integer) => {
            *index += 1;
            Expression::new(ExpressionKind::Literal(Literal::new(LiteralKind::Integer(integer), token.line, token.column)), token.line, token.column)
        }
        TokenKind::Float(float) => {
            *index += 1;
            Expression::new(ExpressionKind::Literal(Literal::new(LiteralKind::Float(float), token.line, token.column)), token.line, token.column)
        }
        TokenKind::String(string) => {
            *index += 1;
            Expression::new(ExpressionKind::Literal(Literal::new(LiteralKind::String(string), token.line, token.column)), token.line, token.column)
        }
        TokenKind::Character(character) => {
            *index += 1;
            Expression::new(ExpressionKind::Literal(Literal::new(LiteralKind::Char(character), token.line, token.column)), token.line, token.column)
        }
        TokenKind::TrueKw => {
            *index += 1;
            Expression::new(ExpressionKind::Literal(Literal::new(LiteralKind::Bool(true), token.line, token.column)), token.line, token.column)
        }
        TokenKind::FalseKw => {
            *index += 1;
            Expression::new(ExpressionKind::Literal(Literal::new(LiteralKind::Bool(false), token.line, token.column)), token.line, token.column)
        }
        TokenKind::NullKw => {
            *index += 1;
            Expression::new(ExpressionKind::Literal(Literal::new(LiteralKind::Null, token.line, token.column)), token.line, token.column)
        }
        TokenKind::LeftParen => {
            *index += 1;
            let expression = parse_expression(tokens, index);
            expectc(tokens, index, TokenKind::RightParen);
            expression
        }
        _ => {
            panic!("Expected primary expression, found {:?}", token.kind);
        }
    }
}
pub fn parse_call_expression(tokens: &Vec<Token>, index: &mut usize) -> Expression {
    let token = &tokens[*index];
    let expression = parse_primary_expression(tokens, index);
    let name = match expression.kind.clone() {
        ExpressionKind::Variable(variable) => variable.name,
        _ => panic!("Expected variable, found {:?}", expression.kind),
    };
    if tokens[*index].kind == TokenKind::LeftParen {
        *index += 1;
        let mut arguments = Vec::new();
        while tokens[*index].kind != TokenKind::RightParen {
            arguments.push(parse_expression(tokens, index));
            if tokens[*index].kind == TokenKind::Comma {
                *index += 1;
            }
        }
        *index += 1;
        Expression::new(
            ExpressionKind::Call(Box::new(Call::new(name, arguments))),
            token.line,
            token.column,
        )
    } else {
        expression
    }
} 

// --- Dumping AST ---
pub fn dump_ast(ast: &Vec<Statement>) {
    for statement in ast {
        dump_statement(statement, 0);
    }
}
pub fn dump_statement(statement: &Statement, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    match &statement.kind {
        StatementKind::Expression(expression) => {
            println!("{}Expression", indent_string);
            dump_expression(expression, indent + 1);
        }
        StatementKind::VarDeclaration(variable_declaration) => {
            println!("{}VariableDeclaration", indent_string);
            dump_variable_declaration(variable_declaration, indent + 1);
        }
        StatementKind::ConstantDeclaration(constant_declaration) => {
            println!("{}ConstantDeclaration", indent_string);
            dump_constant_declaration(constant_declaration, indent + 1);
        }
        StatementKind::FunctionDeclaration(function_declaration) => {
            println!("{}FunctionDeclaration", indent_string);
            dump_function_declaration(function_declaration, indent + 1);
        }
        StatementKind::ClassDeclaration(class_declaration) => {
            println!("{}ClassDeclaration", indent_string);
            dump_class_declaration(class_declaration, indent + 1);
        }
        StatementKind::Return(return_statement) => {
            println!("{}Return", indent_string);
            dump_return(return_statement, indent + 1);
        }
        StatementKind::ConstructorDeclaration(constructor_declaration) => {
            println!("{}ConstructorDeclaration", indent_string);
            dump_constructor_declaration(constructor_declaration, indent + 1);
        }
        _ => {
            println!("{}Statement", indent_string);
        }
    }
}
pub fn dump_expression(expression: &Expression, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    match &expression.kind {
        ExpressionKind::Literal(literal) => {
            println!("{}Literal", indent_string);
            dump_literal(literal, indent + 1);
        }
        ExpressionKind::Variable(variable) => {
            println!("{}Variable", indent_string);
            dump_variable(variable, indent + 1);
        }
        ExpressionKind::Binary(binary) => {
            println!("{}Binary", indent_string);
            dump_binary(binary, indent + 1);
        }
        ExpressionKind::Unary(unary) => {
            println!("{}Unary", indent_string);
            dump_unary(unary, indent + 1);
        }
        ExpressionKind::Call(call) => {
            println!("{}Call", indent_string);
            dump_call(call, indent + 1);
        }
        ExpressionKind::Array(array) => {
            println!("{}Array", indent_string);
            dump_array(array, indent + 1);
        }
        ExpressionKind::Map(map) => {
            println!("{}Map", indent_string);
            dump_map(map, indent + 1);
        }
        ExpressionKind::Parenthesized(parenthesized) => {
            println!("{}Parenthesized", indent_string);
            dump_parenthesized(parenthesized, indent + 1);
        }
        _ => {
            println!("{}Expression", indent_string);
        }
    }
}
pub fn dump_variable_declaration(variable_declaration: &VarDeclaration, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Identifier: {}", indent_string, variable_declaration.name);
    println!("{}Type: {:?}", indent_string, variable_declaration.type_.kind);
    println!("{}Initializer:", indent_string);
    dump_expression(&variable_declaration.initializer.as_ref().unwrap().clone(), indent + 1);
}
pub fn dump_function_declaration(function_declaration: &FunctionDeclaration, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Identifier: {}", indent_string, function_declaration.name);
    println!("{}Parameters:", indent_string);
    for parameter in &function_declaration.parameters {
        dump_parameter(parameter, indent + 1);
    }
    println!("{}Body:", indent_string);
    for statement in &function_declaration.body {
        dump_statement(statement, indent + 1);
    }
}
pub fn dump_constructor_declaration(constructor_declaration: &ConstructorDeclaration, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Parameters:", indent_string);
    for parameter in &constructor_declaration.parameters {
        dump_parameter(parameter, indent + 1);
    }
    println!("{}Body:", indent_string);
    for statement in &constructor_declaration.body {
        dump_statement(statement, indent + 1);
    }
}
pub fn dump_class_declaration(class_declaration: &ClassDeclaration, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Identifier: {}", indent_string, class_declaration.name);
    println!("{}Body:", indent_string);
    for b in &class_declaration.body {
        dump_statement(b, indent + 1);
    }
}
pub fn dump_constant_declaration(constant_declaration: &ConstantDeclaration, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    dump_statement(&constant_declaration.statement, indent);
}
pub fn dump_parameter(parameter: &Parameter, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Identifier: {}, Type: {:?}", indent_string, parameter.name, parameter.type_.kind);
}
pub fn dump_return(return_statement: &Return, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Value:", indent_string);
    dump_expression(&return_statement.expression, indent + 1);
}
pub fn dump_literal(literal: &Literal, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    match &literal.kind {
        LiteralKind::Integer(number) => {
            println!("{}Integer: {}", indent_string, number);
        }
        LiteralKind::Float(number) => {
            println!("{}Float: {}", indent_string, number);
        }
        LiteralKind::String(string) => {
            println!("{}String: {}", indent_string, string);
        }
        LiteralKind::Char(character) => {
            println!("{}Character: {}", indent_string, character);
        }
        LiteralKind::Bool(boolean) => {
            println!("{}Boolean: {}", indent_string, boolean);
        }
        LiteralKind::Null => {
            println!("{}Null", indent_string);
        }
        _ => {
            println!("{}Literal", indent_string);
        }
    }
}
pub fn dump_variable(variable: &Variable, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Variable: {}", indent_string, variable.name);
}
pub fn dump_binary(binary: &Binary, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Operator: {:?}", indent_string, binary.operator);
    println!("{}Left:", indent_string);
    dump_expression(&binary.left, indent + 1);
    println!("{}Right:", indent_string);
    dump_expression(&binary.right, indent + 1);
}
pub fn dump_unary(unary: &Unary, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Operator: {:?}", indent_string, unary.operator);
    println!("{}Right:", indent_string);
    dump_expression(&unary.operand, indent + 1);
}
pub fn dump_call(call: &Call, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Callee: {}", indent_string, call.callee);
    println!("{}Arguments:", indent_string);
    for argument in &call.arguments {
        dump_expression(argument, indent + 1);
    }
}
pub fn dump_parenthesized(parenthesized: &Parenthesized, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Expression:", indent_string);
    dump_expression(&parenthesized.expression, indent + 1);
}
pub fn dump_map(map: &Map, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Entries:", indent_string);
    for entry in &map.entries {
        dump_map_entry(entry, indent + 1);
    }
}
pub fn dump_map_entry(map_entry: &(Expression, Expression), indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Key:", indent_string);
    dump_expression(&map_entry.0, indent + 1);
    println!("{}Value:", indent_string);
    dump_expression(&map_entry.1, indent + 1);
}
pub fn dump_array(array: &Array, indent: usize) {
    let mut indent_string = String::new();
    for _ in 0..indent {
        indent_string.push_str("  ");
    }
    println!("{}Elements:", indent_string);
    for element in &array.elements {
        dump_expression(element, indent + 1);
    }
}

// --- Helper functions ---
pub fn matchc(tokens: &Vec<Token>, index: &mut usize, kind: TokenKind) -> bool {
    if tokens[*index].kind == kind {
        *index += 1;
        true
    } else {
        false
    }
}
pub fn expectc(tokens: &Vec<Token>, index: &mut usize, kind: TokenKind) -> String {
    if *index >= tokens.len() {
        panic!("Unexpected end of file");
    }

    if tokens[*index].kind == kind {
        let token = &tokens[*index];
        *index += 1;
        token.lexeme.clone()
    } else {
        panic!("Expected {:?} at line {}, column {}, got {:?}", kind, tokens[*index].line, tokens[*index].column, tokens[*index].kind);
    }
}