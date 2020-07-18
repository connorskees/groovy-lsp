use crate::ast::{Expr, Identifier, Parameter, Variable, VariableScope};

#[derive(Debug)]
pub enum Stmt {
    /// Represents a Groovy `assert` statement
    ///
    /// E.g. `assert  1 == 0, "1 does not equal 0"`
    Assert {
        bool_expr: Expr,
        message: Expr,
    },
    Block {
        body: Vec<Stmt>,
        // todo: this shouldn't be optional
        scope: Option<VariableScope>,
    },
    Break {
        label: Option<Identifier>,
    },
    Case,
    Catch,
    Continue,
    DoWhile,
    Empty,
    Expression(Expr),
    For {
        variable: Parameter,
        collection: Expr,
        loop_block: Box<Stmt>,
        scope: VariableScope,
    },
    If {
        expr: Expr,
        if_block: Box<Stmt>,
        else_block: Box<Stmt>,
    },
    Return {
        expr: Expr,
    },
    Switch,
    Synchronized,
    Throw,
    TryCatch,
    While,
    VariableDeclaration(Variable),
}
