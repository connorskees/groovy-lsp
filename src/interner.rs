// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! An "interner" is a data structure that associates values with usize tags and
//! allows bidirectional lookup; i.e. given a value, one can easily find the
//! type, and vice versa.
//!
//! Borrowed from rustc

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

/// A symbol is an interned or gensymed string.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(u32);

// The interner in thread-local, so `Symbol` shouldn't move between threads.
impl !Send for Symbol {}

impl Symbol {
    /// Maps a string to its interned representation.
    pub fn intern(string: &str) -> Self {
        with_interner(|interner| interner.intern(string))
    }

    /// gensym's a new usize, using the current interner.
    pub fn gensym(string: &str) -> Self {
        with_interner(|interner| interner.gensym(string))
    }

    pub fn as_str(self) -> InternedString {
        with_interner(|interner| unsafe {
            InternedString {
                string: ::std::mem::transmute::<&str, &str>(interner.get(self)),
            }
        })
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self, self.0)
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.as_str(), f)
    }
}

impl<'a> PartialEq<&'a str> for Symbol {
    fn eq(&self, other: &&str) -> bool {
        *self.as_str() == **other
    }
}

#[derive(Default)]
pub struct Interner {
    names: HashMap<Box<str>, Symbol>,
    strings: Vec<Box<str>>,
}

impl Interner {
    pub fn new() -> Self {
        Interner::default()
    }

    fn prefill(init: &[&str]) -> Self {
        let mut this = Interner::new();
        for &string in init {
            this.intern(string);
        }
        this
    }

    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(&name) = self.names.get(string) {
            return name;
        }

        let name = Symbol(self.strings.len() as u32);
        let string = string.to_string().into_boxed_str();
        self.strings.push(string.clone());
        self.names.insert(string, name);
        name
    }

    fn gensym(&mut self, string: &str) -> Symbol {
        let gensym = Symbol(self.strings.len() as u32);
        // leave out of `names` to avoid colliding
        self.strings.push(string.to_string().into_boxed_str());
        gensym
    }

    pub fn get(&self, name: Symbol) -> &str {
        &self.strings[name.0 as usize]
    }
}

// In this macro, there is the requirement that the name (the number) must be monotonically
// increasing by one in the special identifiers, starting at 0; the same holds for the keywords,
// except starting from the next number instead of zero.
macro_rules! declare_keywords {(
    $( ($index: expr, $konst: ident, $string: expr) )*
) => {
    pub mod keywords {
        use crate::ast;
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct Keyword {
            ident: ast::Identifier,
        }
        impl Keyword {
            #[inline] pub fn ident(self) -> ast::Identifier { self.ident }
        }
        $(
            #[allow(non_upper_case_globals)]
            pub const $konst: Keyword = Keyword {
                ident: ast::Identifier { name: $crate::interner::Symbol($index) }
            };
        )*
    }

    impl Interner {
        fn fresh() -> Self {
            Interner::prefill(&[$($string,)*])
        }
    }
}}

// NB: leaving holes in the ident table is bad! a different ident will get
// interned with the id from the hole, but it will be between the min and max
// of the reserved words, and thus tagged as "reserved".
// After modifying this list adjust `is_strict_keyword`/`is_reserved_keyword`,
// this should be rarely necessary though if the keywords are kept in alphabetic order.
declare_keywords! {
    // Invalid identifier
    (0,  Invalid,        "")

    // Strict keywords used in the language.
    (1,  Abstract,             "abstract")
    (2,  As,            "as")
    (3,  Assert,          "assert")
    (4,  Boolean,          "boolean")
    (5,  Break,       "break")
    (6,  Byte,          "byte")
    (7,  Case,           "case")
    (8,  Catch,           "catch")
    (9,  Char,         "char")
    (10, Class,          "class")
    (11, Const,             "const")
    (12, Continue,            "continue")
    (13, Def,             "def")
    (14, Default,           "default")
    (15, Do,             "do")
    (16, Double,            "double")
    (17, Else,           "else")
    (18, Enum,          "enum")
    (19, Extends,            "extends")
    (20, False,           "false")
    (21, Final,            "final")
    (22, Finally,            "finally")
    (23, Float,            "float")
    (24, For,         "for")
    (25, Goto,      "goto")
    (26, If,       "if")
    (27, Implements,         "implements")
    (28, Import,         "import")
    (29, In,          "in")
    (30, InstanceOf,          "instance_of")
    (31, Int,           "int")
    (32, Interface,           "interface")
    (33, Long,         "long")
    (34, Native,            "native")
    (35, New,          "new")
    (36, Null,          "null")
    (37, Packafe,       "packafe")
    (38, Private,        "private")
    (39, Protected,         "protected")
    (40, Public,             "public")
    (41, Return,          "return")
    (42, Short,          "short")
    (43, Static,       "static")
    (44, StrictFp,       "strict_fp")
    (45, Super,           "super")
    (46, Switch,           "switch")
    (47, Synchronized,           "synchronized")
    (48, This,         "this")
    (49, ThreadSafe,         "thread_safe")
    (50, Throw,        "throw")
    (51, Throws,        "throws")
    (52, Transient,          "transient")
    (53, True,        "true")
    (54, Try, "try")
    (55, Void,          "void")
    (56, Volatile,          "volatile")
    (56, While,          "while")
}

// If an interner exists in TLS, return it. Otherwise, prepare a fresh one.
fn with_interner<T, F: FnOnce(&mut Interner) -> T>(f: F) -> T {
    thread_local!(static INTERNER: RefCell<Interner> = {
        RefCell::new(Interner::fresh())
    });
    INTERNER.with(|interner| f(&mut *interner.borrow_mut()))
}

/// Represents a string stored in the thread-local interner. Because the
/// interner lives for the life of the thread, this can be safely treated as an
/// immortal string, as long as it never crosses between threads.
///
/// FIXME(pcwalton): You must be careful about what you do in the destructors
/// of objects stored in TLS, because they may run after the interner is
/// destroyed. In particular, they must not access string contents. This can
/// be fixed in the future by just leaking all strings until thread death
/// somehow.
#[derive(Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
pub struct InternedString {
    string: &'static str,
}

impl !Send for InternedString {}

impl ::std::ops::Deref for InternedString {
    type Target = str;
    fn deref(&self) -> &str {
        self.string
    }
}

impl fmt::Debug for InternedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.string, f)
    }
}

impl fmt::Display for InternedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.string, f)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::ast::Name;

    #[test]
    fn interner_tests() {
        // let mut i: Interner = Interner::new();
        // // first one is zero:
        // assert_eq!(i.intern("dog"), Name(0));
        // // re-use gets the same entry:
        // assert_eq!(i.intern("dog"), Name(0));
        // // different string gets a different #:
        // assert_eq!(i.intern("cat"), Name(1));
        // assert_eq!(i.intern("cat"), Name(1));
        // // dog is still at zero
        // assert_eq!(i.intern("dog"), Name(0));
        // // gensym gets 3
        // assert_eq!(i.gensym("zebra"), Name(2));
        // // gensym of same string gets new number :
        // assert_eq!(i.gensym("zebra"), Name(3));
        // // gensym of *existing* string gets new number:
        // assert_eq!(i.gensym("dog"), Name(4));
    }
}
