use std::{convert::TryFrom, iter::Peekable};

use crate::{
    ast::{
        AstNode, BinaryOperator, Class, ClassModifier, Expr, Identifier, Literal, Method,
        MethodModifier, Parameter, Stmt, Token, Type, Visibility,
    },
    interner::keywords::Keywords,
    lexer::GroovyLexer,
};

const TODO_BOOL: bool = false;

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
pub struct GroovyError {
    message: &'static str,
}

impl GroovyError {
    pub fn new(message: &'static str) -> Self {
        Self { message }
    }
}

type GResult<T> = Result<T, GroovyError>;

impl<'a> GroovyParser<'a> {
    fn parse_toplevel(&mut self) -> GResult<AstNode> {
        let visibility = self.parse_visibility();
        let modifiers = self.parse_class_modifiers();
        match self.lexer.next() {
            Some(Token::Keyword(Keywords::Class)) => {
                return Ok(AstNode::Class(
                    self.parse_class_declaration(visibility, modifiers)?,
                ));
            }
            Some(Token::Keyword(Keywords::Interface)) => todo!("interface declaration"),
            Some(Token::Keyword(Keywords::Enum)) => todo!("enum declaration"),
            Some(Token::AtSign) => todo!("attribute declaration"),
            _ => return Err(GroovyError::new("unknown token at toplevel")),
        }
    }

    fn parse_visibility(&mut self) -> Visibility {
        if let Some(Token::Keyword(keyword)) = self.lexer.peek() {
            match keyword {
                Keywords::Public => {
                    self.lexer.next();
                    return Visibility::Public;
                }
                Keywords::Private => {
                    self.lexer.next();
                    return Visibility::Private;
                }
                Keywords::Protected => {
                    self.lexer.next();
                    return Visibility::Protected;
                }
                _ => {}
            }
        }
        Visibility::Undefined
    }

    fn parse_class_modifiers(&mut self) -> Vec<ClassModifier> {
        let mut modifiers = Vec::new();
        while let Some(Token::Keyword(keyword)) = self.lexer.peek() {
            match keyword {
                Keywords::Final => {
                    self.lexer.next();
                    modifiers.push(ClassModifier::Final)
                }
                Keywords::Static => {
                    self.lexer.next();
                    modifiers.push(ClassModifier::Static)
                }
                _ => break,
            }
        }
        modifiers
    }

    fn parse_method_modifiers(&mut self) -> Vec<MethodModifier> {
        let mut modifiers = Vec::new();
        while let Some(Token::Keyword(keyword)) = self.lexer.peek() {
            match keyword {
                Keywords::Final => {
                    self.lexer.next();
                    modifiers.push(MethodModifier::Final)
                }
                Keywords::Static => {
                    self.lexer.next();
                    modifiers.push(MethodModifier::Static)
                }
                Keywords::Abstract => {
                    self.lexer.next();
                    modifiers.push(MethodModifier::Abstract)
                }
                Keywords::Transient => {
                    self.lexer.next();
                    modifiers.push(MethodModifier::Transient)
                }
                Keywords::Synchronized => {
                    self.lexer.next();
                    modifiers.push(MethodModifier::Synchronized)
                }
                Keywords::Volatile => {
                    self.lexer.next();
                    modifiers.push(MethodModifier::Volatile)
                }
                _ => break,
            }
        }
        modifiers
    }

    fn parse_class_declaration(
        &mut self,
        visibility: Visibility,
        modifiers: Vec<ClassModifier>,
    ) -> GResult<Class> {
        let name = if let Some(Token::Identifier(ident)) = self.lexer.next() {
            ident
        } else {
            return Err(GroovyError::new("class missing name"));
        };

        self.expect_token(Token::CurlyBraceOpen)?;

        let mut methods = Vec::new();

        loop {
            let visibility = self.parse_visibility();
            let modifiers = self.parse_method_modifiers();
            let type_name = self.parse_type()?;
            let ident = self.expect_identifier()?;
            match self.lexer.peek() {
                Some(Token::ParenOpen) => methods
                    .push(self.parse_method_declaration(visibility, modifiers, type_name, ident)?),
                Some(Token::SingleEqual) => todo!("expr"),
                _ => todo!(),
            }

            match self.lexer.peek() {
                Some(Token::CurlyBraceClose) | None => {
                    self.lexer.next();
                    break;
                }
                Some(..) => continue,
            }
        }

        Ok(Class {
            name,
            visibility,
            modifiers,
            super_class: None,
            methods,
            fields: Vec::new(),
            properties: Vec::new(),
            interfaces: Vec::new(),
            mixin: Vec::new(),
        })
    }

    fn parse_method_declaration(
        &mut self,
        visibility: Visibility,
        modifiers: Vec<MethodModifier>,
        return_type: Type,
        name: Identifier,
    ) -> GResult<Method> {
        let parameters = self.parse_fn_args()?;
        let body = self.parse_block()?;

        Ok(Method {
            name,
            static_constructor: modifiers.contains(&MethodModifier::Static),
            visibility,
            modifiers,
            return_type,
            parameters,
            has_default_value: TODO_BOOL,
            exceptions: Vec::new(),
            body,
            is_constructor: TODO_BOOL,
        })
    }

