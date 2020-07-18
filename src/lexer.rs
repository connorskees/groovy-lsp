use std::convert::TryFrom;

use crate::{
    ast::{Identifier, Literal, Token},
    interner::keywords::Keywords,
};

// todo: we must store the last token in order to resolve ambiguities with
// `/` (is it a slashy-regex string or division?)
pub(crate) struct GroovyLexer<'a> {
    input: &'a str,
    input_len: usize,
    start: usize,
    pos: usize,
}

impl<'a> GroovyLexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            input: s,
            input_len: s.len(),
            start: 0,
            pos: 0,
        }
    }

    pub fn lex(&mut self) -> Token<'a> {
        self.whitespace();
        match self.next_char() {
            Some(
                'a'..='z'
                | 'A'..='Z'
                | '\u{00C0}'..='\u{00D6}'
                | '\u{00D8}'..='\u{00F6}'
                | '\u{00F8}'..='\u{00FF}'
                | '\u{0100}'..='\u{FFFE}'
                | '_',
            ) => self.lex_identifier(),
            Some('0'..='9') => self.lex_number(),
            Some('+') => self.lex_plus(),
            Some('-') => self.lex_minus(),
            Some('*') => self.lex_mul(),
            Some('/') => self.lex_div(),
            Some('%') => self.lex_percent(),
            Some('=') => self.lex_equal(),
            Some('!') => self.lex_exclamation(),
            Some('<') => self.lex_less_than(),
            Some('>') => self.lex_greater_than(),
            Some('&') => self.lex_ampersand(),
            Some('|') => self.lex_pipe(),
            Some('^') => self.lex_caret(),
            Some('~') => self.lex_tilde(),
            Some('.') => Token::Period,
            Some('(') => Token::ParenOpen,
            Some(')') => Token::ParenClose,
            Some('{') => Token::CurlyBraceOpen,
            Some('}') => Token::CurlyBraceClose,
            Some('[') => Token::SquareBraceOpen,
            Some(']') => Token::SquareBraceClose,
            Some(',') => Token::Comma,
            Some('\'') => todo!("lex single quote"),
            Some('"') => todo!("lex double quote"),
            Some('?') => todo!("lex question mark"),
            Some(':') => todo!("lex colon"),
            Some('$') => todo!("could be start of identifier *or* string, `$/.../$`"),
            Some(';') => Token::ExprEnd,
            c => todo!("lex {:?}", c),
        }
    }

    /// 3.1. Normal identifiers
    /// Identifiers start with a letter, a dollar or an underscore. They cannot start with a number.
    ///
    /// A letter can be in the following ranges:
    ///
    ///     'a' to 'z' (lowercase ascii letter)
    ///
    ///     'A' to 'Z' (uppercase ascii letter)
    ///
    ///     '\u{00C0}' to '\u{00D6}'
    ///
    ///     '\u{00D8}' to '\u{00F6}'
    ///
    ///     '\u{00F8}' to '\u{00FF}'
    ///
    ///     '\u{0100}' to '\u{FFFE}'
    ///
    /// Then following characters can contain letters and numbers.
    fn lex_identifier(&mut self) -> Token<'a> {
        while let Some(
            'a'..='z'
            | 'A'..='Z'
            | '\u{00C0}'..='\u{00D6}'
            | '\u{00D8}'..='\u{00F6}'
            | '\u{00F8}'..='\u{00FF}'
            | '\u{0100}'..='\u{FFFE}'
            | '_'
            | '0'..='9',
        ) = self.peek_char()
        {
            self.next_char();
        }

        match Keywords::try_from(Identifier::new(&self.input[self.start..self.pos])) {
            Ok(keyword) => Token::Keyword(keyword),
            Err(ident) => Token::Identifier(ident),
        }
    }

    fn lex_plus(&mut self) -> Token<'a> {
        if self.peek_char() == Some('=') {
            self.next_char();
            Token::AddAssign
        } else {
            Token::Add
        }
    }

    fn lex_minus(&mut self) -> Token<'a> {
        if self.peek_char() == Some('=') {
            self.next_char();
            Token::SubAssign
        } else {
            Token::Sub
        }
    }

    fn lex_mul(&mut self) -> Token<'a> {
        match self.peek_char() {
            Some('=') => {
                self.next_char();
                Token::MulAssign
            }
            Some('*') => {
                self.next_char();
                if self.peek_char() == Some('=') {
                    self.next_char();
                    Token::PowAssign
                } else {
                    Token::Pow
                }
            }
            _ => Token::Mul,
        }
    }

    fn lex_div(&mut self) -> Token<'a> {
        match self.peek_char() {
            Some('=') => {
                self.next_char();
                Token::DivAssign
            }
            Some('*') => todo!("multiline comments"),
            Some('/') => {
                while let Some(tok) = self.next_char() {
                    if tok == '\n' {
                        break;
                    }
                }
                self.lex()
            }
            _ => Token::Div,
        }
    }

    fn lex_percent(&mut self) -> Token<'a> {
        if self.peek_char() == Some('=') {
            self.next_char();
            Token::RemAssign
        } else {
            Token::Rem
        }
    }

    fn lex_ampersand(&mut self) -> Token<'a> {
        match self.peek_char() {
            Some('&') => {
                self.next_char();
                Token::LogicalAnd
            }
            Some('=') => {
                self.next_char();
                Token::BitwiseAndAssign
            }
            _ => Token::BitwiseAnd,
        }
    }

    fn lex_pipe(&mut self) -> Token<'a> {
        match self.peek_char() {
            Some('|') => {
                self.next_char();
                Token::LogicalOr
            }
            Some('=') => {
                self.next_char();
                Token::BitwiseOrAssign
            }
            _ => Token::BitwiseOr,
        }
    }

    fn lex_number(&mut self) -> Token<'a> {
        while let Some('0'..='9') = self.peek_char() {
            self.next_char();
        }
        Token::Literal(Literal::Number(&self.input[self.start..self.pos]))
    }

    fn lex_exclamation(&mut self) -> Token<'a> {
        if self.peek_char() == Some('=') {
            self.next();
            if self.peek_char() == Some('=') {
                self.next();
                Token::NotIdentical
            } else {
                Token::Ne
            }
        } else {
            Token::LogicalNot
        }
    }

    fn lex_caret(&mut self) -> Token<'a> {
        if self.peek_char() == Some('=') {
            self.next_char();
            Token::XorAssign
        } else {
            Token::Xor
        }
    }

    fn lex_tilde(&mut self) -> Token<'a> {
        if self.peek_char() == Some('=') {
            self.next_char();
            Token::BitwiseNotAssign
        } else {
            Token::Tilde
        }
    }

    fn lex_equal(&mut self) -> Token<'a> {
        match self.peek_char() {
            Some('=') => {
                self.next_char();
                match self.peek_char() {
                    Some('=') => {
                        self.next_char();
                        Token::Identical
                    }
                    Some('~') => {
                        self.next_char();
                        Token::Match
                    }
                    _ => Token::Eq,
                }
            }
            Some('~') => {
                self.next_char();
                Token::Find
            }
            _ => Token::SingleEqual,
        }
    }

    fn lex_less_than(&mut self) -> Token<'a> {
        match self.peek_char() {
            Some('<') => {
                self.next_char();
                if self.peek_char() == Some('=') {
                    self.next_char();
                    Token::ShlAssign
                } else {
                    Token::Shl
                }
            }
            Some('=') => {
                self.next_char();
                Token::Le
            }
            _ => Token::Lt,
        }
    }

    fn lex_greater_than(&mut self) -> Token<'a> {
        match self.peek_char() {
            Some('>') => {
                self.next_char();
                if self.peek_char() == Some('=') {
                    self.next_char();
                    Token::ShrAssign
                } else {
                    Token::Shr
                }
            }
            Some('=') => {
                self.next_char();
                Token::Ge
            }
            _ => Token::Gt,
        }
    }

    fn whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_ascii_whitespace() {
                self.next_char();
            } else {
                self.start = self.pos;
                return;
            }
        }
        self.start = self.pos;
    }

    fn next_char(&mut self) -> Option<char> {
        if self.pos > self.input_len {
            return None;
        }
        if let Some(c) = self.input[self.pos..].chars().next() {
            self.pos += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.pos > self.input_len {
            return None;
        }
        self.input[self.pos..].chars().next()
    }
}

impl<'a> Iterator for GroovyLexer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.pos;
        Some(self.lex())
    }
}
