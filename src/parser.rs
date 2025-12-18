#[derive(Debug, PartialEq)]
pub enum TokenValue {
    String(String),
    Float(f64),
    Integer(i64),
    Bool(bool),
    None,
}

/// Represents a token.
/// ### Fields
/// - `token_type`: The token type.
/// - `value`: The value of the token as a String literal.
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub parsed_value: TokenValue,
    pub line: usize,
    pub column: usize,
}

/// Represents a token type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Keywords
    Fn,
    If,
    Else,
    While,
    For,
    Break,
    Continue,
    Return,
    Null,
    Class,
    Public,
    Private,
    Protected,
    Static,
    Async,
    Enum,
    Macro,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    LeftShiftAssign,
    RightShiftAssign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    PowerAssign,
    DivideAssign,
    ModuloAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    Assign,
    Increment,
    Decrement,
    Arrow,

    // Types
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    USize,
    F32,
    F64,
    String,
    Char,
    Bool,
    Vector,
    HashMap,
    IntegerLiteral,
    FloatLiteral,
    CharLiteral,
    BoolLiteral,
    StringLiteral,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Dot,
    Comma,
    Colon,
    Semicolon,

    // Identifiers
    Identifier,

    // Whitespace
    Whitespace,
    Newline,

    // Comments
    Comment,
    BlockComment,
    DocComment,

    // Punctuation
    EOF,
}

const MULTI_CHAR_OPERATORS: &[(&str, TokenType)] = &[
    ("<<=", TokenType::LeftShiftAssign),
    (">>=", TokenType::RightShiftAssign),
    ("**=", TokenType::PowerAssign),
    ("==", TokenType::Equals),
    ("!=", TokenType::NotEqual),
    ("<=", TokenType::LessThanOrEqual),
    (">=", TokenType::GreaterThanOrEqual),
    ("+=", TokenType::AddAssign),
    ("-=", TokenType::SubtractAssign),
    ("*=", TokenType::MultiplyAssign),
    ("/=", TokenType::DivideAssign),
    ("%=", TokenType::ModuloAssign),
    ("&=", TokenType::BitwiseAndAssign),
    ("|=", TokenType::BitwiseOrAssign),
    ("^=", TokenType::BitwiseXorAssign),
    ("++", TokenType::Increment),
    ("--", TokenType::Decrement),
    ("->", TokenType::Arrow),
    ("&&", TokenType::And),
    ("||", TokenType::Or),
    ("<<", TokenType::LeftShift),
    (">>", TokenType::RightShift),
];

/// Represents a source file.
/// ### Fields
/// - `contents`: The contents of the source file.
/// - `current_token_index`: The current token index.
/// - `current_line`: The current line number.
/// - `current_column`: The current column number.
pub struct SourceFile {
    contents: String,
    current_token_index: usize,
    current_line: usize,
    current_column: usize,
}

/// ### Methods
/// - `new`: Creates a new source file from a given string.
/// - `peek`: Returns the character at the given offset from the current token index.
/// - `advance`: Advances the current token index by one, accounting for newlines.
/// - `advance_by`: Advances the current token index by a given amount, accounting for newlines.
impl SourceFile {
    pub fn new(source_file: String) -> SourceFile {
        SourceFile {
            contents: source_file,
            current_token_index: 0,
            current_line: 1,
            current_column: 1,
        }
    }

    /// Returns the character at the given offset from the current token index.
    /// ### Arguments
    /// - `offset`: The offset from the current token index.
    pub fn peek(&self, offset: usize) -> Option<char> {
        Some(
            self.contents
                .chars()
                .nth(self.current_token_index + offset)?,
        )
    }

    /// Advances the current token index by one, accounting for newlines.
    pub fn advance(&mut self) {
        if self.peek(0) == Some('\n') {
            self.current_line += 1;
            self.current_column = 1;
        } else {
            self.current_column += 1;
        }
        self.current_token_index += 1;
    }

    /// Advances the current token index by a given amount, accounting for newlines.
    /// ### Arguments
    /// - `amount`: The amount to advance the index by.
    pub fn advance_by(&mut self, amount: usize) {
        let mut amount = amount;

        while amount > 0 {
            self.current_token_index += 1;
            if self.peek(0) == Some('\n') {
                self.current_line += 1;
                self.current_column = 1;
            } else {
                self.current_column += 1;
            }
            amount -= 1;
        }
    }
}

