#[derive(Debug, PartialEq)]
pub enum ClassModifier {
    /// Declaration cannot be overridden
    Final,
    Static,
}

#[derive(Debug, PartialEq)]
pub enum MethodModifier {
    /// Declaration cannot be overridden
    Final,
    Static,
    /// Body will be missing
    Abstract,
    /// Property should not be persisted
    Transient,
    Synchronized,
    /// Compiler should never cache property
    Volatile,
    /// A native code entrypoint
    Native,
}
