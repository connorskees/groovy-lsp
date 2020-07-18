#[derive(Debug, PartialEq)]
pub enum AssignmentOperator {
    /// +=
    AddAssign,

    /// -=
    SubAssign,

    /// *=
    MulAssign,

    /// /=
    DivAssign,

    /// %=
    RemAssign,

    /// **=
    PowAssign,

    /// &=
    BitwiseAndAssign,

    /// |=
    BitwiseOrAssign,

    /// ^=
    XorAssign,

    /// <<=
    ShlAssign,

    /// >>=
    ShrAssign,
}

#[derive(Debug)]
pub enum BinaryOperator {
    /// +
    Add,

    /// -
    Sub,

    /// *
    Mul,

    /// /
    Div,

    /// %
    Rem,

    /// **
    Pow,

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