/// Returns the token type for a given token. Pretty self-explanatory.
/// ### Arguments
/// - `token`: The token buffer to get the token type for.
fn get_token_type(token: &str) -> TokenType {
    match token {
        // Keywords
        "fn" => TokenType::Fn,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "while" => TokenType::While,
        "for" => TokenType::For,
        "break" => TokenType::Break,
        "continue" => TokenType::Continue,
        "return" => TokenType::Return,
        "i8" => TokenType::I8,
        "i16" => TokenType::I16,
        "i32" => TokenType::I32,
        "i64" => TokenType::I64,
        "u8" => TokenType::U8,
        "u16" => TokenType::U16,
        "u32" => TokenType::U32,
        "u64" => TokenType::U64,
        "usize" => TokenType::USize,
        "f32" => TokenType::F32,
        "f64" => TokenType::F64,
        "char" => TokenType::Char,
        "bool" => TokenType::Bool,
        "String" => TokenType::String,
        "Vector" => TokenType::Vector,
        "HashMap" => TokenType::HashMap,
        "null" => TokenType::Null,
        "true" | "false" => TokenType::BoolLiteral,
        "class" => TokenType::Class,
        "public" => TokenType::Public,
        "private" => TokenType::Private,
        "protected" => TokenType::Protected,
        "static" => TokenType::Static,
        "async" => TokenType::Async,
        "enum" => TokenType::Enum,

        // Punctuation (single char)
        "." => TokenType::Dot,
        "," => TokenType::Comma,
        ";" => TokenType::Semicolon,
        ":" => TokenType::Colon,
        "(" => TokenType::LeftParen,
        ")" => TokenType::RightParen,
        "{" => TokenType::LeftBrace,
        "}" => TokenType::RightBrace,
        "[" => TokenType::LeftBracket,
        "]" => TokenType::RightBracket,
        "+" => TokenType::Plus,
        "-" => TokenType::Minus,
        "*" => TokenType::Multiply,
        "/" => TokenType::Divide,
        "%" => TokenType::Modulo,
        "<" => TokenType::LessThan,
        ">" => TokenType::GreaterThan,
        "&" => TokenType::BitwiseAnd,
        "|" => TokenType::BitwiseOr,
        "^" => TokenType::BitwiseXor,
        "=" => TokenType::Assign,
        _ => {
            if token.starts_with("0x") {
                if i64::from_str_radix(&token[2..], 16).is_ok() {
                    return TokenType::IntegerLiteral;
                }
            } else if token.starts_with("0o") {
                if i64::from_str_radix(&token[2..], 8).is_ok() {
                    return TokenType::IntegerLiteral;
                }
            } else if token.starts_with("0b") {
                if i64::from_str_radix(&token[2..], 2).is_ok() {
                    return TokenType::IntegerLiteral;
                }
            }

            if token.parse::<i64>().is_ok() {
                return TokenType::IntegerLiteral;
            }

            if token.parse::<f64>().is_ok() {
                return TokenType::FloatLiteral;
            }

            TokenType::Identifier
        }
    }
}

/// Handles punctuation tokens and combines them into operators.
/// ### Arguments
/// - `source_file`: The source file to handle the punctuation from.
/// - `tokens`: The vector of tokens to add the punctuation to.
pub fn handle_punctuation(source_file: &mut SourceFile, tokens: &mut Vec<Token>) {
    // Try to match the longest possible operator first (Maximal Munch)
    for (op_str, op_type) in MULTI_CHAR_OPERATORS {
        let mut matched = true;
        for (i, op_char) in op_str.chars().enumerate() {
            if source_file.peek(i) != Some(op_char) {
                matched = false;
                break;
            }
        }

        if matched {
            tokens.push(Token {
                token_type: op_type.clone(),
                value: op_str.to_string(),
                parsed_value: TokenValue::String(op_str.to_string()),
                line: source_file.current_line,
                column: source_file.current_column,
            });
            source_file.advance_by(op_str.len());
            return;
        }
    }

    // If no multi-char matches, handle single char
    if let Some(c) = source_file.peek(0) {
        let token_type = get_token_type(&c.to_string());
        tokens.push(Token {
            token_type: token_type.clone(),
            value: c.to_string(),
            parsed_value: get_token_value(&c.to_string(), &token_type),
            line: source_file.current_line,
            column: source_file.current_column,
        });
        source_file.advance();
    }
}

/// Returns true if the given character is a punctuation character.
/// Pretty much just a list of characters that are not part of an identifier.
fn is_punctuation(c: char) -> bool {
    matches!(
        c,
        '+' | '-'
            | '*'
            | '/'
            | '%'
            | '!'
            | '<'
            | '>'
            | '&'
            | '|'
            | '^'
            | '='
            | '?'
            | '('
            | ')'
            | '{'
            | '}'
            | '['
            | ']'
            | ':'
            | ';'
            | ','
            | '.'
    )
}

