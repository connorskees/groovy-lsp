use std::{collections::HashMap, rc::Rc};

mod decimal;
mod integer;

use crate::interner::{keywords::Keywords, Symbol};

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
pub enum Expr {
    Array {
        values: Vec<Expr>,
        length: Box<Expr>,
        element_type: Type,
    },
    Attribute,
    Binary,
    BitwiseNegation,
    Boolean,
    Cast,
    Class,
    Closure,
    ClosureList,
    Constant,
    ConstructorCall,
    ElvisOperator,
    Field,
    GString,
    List,
    MapEntry,
    Map,
    MethodCall,
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
        accessed_variable: Variable,
        closure_share: bool,
        use_ref: bool,
        origin_type: Option<Type>,
    },
}

#[derive(Debug)]
pub struct Variable {}

#[derive(Debug)]
pub struct VariableScope {
    parent: Option<Box<VariableScope>>,
    class_scope: Box<VariableScope>,
    in_static_context: bool,
    declared_variables: HashMap<Identifier, Variable>,
    referenced_local_variables: HashMap<Identifier, Variable>,
    referenced_class_variables: HashMap<Identifier, Variable>,
}

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
}

#[derive(Debug)]
pub enum BinaryOperator {
    /// +
    Add,
    /// +=
    AddAssign,
    /// -
    Sub,
    /// -=
    SubAssign,
    /// *
    Mul,
    /// *=
    MulAssign,
    /// /
    Div,
    /// /=
    DivAssign,
    /// %
    Rem,
    /// %=
    RemAssign,
    /// **
    Pow,
    /// **=
    PowAssign,

    /// ==
    Eq,
    /// !=
    Ne,
    /// <
    Lt,
    /// >
    Gt,
    /// <=
    Le,
    /// >=
    Ge,
    /// ===
    Identical,
    /// !==
    NotIdentical,

    /// &&
    LogicalAnd,
    /// ||
    LogicalOr,

    /// &
    BitwiseAnd,
    /// |
    BitwiseOr,
    /// &=
    BitwiseAndAssign,
    /// |=
    BitwiseOrAssign,
    /// ^
    Xor,
    /// ^=
    XorAssign,
    /// <<
    Shl,
    /// >>
    Shr,
    /// <<=
    ShlAssign,
    /// >>=
    ShrAssign,

    /// =~
    Find,
    /// ==~
    Match,

    /// ~=
    BitwiseNotAssign,
}

#[derive(Debug)]
pub enum UnaryOperator {
    /// !
    LogicalNot,
    /// ~
    BitwiseNot,
}

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Literal(Literal<'a>),
    Identifier(Identifier),

    Keyword(Keywords),
    /// (
    ParenOpen,
    /// )
    ParenClose,
    /// {
    CurlyBraceOpen,
    /// }
    CurlyBraceClose,
    /// [
    SquareBraceOpen,
    /// ]
    SquareBraceClose,
    /// .
    Period,
    /// =
    SingleEqual,
    /// !
    LogicalNot,
    /// ~
    Tilde,
    /// +
    Add,
    /// +=
    AddAssign,
    /// -
    Sub,
    /// -=
    SubAssign,
    /// *
    Mul,
    /// *=
    MulAssign,
    /// /
    Div,
    /// /=
    DivAssign,
    /// %
    Rem,
    /// %=
    RemAssign,
    /// **
    Pow,
    /// **=
    PowAssign,

    /// ==
    Eq,
    /// !=
    Ne,
    /// <
    Lt,
    /// >
    Gt,
    /// <=
    Le,
    /// >=
    Ge,
    /// ===
    Identical,
    /// !==
    NotIdentical,

    /// &&
    LogicalAnd,
    /// ||
    LogicalOr,

    /// &
    BitwiseAnd,
    /// |
    BitwiseOr,
    /// &=
    BitwiseAndAssign,
    /// |=
    BitwiseOrAssign,
    /// ^
    Xor,
    /// ^=
    XorAssign,
    /// <<
    Shl,
    /// >>
    Shr,
    /// <<=
    ShlAssign,
    /// >>=
    ShrAssign,

    /// =~
    Find,
    /// ==~
    Match,

    /// ~=
    BitwiseNotAssign,

    /// ,
    Comma,

    // TODO: lex everything beyond this point
    /// <<<
    UnsignedShl,
    /// <<<=
    UnsignedShlAssign,
    /// >>>
    UnsignedShr,
    /// >>>=
    UnsignedShrAssign,

    /// ?
    QuestionMark,

    /// :
    Colon,

    /// ?:
    Elvis,
    /// ?=
    ElvisAssignment,

    /// ?.
    SafeNavigation,

    /// @.
    DirectFieldAccess,

    /// .&
    MethodPointer,

    /// ::
    MethodReference,

    /// *.
    Spread,

    /// ..
    ExclusiveRange,
    /// ..<
    InclusiveRange,

    /// <=>
    Spaceship,

    /// <>
    Diamond,

    /// @
    AtSign,
}

#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    String(StringLiteral<'a>),
    Number(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum StringLiteral<'a> {
    Uninterpolated(&'a str),
    Interpolated(Vec<InterpolatedStringPart<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum InterpolatedStringPart<'a> {
    Literal(&'a str),
    Identifier(Identifier),
    Expression(Vec<Token<'a>>),
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
pub enum Visibility {
    PackagePrivate,
    Private,
    Protected,
    Undefined,
    Public,
}

#[derive(Debug, PartialEq)]
pub enum ClassModifier {
    Final,
    Static,
}

#[derive(Debug, PartialEq)]
pub enum MethodModifier {
    Final,
    Static,
    Abstract,
    Transient,
    Synchronized,
    Volatile,
}

#[derive(Debug)]
pub enum Type {
    /// Exists just to get things to compile
    Placeholder,
}
