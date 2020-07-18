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

    /// <<<=
    UnsignedShlAssign,

    /// >>>=
    UnsignedShrAssign,

    /// ~=
    BitwiseNotAssign,

    /// ?=
    ElvisAssignment,
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
    Equal,

    /// !=
    NotEqual,

    /// <
    LessThan,

    /// >
    GreaterThan,

    /// <=
    LessThanOrEqual,

    /// >=
    GreaterThanOrEqual,

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

    /// <<<
    UnsignedShl,

    /// >>>
    UnsignedShr,

    /// =~
    Find,

    /// ==~
    Match,

    /// <=>
    Diamond,

    /// in
    In,

    /// !in
    NotIn,

    /// instanceof
    InstanceOf,

    /// !instanceof
    NotInstanceOf,

    /// as
    As,

    /// ..<
    ExclusiveRange,

    /// ..
    InclusiveRange,

    /// ?.
    SafeNavigation,

    /// .@
    DirectFieldAccess,

    /// .&
    MethodPointer,

    /// ::
    MethodReference,

    /// *.
    SpreadDot,

    /// *:
    SpreadMap,
}

impl BinaryOperator {
    pub fn precendence(&self) -> (u8, u8) {
        match self {
            BinaryOperator::LogicalOr => (1, 2),
            BinaryOperator::LogicalAnd => (3, 4),
            BinaryOperator::BitwiseOr => (5, 6),
            BinaryOperator::Xor => (7, 8),
            BinaryOperator::BitwiseAnd => (9, 10),
            BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::Identical
            | BinaryOperator::NotIdentical
            | BinaryOperator::Find
            | BinaryOperator::Match
            | BinaryOperator::Diamond => (11, 12),
            BinaryOperator::LessThan
            | BinaryOperator::LessThanOrEqual
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanOrEqual
            | BinaryOperator::In
            | BinaryOperator::NotIn
            | BinaryOperator::InstanceOf
            | BinaryOperator::NotInstanceOf
            | BinaryOperator::As => (13, 14),
            BinaryOperator::Shl
            | BinaryOperator::Shr
            | BinaryOperator::ExclusiveRange
            | BinaryOperator::InclusiveRange
            | BinaryOperator::UnsignedShl
            | BinaryOperator::UnsignedShr => (15, 16),
            BinaryOperator::Add | BinaryOperator::Sub => (17, 18),
            BinaryOperator::Mul | BinaryOperator::Div | BinaryOperator::Rem => (19, 20),
            BinaryOperator::Pow => (21, 22),
            BinaryOperator::SafeNavigation
            | BinaryOperator::DirectFieldAccess
            | BinaryOperator::MethodPointer
            | BinaryOperator::MethodReference
            | BinaryOperator::SpreadDot
            | BinaryOperator::SpreadMap => (23, 24),
        }
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    /// !
    LogicalNot,

    /// ~
    BitwiseNot,

    /// *
    Spread,

    /// +
    Plus,

    /// -
    Minus,
}