fn get_token_value(token: &str, token_type: &TokenType) -> TokenValue {
    if matches!(token_type, TokenType::StringLiteral) {
        return TokenValue::String(token.trim_matches('"').to_string());
    }

    if token == "true" {
        return TokenValue::Bool(true);
    }

    if token == "false" {
        return TokenValue::Bool(false);
    }

    if token == "null" {
        return TokenValue::None;
    }

    if token.starts_with("0x") {
        // Hex
        return TokenValue::Integer(i64::from_str_radix(&token[2..], 16).unwrap());
    } else if token.starts_with("0o") {
        // Octal
        return TokenValue::Integer(i64::from_str_radix(&token[2..], 8).unwrap());
    } else if token.starts_with("0b") {
        // Binary
        return TokenValue::Integer(i64::from_str_radix(&token[2..], 2).unwrap());
    } else {
        // Float
        if token.parse::<i64>().is_ok() {
            return TokenValue::Integer(token.parse::<i64>().unwrap());
        } else if token.parse::<f64>().is_ok() {
            return TokenValue::Float(token.parse::<f64>().unwrap());
        }
    }

    // Default to string
    return TokenValue::String(token.to_string());
}

/// Flushes the token buffer to the token vector, and resets the buffer.
/// ### Arguments
/// - `tokens`: The vector of tokens to add the token to.
/// - `tok_buf`: The buffer to flush.
/// - `start_line`: The line number to start the token at.
/// - `start_column`: The column number to start the token at.
fn flush_buffer(
    tokens: &mut Vec<Token>,
    tok_buf: &mut String,
    start_line: usize,
    start_column: usize,
) {
    if !tok_buf.is_empty() {
        let token_type = get_token_type(&tok_buf);
        let parsed_value = get_token_value(&tok_buf, &token_type);

        tokens.push(Token {
            token_type: token_type.clone(),
            value: tok_buf.clone(),
            parsed_value: parsed_value,
            line: start_line,
            column: start_column,
        });
        tok_buf.clear();
    }
}

/// Handles whitespace by collapsing whitespace characters into a single token. Handles newlines.
/// ### Arguments
/// - `source_file`: The source file to handle the whitespace from.
/// - `tokens`: The vector of tokens to add the whitespace token to.
fn handle_whitespace(source_file: &mut SourceFile, tokens: &mut Vec<Token>) {
    let mut whitespace_buffer = String::new();
    while let Some(ws) = source_file.peek(0) {
        if ws.is_whitespace() {
            if ws == '\n' {
                if !whitespace_buffer.is_empty() {
                    tokens.push(Token {
                        token_type: TokenType::Whitespace,
                        value: whitespace_buffer.clone(),
                        parsed_value: TokenValue::String(whitespace_buffer.clone()),
                        line: source_file.current_line,
                        column: source_file.current_column,
                    });
                    whitespace_buffer.clear();
                }
                tokens.push(Token {
                    token_type: TokenType::Newline,
                    value: '\n'.to_string(),
                    parsed_value: TokenValue::String('\n'.to_string()),
                    line: source_file.current_line,
                    column: source_file.current_column,
                });
                source_file.advance();
                break;
            }

            whitespace_buffer.push(ws);
            source_file.advance();
        } else {
            break;
        }
    }
    // Handles EOF and non-whitespace cases
    if !whitespace_buffer.is_empty() {
        tokens.push(Token {
            token_type: TokenType::Whitespace,
            value: whitespace_buffer.clone(),
            parsed_value: TokenValue::String(whitespace_buffer.clone()),
            line: source_file.current_line,
            column: source_file.current_column,
        });
        whitespace_buffer.clear();
    }
}

/// Handles string literals by accumulating the literal into a single token.
/// ### Arguments
/// - `source_file`: The source file to handle the string literal from.
/// - `tokens`: The vector of tokens to add the string literal token to.
fn handle_string_literal(source_file: &mut SourceFile, tokens: &mut Vec<Token>) {
    let mut string_literal = String::new();
    let start_line = source_file.current_line;
    let start_column = source_file.current_column;

    source_file.advance();

    while let Some(inner_c) = source_file.peek(0) {
        // Escape sequences
        if inner_c == '\\' {
            if let Some(next_c) = source_file.peek(1) {
                match next_c {
                    '"' => string_literal.push('"'),
                    'n' => string_literal.push('\n'),
                    't' => string_literal.push('\t'),
                    '\\' => string_literal.push('\\'),
                    _ => {
                        string_literal.push('\\');
                        string_literal.push(next_c);
                    }
                }
                source_file.advance_by(2);
            }
        } else if inner_c == '"' {
            source_file.advance();
            break;
        } else {
            string_literal.push(inner_c);
            source_file.advance();
        }
    }

    tokens.push(Token {
        token_type: TokenType::StringLiteral,
        value: string_literal.clone(),
        parsed_value: TokenValue::String(string_literal),
        line: start_line,
        column: start_column,
    });
}

