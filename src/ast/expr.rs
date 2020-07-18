use std::rc::Rc;

use crate::ast::{
    AssignmentOperator, BinaryOperator, Field, Identifier, Method, MethodModifier, Parameter, Stmt,
    Type, Variable, VariableScope,
};

#[derive(Debug)]
pub enum Expr {
    Array {
        values: Vec<Expr>,
        length: Box<Expr>,
        element_type: Type,
    },
    Attribute,
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
        is_safe: bool,
    },
    BitwiseNegation {
        expr: Box<Expr>,
    },
    Boolean {
        expr: Box<Expr>,
    },
    Cast {
        expr: Box<Expr>,
        ignore_auto_boxing: bool,
        coerce: bool,
        strict: bool,
        cast_to_type: Type,
    },
    /// Represents access to a Java/Groovy class in an expression, such
    /// as when invoking a static method or accessing a static type
    Class {
        type_name: Type,
    },
    Closure {
        parameters: Vec<Parameter>,
        body: Box<Stmt>,
        scope: VariableScope,
    },
    ClosureList,
    Constant(ConstExpr),
    ConstructorCall {
        arguments: Box<Expr>,
        uses_anonymous_inner_class: bool,
    },
    Declaration {
        left: Box<Expr>,
        op: AssignmentOperator,
        right: Box<Expr>,
        is_safe: bool,
    },
    ElvisOperator,
    Empty,
    /// Represents field access, `this.foo`
    Field {
        field: Box<Field>,
        use_ref: bool,
    },
    GString(GString),
    /// A lambda expression, taking the form of
    ///  - x -> x * 2
    ///  - (x, y) -> x + y
    ///  - (x, y) -> { x + y }
    ///  - (int x, int y) -> { x + y }
    Lambda {
        parameters: Vec<Parameter>,
        body: Box<Stmt>,
        scope: VariableScope,
        is_serializable: bool,
    },
    List {
        elements: Vec<Expr>,
        wrapped: bool,
    },
    MapEntry(MapEntry),
    Map {
        elements: Vec<MapEntry>,
    },
    MethodCall {
        object: Box<Expr>,
        method: Box<Expr>,
        arguments: Box<Expr>,
        implicit_this: bool,
        spread_safe: bool,
        safe: bool,
        target: Rc<Method>,
    },
    MethodPointer,
    Not,
    Postfix,
    Prefix,
    Property,
    Range,
    Spread,
    SpreadMap,
    StaticMethodCall,
    Ternary,
    Tuple,
    UnaryMinus,
    UnaryPlus,
    Variable {
        name: Identifier,
        modifiers: Vec<MethodModifier>,
        in_static_context: bool,
        is_dynamically_typed: bool,
        accessed_variable: Box<Variable>,
        closure_share: bool,
        use_ref: bool,
        origin_type: Option<Type>,
    },
}

#[derive(Debug, PartialEq)]
pub enum ConstExpr {
    True,
    False,
    Null,
    String(String),
    Number(String),
}

/// A single key-value map entry
#[derive(Debug)]
pub struct MapEntry {
    key: Box<Expr>,
    value: Box<Expr>,
}

#[derive(Debug)]
pub struct GString(Vec<InterpolatedStringPart>);

#[derive(Debug)]
pub enum InterpolatedStringPart {
    Literal(String),
    Identifier(Identifier),
    Expression(Box<Expr>),
}
