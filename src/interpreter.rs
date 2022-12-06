use crate::parser::*;

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub global_scope: Scope,
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub classes: Vec<Class>,
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub local_scope: Scope,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub local_scope: Scope,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub _type: Type,
    pub value: Expression,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            global_scope: Scope {
                classes: Vec::new(),
                functions: Vec::new(),
                variables: Vec::new(),
            },
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for statement in statements {
            match statement.kind.clone() {
                StatementKind::FunctionDeclaration(function) => {
                    self.global_scope.functions.push(Function {
                        name: function.name,
                        parameters: function.parameters,
                        local_scope: Scope {
                            classes: vec![],
                            functions: vec![],
                            variables: vec![],
                        },
                    });
                }
                StatementKind::VarDeclaration(variable) => {
                    self.global_scope.variables.push(Variable {
                        name: variable.name,
                        _type: variable.type_,
                        value: variable.initializer.unwrap(),
                    });
                }
                StatementKind::ClassDeclaration(class) => {
                    let local_scope = self.interpret_class_body(class.body);
                    self.global_scope.classes.push(Class {
                        name: class.name,
                        local_scope,
                    });
                }
                _ => {}
            }
        }
    }

    pub fn interpret_class_body(&mut self, statements: Vec<Statement>) -> Scope {
        let mut local_scope = Scope {
            classes: vec![],
            functions: vec![],
            variables: vec![],
        };
        for statement in statements {
            match statement.kind.clone() {
                StatementKind::FunctionDeclaration(function) => {
                    let ls = self.interpret_function_body(function.body);
                    local_scope.functions.push(Function {
                        name: function.name,
                        parameters: function.parameters,
                        local_scope: ls
                    });
                }
                _ => {}
            }
        }
        local_scope
    }

    pub fn interpret_function_body(&mut self, statements: Vec<Statement>) -> Scope {
        let mut local_scope = Scope {
            classes: vec![],
            functions: vec![],
            variables: vec![],
        };
        for statement in statements {
            match statement.kind.clone() {
                StatementKind::VarDeclaration(variable) => {
                    local_scope.variables.push(Variable {
                        name: variable.name,
                        _type: variable.type_,
                        value: variable.initializer.unwrap(),
                    });
                }
                _ => {}
            }
        }
        local_scope
    }

}
