use std::iter::Peekable;

use crate::{
    ast::{AstNode, BinaryOperator, ClassModifier, Identifier, Literal, Token},
    interner::keywords::Keywords,
    lexer::GroovyLexer,
};

struct GroovyParser<'a> {
    lexer: &'a mut Peekable<GroovyLexer<'a>>,
}

struct GroovyError {}

impl GroovyError {
    pub fn new() -> Self {
        Self {}
    }
}

type GResult<T> = Result<T, GroovyError>;

impl<'a> GroovyParser<'a> {
    fn parse_toplevel(&mut self) -> GResult<Vec<AstNode>> {
        let modifiers = self.parse_class_modifiers()?;
        match self.lexer.next() {
            Some(Token::Identifier(ident)) if Keywords::Class == ident => {
                todo!("class declaration")
            }
            Some(Token::Identifier(ident)) if Keywords::Interface == ident => {
                todo!("interface declaration")
            }
            Some(Token::Identifier(ident)) if Keywords::Enum == ident => todo!("enum declaration"),
            Some(Token::AtSign) => todo!("attribute declaration"),
            _ => return Err(GroovyError::new()),
        }
        todo!()
    }

    fn parse_class_modifiers(&mut self) -> GResult<Vec<ClassModifier>> {
        todo!()
    }

    fn parse_class_declaration(&mut self) -> GResult<AstNode> {
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
