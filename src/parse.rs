use std::iter::Peekable;

use crate::{
    ast::{AstNode, BinaryOperator, ClassModifier, Identifier, Literal, Token, Visibility},
    interner::keywords::Keywords,
    lexer::GroovyLexer,
};

pub struct GroovyParser<'a> {
    lexer: Peekable<GroovyLexer<'a>>,
}

impl<'a> GroovyParser<'a> {
    pub fn new(input: &'a str) -> GResult<Vec<AstNode>> {
        let mut parser = Self {
            lexer: GroovyLexer::new(input).peekable(),
        };

        Ok(vec![parser.parse_toplevel()?])
    }
}

#[derive(Debug)]
pub struct GroovyError {}

impl GroovyError {
    pub fn new() -> Self {
        Self {}
    }
}

type GResult<T> = Result<T, GroovyError>;

impl<'a> GroovyParser<'a> {
    fn parse_toplevel(&mut self) -> GResult<AstNode> {
        let visibility = self.parse_visibility();
        let modifiers = self.parse_class_modifiers();
        match self.lexer.next() {
            Some(Token::Identifier(ident)) if Keywords::Class == ident => {
                return self.parse_class_declaration(visibility, modifiers);
            }
            Some(Token::Identifier(ident)) if Keywords::Interface == ident => {
                todo!("interface declaration")
            }
            Some(Token::Identifier(ident)) if Keywords::Enum == ident => todo!("enum declaration"),
            Some(Token::AtSign) => todo!("attribute declaration"),
            _ => return Err(GroovyError::new()),
        }
    }

    fn parse_visibility(&mut self) -> Visibility {
        if let Some(Token::Identifier(ident)) = self.lexer.peek() {
            let ident = *ident;
            self.lexer.next();
            if Keywords::Public == ident {
                return Visibility::Public;
            } else if Keywords::Private == ident {
                return Visibility::Private;
            } else if Keywords::Protected == ident {
                return Visibility::Protected;
            }
        }
        Visibility::Undefined
    }

    fn parse_class_modifiers(&mut self) -> Vec<ClassModifier> {
        let mut modifiers = Vec::new();
        while let Some(Token::Identifier(ident)) = self.lexer.peek() {
            if Keywords::Final == *ident {
                self.lexer.next();
                modifiers.push(ClassModifier::Final)
            } else if Keywords::Static == *ident {
                self.lexer.next();
                modifiers.push(ClassModifier::Static)
            } else {
                break;
            }
        }
        modifiers
    }

    fn parse_class_declaration(
        &mut self,
        visibility: Visibility,
        modifiers: Vec<ClassModifier>,
    ) -> GResult<AstNode> {
        let name = if let Some(Token::Identifier(ident)) = self.lexer.next() {
            ident
        } else {
            return Err(GroovyError::new());
        };

        todo!()
    }

    fn parse_method_declaration(&mut self) -> GResult<AstNode> {
        todo!()
    }

    fn parse_variable_declaration(&mut self) -> GResult<AstNode> {
        todo!()
    }

    fn expect_keyword(&mut self, k: Keywords) -> GResult<()> {
        if let Some(Token::Identifier(ident)) = self.lexer.next() {
            if k == ident {
                return Ok(());
            }
        }
        Err(GroovyError::new())
    }
}