fn handle_comment(source_file: &mut SourceFile, tokens: &mut Vec<Token>) -> bool {
    let is_slash_slash = source_file.peek(1) == Some('/');
    let is_slash_star = source_file.peek(1) == Some('*');

    if !is_slash_slash && !is_slash_star {
        return false;
    }

    let start_line = source_file.current_line;
    let start_column = source_file.current_column;

    let mut comment_buf = String::new();
    let token_type;

    if is_slash_slash {
        source_file.advance_by(2);

        // Doc comment
        if source_file.peek(0) == Some('/') {
            token_type = TokenType::DocComment;
            source_file.advance();
        } else {
            token_type = TokenType::Comment;
        }

        // Consume until newline
        while let Some(inner_c) = source_file.peek(0) {
            if inner_c == '\n' {
                break;
            }
            comment_buf.push(inner_c);
            source_file.advance();
        }
    } else {
        // Multiline comment
        token_type = TokenType::BlockComment;
        source_file.advance_by(2);

        while let Some(inner_c) = source_file.peek(0) {
            if inner_c == '*' && source_file.peek(1) == Some('/') {
                source_file.advance_by(2);
                break;
            }
            comment_buf.push(inner_c);
            source_file.advance();
        }
    }

    tokens.push(Token {
        token_type,
        value: comment_buf.clone(),
        parsed_value: TokenValue::String(comment_buf),
        line: start_line,
        column: start_column,
    });

    true
}

fn handle_macro(source_file: &mut SourceFile, tokens: &mut Vec<Token>) {
    let mut macro_buf = String::new();
    let start_line = source_file.current_line;
    let start_column = source_file.current_column;

    while let Some(inner_c) = source_file.peek(0) {
        if !is_punctuation(inner_c) || inner_c.is_whitespace() {
            macro_buf.push(inner_c);
            source_file.advance();
        } else {
            break;
        }
    }

    tokens.push(Token {
        token_type: TokenType::Macro,
        value: macro_buf.clone(),
        parsed_value: TokenValue::String(macro_buf),
        line: start_line,
        column: start_column,
    });
}

// Lexical analysis
pub fn parse(source: String) -> Vec<Token> {
    let mut source_file = SourceFile::new(source);
    let mut tokens: Vec<Token> = Vec::new();
    let mut tok_buf: String = String::new();

    // Keep track of the starting line and column for literals and identifiers
    let mut start_line = 1;
    let mut start_column = 1;

    while let Some(c) = source_file.peek(0) {
        if c.is_whitespace() {
            // flush whatever is in the identifier buffer
            flush_buffer(&mut tokens, &mut tok_buf, start_line, start_column);
            handle_whitespace(&mut source_file, &mut tokens);
        } else if is_punctuation(c) {
            flush_buffer(&mut tokens, &mut tok_buf, start_line, start_column);

            if c == '/' {
                // Handle single and multi line comments
                if handle_comment(&mut source_file, &mut tokens) {
                    continue;
                } else {
                    handle_punctuation(&mut source_file, &mut tokens);
                }
            } else {
                handle_punctuation(&mut source_file, &mut tokens);
            }
        } else if c == '"' {
            handle_string_literal(&mut source_file, &mut tokens);
        } else if c == '@' {
            handle_macro(&mut source_file, &mut tokens);
        } else {
            if tok_buf.is_empty() {
                start_line = source_file.current_line;
                start_column = source_file.current_column;
            }

            // Handle Identifiers/Numbers
            tok_buf.push(c);
            source_file.advance();
        }
    }

    // Flush the last token
    flush_buffer(&mut tokens, &mut tok_buf, start_line, start_column);

    tokens.push(Token {
        token_type: TokenType::EOF,
        value: String::new(), // Empty string
        parsed_value: TokenValue::None,
        line: source_file.current_line,
        column: source_file.current_column,
    });

    tokens
}

// Courtesy of Google Gemini
#[cfg(test)]
mod tests {
    use super::*;
    fn get_values(input: &str) -> Vec<String> {
        parse(input.to_string())
            .into_iter()
            // FIX: Add Comment, BlockComment, DocComment to filter
            .filter(|t| {
                !matches!(
                    t.token_type,
                    TokenType::Whitespace
                        | TokenType::EOF
                        | TokenType::Comment
                        | TokenType::BlockComment
                        | TokenType::DocComment
                )
            })
            .map(|t| t.value)
            .collect()
    }

