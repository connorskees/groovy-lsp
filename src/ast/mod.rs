mod decimal;
mod integer;

use crate::interner::Symbol;

#[allow(dead_code)]
pub enum AstNode {
    Annotated,
    ClassCodeVisitorSupport,
    Class,
    Constructor,
    Field,
    Import,
    Method,
    Module,
    Parameter,
    Property,
}

#[allow(dead_code)]
pub enum Expr {
    Array,
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
    Variable,
}

#[allow(dead_code)]
pub enum Stmt {
    Assert,
    Block,
    Break,
    Case,
    Catch,
    Continue,
    DoWhile,
    Empty,
    Expression(Expr),
    For,
    If,
    Return,
    Switch,
    Synchronized,
    Throw,
    TryCatch,
    While,
}

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

pub enum UnaryOperator {
    /// !
    LogicalNot,
    /// ~
    BitwiseNot,
}

pub enum Token<'a> {
    Literal(Literal<'a>),
    Identifier(Identifier),
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
}

pub enum Literal<'a> {
    String(StringLiteral<'a>),
    Number(&'a str),
}

pub enum StringLiteral<'a> {
    Uninterpolated(&'a str),
    Interpolated(Vec<InterpolatedStringPart<'a>>),
}

pub enum InterpolatedStringPart<'a> {
    Literal(&'a str),
    Identifier(Identifier),
    Expression(Vec<Token<'a>>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
