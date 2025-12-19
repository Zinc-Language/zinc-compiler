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
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    Block(Vec<Statement>),
    Declare(String, Type, Expression),
    // Future expansion:
    // Let(String, Expression),
    // ExpressionStmt(Expression),
}

/// Binary operators.
#[derive(Debug)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    NotAnd,
    Or,
    NotOr,
    Xor,
    NotXor,
    ShiftLeft,
    ShiftRight,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
}

#[derive(Debug)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
}

/// Expressions evaluate to a value.
#[derive(Debug)]
pub enum Expression {
    Literal(TokenValue),
    Identifier(String),
    Binary(Box<Expression>, BinaryOp, Box<Expression>),
}

// --- Display Implementations for pretty printing ---

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("Zinc Program");
        for (i, stmt) in self.statements.iter().enumerate() {
            let is_last = i == self.statements.len() - 1;
            // Pass an empty label "" for top-level statements
            stmt.print_tree("", is_last, "", f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::I8 => write!(f, "i8"),
            Type::I16 => write!(f, "i16"),
            Type::I32 => write!(f, "i32"),
            Type::I64 => write!(f, "i64"),
            Type::U8 => write!(f, "u8"),
            Type::U16 => write!(f, "u16"),
            Type::U32 => write!(f, "u32"),
            Type::U64 => write!(f, "u64"),
            Type::F32 => write!(f, "f32"),
            Type::F64 => write!(f, "f64"),
            Type::Bool => write!(f, "bool"),
        }
    }
}

impl Statement {
    // CHANGED: Added `label` parameter to match Expression's style
    fn print_tree(
        &self,
        prefix: &str,
        is_last: bool,
        label: &str,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let marker = if is_last { "└── " } else { "├── " };

        // Format the label if it exists (e.g., "Then: ")
        let label_prefix = if label.is_empty() {
            String::new()
        } else {
            format!("{}: ", label)
        };

        match self {
            Statement::Return(opt_expr) => {
                // Print: └── Then: Statement::Return
                writeln!(f, "{}{}{}Statement::Return", prefix, marker, label_prefix)?;

                if let Some(expr) = opt_expr {
                    let child_prefix =
                        format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                    expr.print_tree(&child_prefix, true, "Value", f)?;
                }
            }
            // NEW: Implementation for If
            Statement::If(condition, then_branch, else_branch) => {
                // Print: └── Statement::If
                writeln!(f, "{}{}{}Statement::If", prefix, marker, label_prefix)?;

                let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

                // 1. Print Condition
                // It is never the last child because a "Then" block always follows
                condition.print_tree(&child_prefix, false, "Condition", f)?;

                // 2. Print Then Branch
                // It is last ONLY if there is no Else branch
                let has_else = else_branch.is_some();
                then_branch.print_tree(&child_prefix, !has_else, "Then", f)?;

                // 3. Print Else Branch (Optional)
                if let Some(else_stmt) = else_branch {
                    else_stmt.print_tree(&child_prefix, true, "Else", f)?;
                }
            }
            Statement::Block(statements) => {
                writeln!(f, "{}{}{}Statement::Block", prefix, marker, label_prefix)?;

                // Iterate through statements in the block
                for (i, stmt) in statements.iter().enumerate() {
                    let is_stmt_last = i == statements.len() - 1;
                    let child_prefix =
                        format!("{}{}", prefix, if is_last { "    " } else { "│   " });

                    // We don't need a label for generic statements in a block
                    stmt.print_tree(&child_prefix, is_stmt_last, "", f)?;
                }
            }
            Statement::Declare(name, ty, expr) => {
                // Print: └── Statement::Declare
                writeln!(f, "{}{}{}Statement::Declare", prefix, marker, label_prefix)?;

                let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

                // 1. Print Type
                writeln!(f, "{}{}{}Type: {}", child_prefix, marker, label_prefix, ty)?;

                // 2. Print Name
                writeln!(
                    f,
                    "{}{}{}Name: {}",
                    child_prefix, marker, label_prefix, name
                )?;

                // 3. Print Expression
                expr.print_tree(&child_prefix, true, "Value", f)?;
            }
        }
        Ok(())
    }
}

impl Expression {
    fn print_tree(
        &self,
        prefix: &str,
        is_last: bool,
        label: &str,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        let marker = if is_last { "└── " } else { "├── " };

        match self {
            Expression::Literal(val) => {
                // Print: └── Label: Expression::Literal(10)
                writeln!(
                    f,
                    "{}{}{}: Expression::Literal({:?})",
                    prefix, marker, label, val
                )?;
            }
            Expression::Identifier(name) => {
                writeln!(
                    f,
                    "{}{}{}: Expression::Identifier({})",
                    prefix, marker, label, name
                )?;
            }
            Expression::Binary(left, op, right) => {
                let op_str = match op {
                    BinaryOp::Plus => "Plus",
                    BinaryOp::Minus => "Minus",
                    BinaryOp::Multiply => "Multiply",
                    BinaryOp::Divide => "Divide",
                    BinaryOp::Modulo => "Modulo",
                    BinaryOp::Power => "Power",

                    // Comparison
                    BinaryOp::Equal => "Equal",
                    BinaryOp::NotEqual => "NotEqual",
                    BinaryOp::LessThan => "LessThan",
                    BinaryOp::LessThanOrEqual => "LessThanOrEqual",
                    BinaryOp::GreaterThan => "GreaterThan",
                    BinaryOp::GreaterThanOrEqual => "GreaterThanOrEqual",

                    // Logical
                    BinaryOp::And => "And",
                    BinaryOp::Or => "Or",

                    // Catch-all for others you haven't implemented parsing for yet
                    _ => "UnknownOperator",
                };

                // Print: └── Label: Expression::Binary (Plus)
                writeln!(
                    f,
                    "{}{}{}: Expression::Binary ({})",
                    prefix, marker, label, op_str
                )?;

                // Prepare prefix for children
                let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

                // Print Left Child (is_last = false because Right comes after)
                left.print_tree(&child_prefix, false, "Left", f)?;

                // Print Right Child (is_last = true)
                right.print_tree(&child_prefix, true, "Right", f)?;
            }
        }
        Ok(())
    }
}
