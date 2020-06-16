pub(crate) enum Keyword {
    Abstract,
    As,
    Assert,
    Boolean,
    Break,
    Byte,
    Case,
    Catch,
    Char,
    Class,
    Const,
    Continue,
    Def,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extends,
    False,
    Final,
    Finally,
    Float,
    For,
    Goto,
    If,
    Implements,
    Import,
    In,
    InstanceOf,
    Int,
    Interface,
    Long,
    Native,
    New,
    Null,
    Packafe,
    Private,
    Protected,
    Public,
    Return,
    Short,
    Static,
    StrictFp,
    Super,
    Switch,
    Synchronized,
    This,
    ThreadSafe,
    Throw,
    Throws,
    Transient,
    True,
    Try,
    Void,
    Volatile,
    While,
}

pub(crate) enum BinaryOperator {
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
    /// ^
    Xor,
    /// <<
    Shl,
    /// >>
    Shr,

    /// =~
    Find,
    /// ==~
    Match,
}

pub(crate) enum UnaryOperator {
    /// !
    LogicalNot,
    /// ~
    BitwiseNot,
}