    // ========================================================================
    // 1. SourceFile Tests (The Foundation)
    // ========================================================================

    #[test]
    fn test_source_file_advancement() {
        let mut src = SourceFile::new("abc\ndef".to_string());

        // Check initial state
        assert_eq!(src.peek(0), Some('a'));
        assert_eq!(src.current_line, 1);
        assert_eq!(src.current_column, 1);

        // Advance past 'a', 'b', 'c'
        src.advance();
        src.advance();
        src.advance();

        // Now at newline
        assert_eq!(src.peek(0), Some('\n'));
        assert_eq!(src.current_line, 1);
        assert_eq!(src.current_column, 4);

        // Advance past newline (should trigger line update)
        src.advance();
        assert_eq!(src.peek(0), Some('d'));
        assert_eq!(src.current_line, 2);
        assert_eq!(src.current_column, 1); // Reset column
    }

    #[test]
    fn test_source_file_advance_by() {
        let mut src = SourceFile::new("12345".to_string());
        src.advance_by(3);
        assert_eq!(src.peek(0), Some('4'));
        assert_eq!(src.current_token_index, 3);
    }

    // ========================================================================
    // 2. Whitespace Handler Tests
    // ========================================================================

    #[test]
    fn test_handle_whitespace_basic() {
        let mut src = SourceFile::new("   abc".to_string());
        let mut tokens = Vec::new();

        handle_whitespace(&mut src, &mut tokens);

        // Should have 1 Whitespace token
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Whitespace);
        assert_eq!(tokens[0].value, "   ");

