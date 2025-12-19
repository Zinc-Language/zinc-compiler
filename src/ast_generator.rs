use crate::ast::{BinaryOp, Expression, Program, Statement};
use crate::parser::{Token, TokenType};

pub struct AstGenerator {
    tokens: Vec<Token>,
    current: usize,
}

impl AstGenerator {
    pub fn new(tokens: Vec<Token>) -> Self {
        // Filter out tokens that the parser doesn't need to care about
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

    pub fn parse_program(&mut self) -> Program {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            // Skip "invisible" tokens that the AST doesn't care about
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

    fn statement(&mut self) -> Statement {
        if self.match_token(&[TokenType::Return]) {
            return self.return_statement();
        }

        panic!("Unexpected token at start of statement: {:?}", self.peek());
    }

    fn return_statement(&mut self) -> Statement {
        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression())
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expected ';' after return value.");
        Statement::Return(value)
    }

    fn expression(&mut self) -> Expression {
        let mut left = self.term();

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            let operator = match self.previous().token_type {
                TokenType::Plus => BinaryOp::Plus,
                TokenType::Minus => BinaryOp::Minus,
                _ => unreachable!(),
            };

            let right = self.term(); // Parse the right side

            // Combine them into a new Binary expression
            left = Expression::Binary(Box::new(left), operator, Box::new(right));
        }

        left
    }

    // 2. Mid Level: Handles Multiplication and Division
    fn term(&mut self) -> Expression {
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

    // 3. Bottom Level: Literals (and eventually Parentheses)
    fn primary(&mut self) -> Expression {
        // 1. Check for Literals
        if self.match_token(&[
            TokenType::IntegerLiteral,
            TokenType::FloatLiteral,
            TokenType::StringLiteral,
            TokenType::BoolLiteral,
        ]) {
            return Expression::Literal(self.previous().parsed_value.clone());
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression();

            self.consume(TokenType::RightParen, "Expected ')' after expression.");
            return expr;
        }

        panic!("Expected expression, found {:?}", self.peek());
    }

    // --- Helper Methods ---

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> &Token {
        if self.check(token_type) {
            return self.advance();
        }
        panic!("Parse Error: {} found {:?}", message, self.peek());
    }
}
