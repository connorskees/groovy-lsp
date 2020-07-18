use std::{collections::HashMap, rc::Rc};

mod decimal;
mod expr;
mod integer;
mod modifier;
mod operator;
mod stmt;
mod token;
mod visibility;

use crate::interner::{keywords::Keywords, Symbol};

pub use expr::{ConstExpr, Expr};
pub use modifier::{ClassModifier, MethodModifier};
pub use operator::{AssignmentOperator, BinaryOperator, UnaryOperator};
pub use stmt::Stmt;
pub use token::{Literal, Token};
pub use visibility::Visibility;

#[derive(Debug)]
pub struct Class {
    pub name: Identifier,
    pub visibility: Visibility,
    pub modifiers: Vec<ClassModifier>,
    pub super_class: Option<Rc<Class>>,
    pub methods: Vec<Method>,
    pub fields: Vec<Field>,
    pub properties: Vec<Property>,
    pub interfaces: Vec<Class>,
    pub mixin: Vec<Mixin>,
}

#[derive(Debug)]
pub struct Method {
    pub name: Identifier,
    pub visibility: Visibility,
    pub modifiers: Vec<MethodModifier>,
    pub return_type: Type,
    pub parameters: Vec<Parameter>,
    pub has_default_value: bool,
    pub exceptions: Vec<Class>,
    pub static_constructor: bool,
    pub body: Stmt,
    pub is_constructor: bool,
}

#[derive(Debug)]
pub struct Field {
    name: Identifier,
    visibility: Visibility,
    modifiers: Vec<MethodModifier>,
    field_type: Type,
    owner: Class,
    dynamically_typed: bool,
    holder: bool,
    origin_type: Option<Type>,
    initial_value: Expr,
}

#[derive(Debug)]
pub struct Property {
    field: Field,
    setter_block: Stmt,
    getter_block: Stmt,
    modifiers: Vec<MethodModifier>,
}

#[derive(Debug)]
pub struct Interface {}

#[derive(Debug)]
pub struct Mixin {}

#[derive(Debug)]
pub struct Parameter {
    pub param_type: Type,
    pub name: Identifier,
    pub origin_type: Option<Type>,
    pub dynamically_typed: bool,
    pub closure_shared: bool,
    pub default_value: Option<Expr>,
    pub in_static_context: bool,
    pub modifiers: Vec<MethodModifier>,
}

#[derive(Debug)]
pub struct Import {
    import_type: Type,
    alias: Identifier,
    field_name: Identifier,
    package_name: Identifier,
    is_star: bool,
    is_static: bool,
}

#[derive(Debug)]
pub enum AstNode {
    Annotated,
    ClassCodeVisitorSupport,
    Class(Class),
    Constructor(Method),
    Field(Field),
    Import(Import),
    Method(Method),
    Module,
    Parameter(Parameter),
    Property(Property),
}

#[derive(Debug)]
pub struct Variable {
    pub name: Identifier,
    pub type_name: Type,
    pub value: Expr,
    pub in_static_context: bool,
    pub is_dynamically_typed: bool,
    pub is_closure_shared_variable: bool,
    pub modifiers: Vec<MethodModifier>,
}

#[derive(Debug)]
pub struct VariableScope {
    parent: Option<Box<VariableScope>>,
    class_scope: Box<VariableScope>,
    in_static_context: bool,
    declared_variables: HashMap<Identifier, Variable>,
    referenced_local_variables: HashMap<Identifier, Variable>,
    referenced_class_variables: HashMap<Identifier, Variable>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identifier {
    pub name: Symbol,
}

impl Identifier {
    pub fn new(s: &str) -> Self {
        Self {
            name: Symbol::intern(s),
        }
    }
}

#[derive(Debug)]
pub enum Type {
    /// void
    Void,
    /// bool
    Boolean,
    /// 1 byte integer
    Byte,
    /// 2 byte integer
    Short,
    ///
    Int,
    Double,
    Float,
    Char,
    Long,
    Class(Identifier),
    Array(Box<Type>),
}