        // Source should be pointing at 'a'
        assert_eq!(src.peek(0), Some('a'));
    }

    #[test]
    fn test_handle_whitespace_with_newline() {
        let mut src = SourceFile::new("  \n".to_string());
        let mut tokens = Vec::new();

        handle_whitespace(&mut src, &mut tokens);

        // Should have [Whitespace("  "), Newline("\n")]
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Whitespace);
        assert_eq!(tokens[1].token_type, TokenType::Newline);
    }

    #[test]
    fn test_handle_whitespace_eof() {
        // Critical edge case: File ends with spaces
        let mut src = SourceFile::new("   ".to_string());
        let mut tokens = Vec::new();

        handle_whitespace(&mut src, &mut tokens);

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].value, "   ");
    }

    // ========================================================================
    // 3. Punctuation Handler Tests
    // ========================================================================

    #[test]
    fn test_handle_punctuation_maximal_munch() {
        // Should prefer '<<=' over '<' or '<<' or '<='
        let mut src = SourceFile::new("<<=".to_string());
        let mut tokens = Vec::new();

        handle_punctuation(&mut src, &mut tokens);

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].token_type, TokenType::LeftShiftAssign));
        assert_eq!(tokens[0].value, "<<=");
    }

    #[test]
    fn test_handle_punctuation_fallback() {
        // '&' is a prefix for '&&' and '&=', but here it stands alone
        let mut src = SourceFile::new("&a".to_string());
        let mut tokens = Vec::new();

        handle_punctuation(&mut src, &mut tokens);

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].token_type, TokenType::BitwiseAnd));
        assert_eq!(tokens[0].value, "&");

        // Ensure it didn't consume the 'a'
        assert_eq!(src.peek(0), Some('a'));
    }

    // ========================================================================
    // 4. Comment Handler Tests
    // ========================================================================

    #[test]
    fn test_handle_single_line_comment() {
        let mut src = SourceFile::new("// comment\nnext".to_string());
        let mut tokens = Vec::new();

        // We need to advance past the initial '/' manually because
        // the main loop usually handles that check before calling handle_comment
        let was_comment = handle_comment(&mut src, &mut tokens);

        assert!(was_comment);
        // Should stop at '\n' without consuming it
        assert_eq!(src.peek(0), Some('\n'));
    }

    #[test]
    fn test_handle_block_comment() {
        let mut src = SourceFile::new("/* block \n comment */after".to_string());
        let mut tokens = Vec::new();

        let was_comment = handle_comment(&mut src, &mut tokens);

        assert!(was_comment);
        // Should be pointing at 'a' in "after"
        assert_eq!(src.peek(0), Some('a'));
    }

    #[test]
    fn test_handle_slash_is_not_comment() {
        // Just a divide operator
        let mut src = SourceFile::new("/ 5".to_string());
        let mut tokens = Vec::new();

        let was_comment = handle_comment(&mut src, &mut tokens);

        assert!(!was_comment); // It returns false
        // It should NOT have consumed the '/' or anything else
        assert_eq!(src.peek(0), Some('/'));
    }

    #[test]
    fn test_nested_asterisk_in_block_comment() {
        // Edge case: /* * */ should handle the middle * correctly
        let mut src = SourceFile::new("/* * */".to_string());
        let mut tokens = Vec::new();
        let was_comment = handle_comment(&mut src, &mut tokens);
        assert!(was_comment);
        assert_eq!(src.peek(0), None); // Should be at EOF
    }

    // ========================================================================
    // 5. String Literal Handler Tests
    // ========================================================================

    #[test]
    fn test_handle_string_simple() {
        let mut src = SourceFile::new("\"hello\"".to_string());
        let mut tokens = Vec::new();

        handle_string_literal(&mut src, &mut tokens);

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].token_type, TokenType::StringLiteral));
        assert_eq!(tokens[0].value, "hello");
    }

    #[test]
    fn test_handle_string_escapes() {
        // Input represents: " \" \\ "
        let mut src = SourceFile::new(r#"" \" \\ ""#.to_string());
        let mut tokens = Vec::new();

        handle_string_literal(&mut src, &mut tokens);

        assert_eq!(tokens.len(), 1);
        // Value should be: " \
        assert_eq!(tokens[0].value, r#" " \ "#);
    }

    // ========================================================================
    // 6. Full Integration Tests (The Main Parse Loop)
    // ========================================================================

    // Helper for integration tests
    fn get_types(input: &str) -> Vec<String> {
        parse(input.to_string())
            .iter()
            .map(|t| format!("{:?}", t.token_type)) // Debug print the enum variant
            .collect()
    }

    #[test]
    fn test_parse_function_variable_decl() {
        let input = "let x = 10;";
        // Note: 'let' is not in your keywords list yet, so it will be an Identifier
        let types = get_types(input);

        let expected = vec![
            "Identifier", // let
            "Whitespace",
            "Identifier", // x
            "Whitespace",
            "Assign", // =
            "Whitespace",
            "Identifier", // 10 (since you don't have a Number type logic yet, 10 is an ident)
            "Semicolon",  // ;
            "EOF",
        ];

        assert_eq!(types, expected);
    }

    #[test]
    fn test_parse_function_mixed_no_spaces() {
        // verifying delimiters separate tokens without needing spaces
        let input = "fn(x)";
        let types = get_types(input);

        let expected = vec!["Fn", "LeftParen", "Identifier", "RightParen", "EOF"];

        assert_eq!(types, expected);
    }

    // Weird Edge Cases

    // ========================================================================
    // 1. String Literal Edge Cases
    // ========================================================================

    #[test]
    fn test_string_empty() {
        let input = r#""""#; // Empty string ""
        let tokens = parse(input.to_string());
        let str_token = tokens
            .iter()
            .find(|t| matches!(t.token_type, TokenType::StringLiteral))
            .unwrap();
        assert_eq!(str_token.value, "");
    }

    #[test]
    fn test_string_escaped_backslash_at_end() {
        // This is tricky: "abc\\" should be the string [abc\]
        // If the lexer sees the last \" as an escaped quote, it will fail to close the string.
        let input = r#" "abc\\" "#;
        let tokens = parse(input.to_string());
        let str_token = tokens
            .iter()
            .find(|t| matches!(t.token_type, TokenType::StringLiteral))
            .unwrap();
        assert_eq!(str_token.value, r#"abc\"#); // rust raw string syntax
    }

    #[test]
    fn test_string_unterminated() {
        // File ends while inside a string
        let input = r#" "unterminated "#;
        let tokens = parse(input.to_string());

        let str_token = tokens
            .iter()
            .find(|t| matches!(t.token_type, TokenType::StringLiteral));
        // Depending on your implementation, this usually returns what it collected so far
        assert!(str_token.is_some());
        assert_eq!(str_token.unwrap().value, "unterminated ");
    }

    // ========================================================================
    // 2. Comment Edge Cases
    // ========================================================================

    #[test]
    fn test_comment_fake_outs() {
        // Things that look like comments but aren't
        let input = "/ * 5"; // Space between / and *
        let values = get_values(input);
        // Should be Division, Asterisk, Identifier(5)
        assert_eq!(values, vec!["/", "*", "5"]);
    }

    #[test]
    fn test_comment_unterminated_block() {
        // File ends inside a block comment
        let input = "/* starting comment...";
        let values = get_values(input);
        // Should return empty significant tokens (comment ate everything)
        assert!(values.is_empty());
    }

    #[test]
    fn test_comment_nested_lookalike() {
        // /* inside */ should close at the first */
        let input = "/* comment /* nested? */ code";
        let values = get_values(input);
        // The first */ closes the comment. "code" should be tokenized.
        assert_eq!(values, vec!["code"]);
    }

    #[test]
    fn test_comment_ending_at_eof_no_newline() {
        let input = "// comment at eof";
        let tokens = parse(input.to_string());

        // Check that the last token is EOF
        assert_eq!(tokens.last().unwrap().token_type, TokenType::EOF);

        // FIX: Expect 2 tokens (Comment + EOF), not 1
        assert_eq!(tokens.len(), 2);

        // Optional: Verify the first token is actually the comment
        assert_eq!(tokens[0].token_type, TokenType::Comment);
        assert_eq!(tokens[0].value, " comment at eof");
    }

    // ========================================================================
    // 3. Punctuation & Maximal Munch Stress Tests
    // ========================================================================

    #[test]
    fn test_punctuation_soup() {
        // A sequence of characters that touches nearly every boundary condition
        let input = "+++++";
        let values = get_values(input);
        // ++, ++, +
        assert_eq!(values, vec!["++", "++", "+"]);
    }

    #[test]
    fn test_punctuation_tricky_equals() {
        // !=== should be !=, ==
        let input = "!===";
        let values = get_values(input);
        assert_eq!(values, vec!["!=", "=="]);
    }

    #[test]
    fn test_punctuation_touching_identifiers() {
        // Ensure no spaces are needed
        let input = "return(x+1);";
        let values = get_values(input);
        assert_eq!(values, vec!["return", "(", "x", "+", "1", ")", ";"]);
    }

    // ========================================================================
    // 4. Identifier & Keyword Edge Cases
    // ========================================================================

    #[test]
    fn test_identifier_containing_keywords() {
        // 'if' is a keyword, 'iff' is an identifier
        let input = "if iff return_val";
        let values = get_values(input);

        let tokens = parse(input.to_string());
        let types: Vec<&TokenType> = tokens
            .iter()
            .filter(|t| !matches!(t.token_type, TokenType::Whitespace | TokenType::EOF))
            .map(|t| &t.token_type)
            .collect();

        // Check values
        assert_eq!(values, vec!["if", "iff", "return_val"]);

        // Check types
        assert_eq!(types[0], &TokenType::If);
        assert_eq!(types[1], &TokenType::Identifier);
        assert_eq!(types[2], &TokenType::Identifier);
    }

    #[test]
    fn test_identifier_mixed_case() {
        // Assuming your language is case-sensitive
        let input = "Return return RETURN";
        let values = get_values(input);

        let tokens = parse(input.to_string());
        let types: Vec<&TokenType> = tokens
            .iter()
            .filter(|t| !matches!(t.token_type, TokenType::Whitespace | TokenType::EOF))
            .map(|t| &t.token_type)
            .collect();

        assert_eq!(values, vec!["Return", "return", "RETURN"]);
        assert_eq!(types[0], &TokenType::Identifier); // Capital R
        assert_eq!(types[1], &TokenType::Return); // Keyword
        assert_eq!(types[2], &TokenType::Identifier); // All caps
    }

    // ========================================================================
    // 5. Whitespace Edge Cases
    // ========================================================================

    #[test]
    fn test_file_only_whitespace() {
        let input = "    \n   \t   ";
        let tokens = parse(input.to_string());

        // Should contain Whitespace, Newline, Whitespace, EOF
        assert!(tokens.len() > 1);
        assert_eq!(tokens.last().unwrap().token_type, TokenType::EOF);
    }

    #[test]
    fn test_empty_file() {
        let input = "";
        let tokens = parse(input.to_string());
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::EOF);
    }

    /// Helper to filter out Whitespace/EOF for cleaner assertions,
    /// checking only the meaningful tokens and their locations.
    fn parse_meaningful(input: &str) -> Vec<Token> {
        parse(input.to_string())
            .into_iter()
            // FIX: Add Comment, BlockComment, DocComment to filter
            .filter(|t| {
                !matches!(
                    t.token_type,
                    TokenType::Whitespace
                        | TokenType::Newline
                        | TokenType::EOF
                        | TokenType::Comment
                        | TokenType::BlockComment
                        | TokenType::DocComment
                )
            })
            .collect()
    }

    /// Helper assertion to check a token's location and value
    fn check_tok(token: &Token, expected_val: &str, line: usize, col: usize) {
        assert_eq!(token.value, expected_val, "Token Value Mismatch");
        assert_eq!(token.line, line, "Line Mismatch for '{}'", expected_val);
        assert_eq!(token.column, col, "Column Mismatch for '{}'", expected_val);
    }

    // ==========================================
    // 1. Basic Placement
    // ==========================================

    #[test]
    fn test_loc_start_of_file() {
        let input = "start";
        let tokens = parse_meaningful(input);

        // "start" should be at 1:1
        check_tok(&tokens[0], "start", 1, 1);
    }

    #[test]
    fn test_loc_horizontal_spacing() {
        // "one" is 3 chars. Spaces at 4, 5, 6. "two" starts at 7.
        let input = "one   two";
        let tokens = parse_meaningful(input);

        check_tok(&tokens[0], "one", 1, 1);
        check_tok(&tokens[1], "two", 1, 7);
    }

    // ==========================================
    // 2. Vertical Placement (Newlines)
    // ==========================================

    #[test]
    fn test_loc_simple_newline_reset() {
        // Line 1: 'a' (1:1)
        // Line 2: 'b' (2:1)
        let input = "a\nb";
        let tokens = parse_meaningful(input);

        check_tok(&tokens[0], "a", 1, 1);
        // Ensure Newline tokens are processed correctly internally
        // so that 'b' resets to column 1 on the next line.
        check_tok(&tokens[1], "b", 2, 1);
    }

    #[test]
    fn test_loc_indentation() {
        // Line 1: 'fn' (1:1)
        // Line 2: indent(2 spaces) -> 'x' starts at 2:3
        let input = "fn\n  x";
        let tokens = parse_meaningful(input);

        check_tok(&tokens[0], "fn", 1, 1);
        check_tok(&tokens[1], "x", 2, 3);
    }

    #[test]
    fn test_loc_multiple_empty_lines() {
        // Line 1: x
        // Line 2: (empty)
        // Line 3: (empty)
        // Line 4: y
        let input = "x\n\n\ny";
        let tokens = parse_meaningful(input);

        check_tok(&tokens[0], "x", 1, 1);
        check_tok(&tokens[1], "y", 4, 1);
    }

    // ==========================================
    // 3. Compact Operators (No Whitespace)
    // ==========================================

    #[test]
    fn test_loc_compact_arithmetic() {
        // 1+2
        // '1' -> 1:1
        // '+' -> 1:2
        // '2' -> 1:3
        let input = "1+2";
        let tokens = parse_meaningful(input);

        check_tok(&tokens[0], "1", 1, 1);
        check_tok(&tokens[1], "+", 1, 2);
        check_tok(&tokens[2], "2", 1, 3);
    }

    #[test]
    fn test_loc_punctuation_maximal_munch() {
        // Check that multi-char operators report the start position correctly
        // "=="
        // "==" starts at 1:1.
        // "x" should be at 1:3.
        let input = "==x";
        let tokens = parse_meaningful(input);

        check_tok(&tokens[0], "==", 1, 1);
        check_tok(&tokens[1], "x", 1, 3);
    }

    // ==========================================
    // 4. Complex Integration
    // ==========================================

    #[test]
    fn test_loc_complex_code_block() {
        // 1: if (true) {
        // 2:     return;
        // 3: }
        let input = "if (true) {\n    return;\n}";
        let tokens = parse_meaningful(input);

        // Line 1
        check_tok(&tokens[0], "if", 1, 1);
        check_tok(&tokens[1], "(", 1, 4);
        check_tok(&tokens[2], "true", 1, 5);
        check_tok(&tokens[3], ")", 1, 9);
        check_tok(&tokens[4], "{", 1, 11);

        // Line 2
        // 4 spaces indent, so return starts at 5
        check_tok(&tokens[5], "return", 2, 5);
        // return is 6 chars long (5..10), ; is at 11
        check_tok(&tokens[6], ";", 2, 11);

        // Line 3
        check_tok(&tokens[7], "}", 3, 1);
    }

    // ==========================================
    // 5. String Literals (Potential Bug Check)
    // ==========================================

    #[test]
    fn test_loc_string_literal_start_position() {
        // "hello"
        // The token should ideally report the position of the *opening quote*.
        // " starts at 1:1.
        let input = "\"hello\"";
        let tokens = parse_meaningful(input);

        // NOTE: If your handle_string_literal implementation sets
        // current_line/current_column AFTER the loop, this test will fail
        // (it will report the end of string).
        // Standard lexer behavior is to report the START.
        check_tok(&tokens[0], "hello", 1, 1);
    }

    #[test]
    fn test_loc_tokens_after_string() {
        // "a" b
        // "a" is length 3 (1,2,3). Space at 4. 'b' at 5.
        let input = "\"a\" b";
        let tokens = parse_meaningful(input);

        check_tok(&tokens[0], "a", 1, 1);
        check_tok(&tokens[1], "b", 1, 5);
    }
}
