use std::iter::Peekable;

use crate::{
    ast::{AstNode, BinaryOperator, Identifier, Literal, Token},
    lexer::GroovyLexer,
};

struct GroovyParser<'a> {
    lexer: &'a mut Peekable<GroovyLexer<'a>>,
}

impl<'a> GroovyParser<'a> {
    fn parse_toplevel(&mut self) -> Vec<AstNode> {
        todo!()
    }
}
