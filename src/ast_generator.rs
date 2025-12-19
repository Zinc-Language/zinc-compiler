use crate::ast::{BinaryOp, Expression, Program, Statement, Type};
use crate::parser::{Token, TokenType};

pub struct AstGenerator {
    tokens: Vec<Token>,
    current: usize,
}

impl AstGenerator {
    /// Creates a new AstGenerator from a vector of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        let filtered_tokens: Vec<Token> = tokens
            .into_iter()
            .filter(|t| {
                !matches!(
                    t.token_type,
                    TokenType::Whitespace
                        | TokenType::Newline
                        | TokenType::Comment
                        | TokenType::BlockComment
                        | TokenType::DocComment
                )
            })
            .collect();

        AstGenerator {
            tokens: filtered_tokens,
            current: 0,
        }
    }

    /// Parses the source code into an AST.
    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if self.match_token(&[
                TokenType::Newline,
                TokenType::Whitespace,
                TokenType::Comment,
                TokenType::DocComment,
            ]) {
                continue;
            }
            statements.push(self.statement());
        }
        Program { statements }
    }

    /// Helper to parse a list of statements inside braces
    fn block(&mut self) -> Statement {
        let mut statements = Vec::new();

        // Keep parsing statements until we hit the closing brace or EOF
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            // Filter noise (newlines/comments) if your loop inside parse_program doesn't handle it globally
            if self.match_token(&[
                TokenType::Newline,
                TokenType::Whitespace,
                TokenType::Comment,
            ]) {
                continue;
            }
            statements.push(self.statement());
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block.");
        Statement::Block(statements)
    }

    /// Parses a single statement.
    fn statement(&mut self) -> Statement {
        if self.match_token(&[TokenType::Return]) {
            return self.return_statement();
        }

        if self.match_token(&[TokenType::If]) {
            return self.if_statement();
        }

        // NEW: Check for Declaration (Identifier followed by Colon)
        if self.check(TokenType::Identifier) && self.peek_next().token_type == TokenType::Colon {
            return self.declare_statement();
        }

        // Future: If identifier followed by '=', it's an assignment.
        // Future: If identifier followed by '(', it's a function call.

        panic!("Unexpected token at start of statement: {:?}", self.peek());
    }

    /// Parses an if statement.
    fn if_statement(&mut self) -> Statement {
        // 1. Parse Condition
        // The 'if' token was already consumed by statement()
        let condition = self.expression();

        // 2. Parse the "Then" Block
        self.consume(TokenType::LeftBrace, "Expected '{' after if condition.");
        let then_branch = self.block(); // Use a helper for blocks

        // 3. Parse Optional "Else" Block
        let else_branch = if self.match_token(&[TokenType::Else]) {
            // Handle "else if" vs "else { block }"
            if self.match_token(&[TokenType::If]) {
                // Recursive call for "else if"
                Some(Box::new(self.if_statement()))
            } else {
                self.consume(TokenType::LeftBrace, "Expected '{' after else.");
                Some(Box::new(self.block()))
            }
        } else {
            None
        };

        Statement::If(condition, Box::new(then_branch), else_branch)
    }

    // declare syntax : name: type = expr;
    fn declare_statement(&mut self) -> Statement {
        let name = self
            .consume(TokenType::Identifier, "Expected variable name.")
            .value
            .clone();

        self.consume(TokenType::Colon, "Expected ':' after variable name.");

        let ty = self.get_type();

        self.consume(TokenType::Assign, "Expected '=' after variable type.");

        let expr = self.expression();

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        );

        Statement::Declare(name, ty, expr)
    }

    /// Parses a return statement.
    fn return_statement(&mut self) -> Statement {
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression())
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expected ';' after return value.");
        Statement::Return(value)
    }

    // 1. EXPRESSION (Entry Point)
    // Simply delegates to the lowest precedence operator (Equality)
    fn expression(&mut self) -> Expression {
        self.equality()
    }

    // 2. EQUALITY (==, !=)
    fn equality(&mut self) -> Expression {
        let mut left = self.comparison();

        while self.match_token(&[TokenType::Equals, TokenType::NotEqual]) {
            let operator = match self.previous().token_type {
                TokenType::Equals => BinaryOp::Equal,
                TokenType::NotEqual => BinaryOp::NotEqual,
                _ => unreachable!(),
            };
            let right = self.comparison();
            left = Expression::Binary(Box::new(left), operator, Box::new(right));
        }
        left
    }

    // 3. COMPARISON (<, >, <=, >=)
    fn comparison(&mut self) -> Expression {
        let mut left = self.term();

        while self.match_token(&[
            TokenType::LessThan,
            TokenType::LessThanOrEqual,
            TokenType::GreaterThan,
            TokenType::GreaterThanOrEqual,
        ]) {
            let operator = match self.previous().token_type {
                TokenType::LessThan => BinaryOp::LessThan,
                TokenType::LessThanOrEqual => BinaryOp::LessThanOrEqual,
                TokenType::GreaterThan => BinaryOp::GreaterThan,
                TokenType::GreaterThanOrEqual => BinaryOp::GreaterThanOrEqual,
                _ => unreachable!(),
            };
            let right = self.term();
            left = Expression::Binary(Box::new(left), operator, Box::new(right));
        }
        left
    }

    // 4. TERM (+, -)
    // (This was previously named 'expression')
    fn term(&mut self) -> Expression {
        let mut left = self.factor(); // Calls factor now

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = match self.previous().token_type {
                TokenType::Plus => BinaryOp::Plus,
                TokenType::Minus => BinaryOp::Minus,
                _ => unreachable!(),
            };
            let right = self.factor();
            left = Expression::Binary(Box::new(left), operator, Box::new(right));
        }
        left
    }

    // 5. FACTOR (*, /)
    // (This was previously named 'term')
    fn factor(&mut self) -> Expression {
        let mut left = self.primary();

        while self.match_token(&[TokenType::Multiply, TokenType::Divide]) {
            let operator = match self.previous().token_type {
                TokenType::Multiply => BinaryOp::Multiply,
                TokenType::Divide => BinaryOp::Divide,
                _ => unreachable!(),
            };
            let right = self.primary();
            left = Expression::Binary(Box::new(left), operator, Box::new(right));
        }
        left
    }

    // 6. PRIMARY (Literals, Parens)
    fn primary(&mut self) -> Expression {
        // ... (Your existing primary logic stays the same) ...
        if self.match_token(&[
            TokenType::IntegerLiteral,
            TokenType::FloatLiteral,
            TokenType::StringLiteral,
            TokenType::BoolLiteral,
        ]) {
            return Expression::Literal(self.previous().parsed_value.clone());
        }

        if self.match_token(&[TokenType::Identifier]) {
            return Expression::Identifier(self.previous().value.clone());
        }

        if self.match_token(&[TokenType::LeftParen]) {
            // This call to self.expression() now starts at the top (equality),
            // allowing (1 + 1 == 2) to work correctly inside parens.
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected ')' after expression.");
            return expr;
        }

        panic!("Expected expression, found {:?}", self.peek());
    }

    fn get_type(&mut self) -> Type {
        let token = self.advance();

        match token.token_type {
            TokenType::I8 => crate::ast::Type::I8,
            TokenType::I16 => crate::ast::Type::I16,
            TokenType::I32 => crate::ast::Type::I32,
            TokenType::I64 => crate::ast::Type::I64,
            TokenType::U8 => crate::ast::Type::U8,
            TokenType::U16 => crate::ast::Type::U16,
            TokenType::U32 => crate::ast::Type::U32,
            TokenType::U64 => crate::ast::Type::U64,
            TokenType::F32 => crate::ast::Type::F32,
            TokenType::F64 => crate::ast::Type::F64,
            TokenType::Bool => crate::ast::Type::Bool,
            _ => panic!("Expected type, found {:?}", token),
        }
    }

    /// Checks if the current token is the end of the source code.
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    /// Returns the current token.
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_next(&self) -> &Token {
        if self.current + 1 >= self.tokens.len() {
            return &self.tokens[self.tokens.len() - 1]; // Return EOF if out of bounds
        }
        &self.tokens[self.current + 1]
    }

    /// Returns the previous token.
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Advances the current token index.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Checks if the current token matches the given type.
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    /// Checks if the current token matches any of the given types.
    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Consumes the current token if it matches the given type.
    fn consume(&mut self, token_type: TokenType, message: &str) -> &Token {
        if self.check(token_type) {
            return self.advance();
        }
        panic!("Parse Error: {} found {:?}", message, self.peek());
    }
}