    fn parse_fn_args(&mut self) -> GResult<Vec<Parameter>> {
        self.expect_token(Token::ParenOpen)?;

        let mut params = Vec::new();

        loop {
            let param_type = self.parse_type()?;
            let name = self.expect_identifier()?;
            if let Some(Token::SquareBraceOpen) = self.lexer.peek() {
                self.lexer.next();
                todo!("array params")
            }

            if let Some(Token::SingleEqual) = self.lexer.peek() {
                self.lexer.next();
                todo!("param with default values")
            }

            params.push(Parameter {
                param_type,
                name,
                origin_type: None,
                dynamically_typed: TODO_BOOL,
                closure_shared: TODO_BOOL,
                default_value: None,
                in_static_context: TODO_BOOL,
                modifiers: Vec::new(),
            });

            match self.lexer.peek() {
                Some(Token::ParenClose) | None => {
                    self.lexer.next();
                    break;
                }
                Some(Token::Comma) => continue,
                Some(..) => {
                    return Err(GroovyError::new("parameter had something other than comma"))
                }
            }
        }

        Ok(params)
    }

    fn expect_keyword(&mut self, k: Keywords) -> GResult<()> {
        if Some(Token::Keyword(k)) == self.lexer.next() {
            return Ok(());
        }
        Err(GroovyError::new("expected keyword"))
    }

    fn expect_token(&mut self, tok: Token) -> GResult<()> {
        if Some(tok) != self.lexer.next() {
            return Err(GroovyError::new("expected token"));
        }
        Ok(())
    }

    fn expect_identifier(&mut self) -> GResult<Identifier> {
        if let Some(Token::Identifier(ident)) = self.lexer.next() {
            return Ok(ident);
        }
        return Err(GroovyError::new("expected identifier"));
    }
}

impl GroovyParser<'_> {
    fn parse_type(&mut self) -> GResult<Type> {
        let initial_type = match self.lexer.peek() {
            Some(Token::Keyword(Keywords::Void)) => Type::Void,
            Some(Token::Keyword(Keywords::Int)) => Type::Int,
            Some(Token::Keyword(Keywords::Double)) => Type::Double,
            Some(Token::Keyword(Keywords::Float)) => Type::Float,
            Some(Token::Keyword(Keywords::Short)) => Type::Short,
            Some(Token::Keyword(Keywords::Char)) => Type::Char,
            Some(Token::Keyword(Keywords::Boolean)) => Type::Boolean,
            Some(Token::Keyword(Keywords::Byte)) => Type::Byte,
            Some(Token::Keyword(Keywords::Long)) => Type::Long,
            Some(Token::Identifier(ident)) => Type::Class(*ident),
            _ => return Err(GroovyError::new("expected type")),
        };
        self.lexer.next();
        if let Some(Token::SquareBraceOpen) = self.lexer.peek() {
            self.lexer.next();
            todo!("array type")
        }
        Ok(initial_type)
    }
}

impl GroovyParser<'_> {
    fn parse_expr(&mut self) -> GResult<Expr> {
        todo!()
    }
}

impl GroovyParser<'_> {
    fn parse_stmt(&mut self) -> GResult<Stmt> {
        match self.lexer.peek() {
            Some(Token::CurlyBraceOpen) => self.parse_block(),
            Some(Token::Keyword(Keywords::Assert)) => todo!("assert stmt"),
            Some(Token::Keyword(Keywords::Break)) => todo!("break stmt"),
            Some(Token::Keyword(Keywords::Class)) => todo!("class stmt"),
            Some(Token::Keyword(Keywords::Continue)) => todo!("continue stmt"),
            Some(Token::Keyword(Keywords::Return)) => todo!("return stmt"),
            Some(Token::Keyword(Keywords::Throw)) => todo!("throw stmt"),
            Some(Token::Keyword(Keywords::Try)) => todo!("try stmt"),
            Some(Token::Keyword(Keywords::For)) => todo!("for stmt"),
            Some(Token::Keyword(Keywords::Do)) => todo!("do .. while stmt"),
            Some(Token::Keyword(Keywords::While)) => todo!("while stmt"),
            Some(Token::Keyword(Keywords::Switch)) => todo!("switch stmt"),
            Some(Token::Keyword(Keywords::Synchronized)) => todo!("synchronized stmt"),
            Some(Token::Keyword(Keywords::If)) => todo!("if stmt"),
            _ => {
                if let Ok(type_name) = self.parse_type() {
                    self.parse_variable_declaration(type_name)
                } else if let Ok(expr) = self.parse_expr() {
                    Ok(Stmt::Expression(expr))
                } else {
                    return Err(GroovyError::new("expected stmt"));
                }
            }
        }
    }

    fn parse_block(&mut self) -> GResult<Stmt> {
        self.expect_token(Token::CurlyBraceOpen)?;

        let mut body = Vec::new();

        loop {
            if let Some(Token::CurlyBraceClose) | None = self.lexer.peek() {
                self.lexer.next();
                break;
            }
            body.push(self.parse_stmt()?);
        }

        Ok(Stmt::Block { body, scope: None })
    }

    fn parse_variable_declaration(&mut self, type_name: Type) -> GResult<Stmt> {
        let name = self.expect_identifier()?;
        self.expect_token(Token::SingleEqual)?;
        let value = self.parse_expr()?;
        todo!()
    }
}
