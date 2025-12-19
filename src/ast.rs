use std::fmt;

// We can reuse the TokenValue from your lexer, or define a specific Literal type.
// For simplicity, let's reuse the one you already have if it's public,
// otherwise, we mirror it here.
use crate::parser::TokenValue;

/// The root node of the AST.
#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Statements do things but don't return values (usually).
#[derive(Debug)]
pub enum Statement {
    Return(Option<Expression>),
    // Future expansion:
    // Let(String, Expression),
    // If(Expression, Box<Statement>, Option<Box<Statement>>),
    // ExpressionStmt(Expression),
}

/// Binary operators.
#[derive(Debug)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
}

/// Expressions evaluate to a value.
#[derive(Debug)]
pub enum Expression {
    Literal(TokenValue),
    Binary(Box<Expression>, BinaryOp, Box<Expression>),
}

// --- Display Implementations for pretty printing ---

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Return(Some(expr)) => write!(f, "Return({})", expr),
            Statement::Return(None) => write!(f, "Return(Void)"),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(val) => write!(f, "{:?}", val),
            Expression::Binary(left, op, right) => {
                let op_str = match op {
                    BinaryOp::Plus => "+",
                    BinaryOp::Minus => "-",
                    BinaryOp::Multiply => "*",
                    BinaryOp::Divide => "/",
                };
                write!(f, "({} {} {})", left, op_str, right)
            }
        }
    }
}
