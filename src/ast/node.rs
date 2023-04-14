use std::fmt::{Display, self};

use crate::compiler::parser::Token;

pub trait SList: Display {}

pub trait Stmt: SList {

}

pub trait Expr: SList {

}

pub struct Block {
    pub stmts: Vec<Box<dyn Stmt>>
}

impl Block {
    pub fn new() -> Self {
        Self {
            stmts: vec![]
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start>( ")?;
        for i in &self.stmts {
            write!(f, "{}, ", i)?;
        }
        write!(f, ") ")?;
        Ok(())
    }
}

pub struct Band {

    pub hash_sign: Token,
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub value: Box<dyn Expr>
}

impl Display for Band {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "( # {} (", self.name)?;

        for i in &self.args {
            write!(f, " {} ", i)?;
        }

        write!(f, ") = {} )", self.value)?;
        Ok(())
    }
}

impl SList for Band {}

impl Stmt for Band {}

impl Band {
    
}

pub struct Identifier {
    pub literal: Token,
    pub value: String, 
}

impl Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)?;       
        Ok(())
    }
}

pub struct Expression {}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")?;
        Ok(())
    }
}
impl SList for Expression {}
impl Expr for Expression {}