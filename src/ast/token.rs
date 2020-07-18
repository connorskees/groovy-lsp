use crate::{ast::Identifier, interner::keywords::Keywords};

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Literal(Literal<'a>),
    Identifier(Identifier),

    Keyword(Keywords),

    /// Marks the potential end to an expression
    ExprEnd,

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

    /// !in
    NotIn,

    /// !instanceof
    NotInstanceOf,

    /// \
    IntegerDivision,

    /// ++
    PlusPlus,

    /// --
    MinusMinus,
}

#[derive(Debug, PartialEq)]
pub enum Literal<'a> {
    String(&'a str),
    GString(GString<'a>),
    Number(&'a str),
    True,
    False,
    Null,
}

#[derive(Debug, PartialEq)]
pub struct GString<'a>(Vec<InterpolatedStringPart<'a>>);

#[derive(Debug, PartialEq)]
pub enum InterpolatedStringPart<'a> {
    Literal(&'a str),
    Identifier(Identifier),
    Expression(Vec<Token<'a>>),
}
