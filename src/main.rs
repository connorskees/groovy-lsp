#![feature(or_patterns)]
#![feature(negative_impls)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use std::fs;

use parse::GroovyParser;

mod ast;
mod interner;
mod lexer;
mod parse;

fn main() {
    let input = fs::read_to_string("test.groovy").unwrap();
    dbg!(GroovyParser::new(&input)).unwrap();
}
