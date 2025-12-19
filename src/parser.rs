/// Represents the value of a token.
/// ### Fields
/// - `String`: The string value of the token.
/// - `Float`: The float value of the token.
/// - `Integer`: The integer value of the token.
/// - `Bool`: The boolean value of the token.
/// - `None`: The token is None. (Useful for EOF)
#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, Clone)]
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
    Function,
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
    Const,
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

/// A list of multi-character operators and their corresponding token types.
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
        "fun" => TokenType::Function,
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
        "const" => TokenType::Const,

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
/// ### Arguments:
/// - `c`: The character to check.
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

/// Returns the value of a token based on its type.
/// ### Arguments
/// - `token`: The token to get the value of.
/// - `token_type`: The type of the token.
/// ### Returns
/// TokenValue (enum) representing the value of the token. Possible values are:
/// - String: The string value of the token.
/// - Float: The float value of the token.
/// - Integer: The integer value of the token.
/// - Bool: The boolean value of the token.
/// - None: The token is None. (Useful for EOF)
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

/// Handles comments by accumulating the comment into a single token.
/// ### Arguments
/// - `source_file`: The source file to handle the comment from.
/// - `tokens`: The vector of tokens to add the comment token to.
/// ### Returns
/// True if the comment was handled, false otherwise.
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

/// Handles macros by accumulating the macro into a single token.
/// ### Arguments
/// - `source_file`: The source file to handle the macro from.
/// - `tokens`: The vector of tokens to add the macro token to.
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

    let parsed_value = macro_buf.trim_start_matches('@');
    tokens.push(Token {
        token_type: TokenType::Macro,
        value: macro_buf.clone(),
        parsed_value: TokenValue::String(parsed_value.to_string()),
        line: start_line,
        column: start_column,
    });
}

/// Parses the given source code into a vector of tokens.
/// ### Arguments
/// - `source`: The source code to parse.
/// ### Returns
/// A vector of tokens (type Token) representing the source code.
pub fn parse(source: String) -> Vec<Token> {
    let mut source_file = SourceFile::new(source);
    let mut tokens: Vec<Token> = Vec::new();
    let mut tok_buf: String = String::new();

    // Keep track of the starting line and column for literals and identifiers
    let mut start_line = 1;
    let mut start_column = 1;

    while let Some(c) = source_file.peek(0) {
        // Check for floats with a dot
        let is_float_dot = c == '.'
            && source_file.peek(1).map_or(false, |n| n.is_ascii_digit())
            && (tok_buf.is_empty() || tok_buf.chars().all(|ch| ch.is_ascii_digit()));

        // Handle floats
        if is_float_dot {
            if tok_buf.is_empty() {
                start_line = source_file.current_line;
                start_column = source_file.current_column;
            }
            tok_buf.push(c);
            source_file.advance();
        } else if c.is_whitespace() {
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to assert a single token's properties
    fn assert_token(token: &Token, expected_type: TokenType, expected_value: &str) {
        assert_eq!(
            token.token_type, expected_type,
            "Type mismatch for value {}",
            expected_value
        );
        assert_eq!(token.value, expected_value, "Value mismatch");
    }

    /// Helper to verify line and column numbers
    fn assert_location(token: &Token, line: usize, col: usize) {
        assert_eq!(token.line, line, "Line mismatch for token {}", token.value);
        assert_eq!(
            token.column, col,
            "Column mismatch for token {}",
            token.value
        );
    }

    #[test]
    fn test_01_empty_source() {
        let tokens = parse("".to_string());
        assert_eq!(tokens.len(), 1);
        assert_token(&tokens[0], TokenType::EOF, "");
    }

    #[test]
    fn test_02_single_identifier() {
        let tokens = parse("variable".to_string());
        assert_token(&tokens[0], TokenType::Identifier, "variable");
    }

    #[test]
    fn test_03_keyword_fn() {
        let tokens = parse("fun".to_string());
        assert_token(&tokens[0], TokenType::Function, "fun");
    }

    #[test]
    fn test_04_keyword_return() {
        let tokens = parse("return".to_string());
        assert_token(&tokens[0], TokenType::Return, "return");
    }

    #[test]
    fn test_05_keyword_class() {
        let tokens = parse("class".to_string());
        assert_token(&tokens[0], TokenType::Class, "class");
    }

    #[test]
    fn test_06_integer_literal() {
        let tokens = parse("12345".to_string());
        assert_token(&tokens[0], TokenType::IntegerLiteral, "12345");
        assert_eq!(tokens[0].parsed_value, TokenValue::Integer(12345));
    }

    #[test]
    fn test_07_float_literal() {
        let tokens = parse("123.456".to_string());
        assert_token(&tokens[0], TokenType::FloatLiteral, "123.456");
        assert_eq!(tokens[0].parsed_value, TokenValue::Float(123.456));
    }

    #[test]
    fn test_08_hex_literal() {
        let tokens = parse("0xFF".to_string());
        assert_token(&tokens[0], TokenType::IntegerLiteral, "0xFF");
        assert_eq!(tokens[0].parsed_value, TokenValue::Integer(255));
    }

    #[test]
    fn test_09_binary_literal() {
        let tokens = parse("0b1010".to_string());
        assert_token(&tokens[0], TokenType::IntegerLiteral, "0b1010");
        assert_eq!(tokens[0].parsed_value, TokenValue::Integer(10));
    }

    #[test]
    fn test_10_octal_literal() {
        let tokens = parse("0o77".to_string());
        assert_token(&tokens[0], TokenType::IntegerLiteral, "0o77");
        assert_eq!(tokens[0].parsed_value, TokenValue::Integer(63));
    }

    #[test]
    fn test_11_string_simple() {
        let tokens = parse("\"hello\"".to_string());
        assert_token(&tokens[0], TokenType::StringLiteral, "hello");
        assert_eq!(
            tokens[0].parsed_value,
            TokenValue::String("hello".to_string())
        );
    }

    #[test]
    fn test_12_string_empty() {
        let tokens = parse("\"\"".to_string());
        assert_token(&tokens[0], TokenType::StringLiteral, "");
    }

    #[test]
    fn test_13_string_with_spaces() {
        let tokens = parse("\"hello world\"".to_string());
        assert_token(&tokens[0], TokenType::StringLiteral, "hello world");
    }

    #[test]
    fn test_14_string_escaped_quote() {
        let tokens = parse("\"say \\\"hello\\\"\"".to_string());
        assert_token(&tokens[0], TokenType::StringLiteral, "say \"hello\"");
    }

    #[test]
    fn test_15_string_escaped_newline() {
        let tokens = parse("\"Line\\nBreak\"".to_string());
        assert_token(&tokens[0], TokenType::StringLiteral, "Line\nBreak");
    }

    #[test]
    fn test_16_string_escaped_backslash() {
        let tokens = parse("\"Path\\\\To\"".to_string());
        assert_token(&tokens[0], TokenType::StringLiteral, "Path\\To");
    }

    #[test]
    fn test_17_bool_true() {
        let tokens = parse("true".to_string());
        assert_token(&tokens[0], TokenType::BoolLiteral, "true");
        assert_eq!(tokens[0].parsed_value, TokenValue::Bool(true));
    }

    #[test]
    fn test_18_bool_false() {
        let tokens = parse("false".to_string());
        assert_token(&tokens[0], TokenType::BoolLiteral, "false");
        assert_eq!(tokens[0].parsed_value, TokenValue::Bool(false));
    }

    #[test]
    fn test_19_null_literal() {
        let tokens = parse("null".to_string());
        assert_token(&tokens[0], TokenType::Null, "null");
        assert_eq!(tokens[0].parsed_value, TokenValue::None);
    }

    #[test]
    fn test_20_single_line_comment() {
        let tokens = parse("// This is a comment".to_string());
        assert_token(&tokens[0], TokenType::Comment, " This is a comment");
    }

    #[test]
    fn test_21_doc_comment() {
        let tokens = parse("/// This is a doc comment".to_string());
        assert_token(&tokens[0], TokenType::DocComment, " This is a doc comment");
    }

    #[test]
    fn test_22_block_comment_single_line() {
        let tokens = parse("/* block */".to_string());
        assert_token(&tokens[0], TokenType::BlockComment, " block ");
    }

    #[test]
    fn test_23_block_comment_multi_line() {
        let tokens = parse("/* line1\nline2 */".to_string());
        assert_token(&tokens[0], TokenType::BlockComment, " line1\nline2 ");
    }

    #[test]
    fn test_24_operator_plus() {
        let tokens = parse("+".to_string());
        assert_token(&tokens[0], TokenType::Plus, "+");
    }

    #[test]
    fn test_25_operator_increment() {
        let tokens = parse("++".to_string());
        assert_token(&tokens[0], TokenType::Increment, "++");
    }

    #[test]
    fn test_26_operator_add_assign() {
        let tokens = parse("+=".to_string());
        assert_token(&tokens[0], TokenType::AddAssign, "+=");
    }

    #[test]
    fn test_27_operator_arrow() {
        let tokens = parse("->".to_string());
        assert_token(&tokens[0], TokenType::Arrow, "->");
    }

    #[test]
    fn test_28_operator_equality() {
        let tokens = parse("==".to_string());
        assert_token(&tokens[0], TokenType::Equals, "==");
    }

    #[test]
    fn test_29_operator_not_equal() {
        let tokens = parse("!=".to_string());
        assert_token(&tokens[0], TokenType::NotEqual, "!=");
    }

    #[test]
    fn test_30_operator_complex_shift_assign() {
        let tokens = parse("<<=".to_string());
        assert_token(&tokens[0], TokenType::LeftShiftAssign, "<<=");
    }

    #[test]
    fn test_31_macro_basic() {
        let tokens = parse("@my_macro".to_string());
        assert_token(&tokens[0], TokenType::Macro, "@my_macro");
        assert_eq!(
            tokens[0].parsed_value,
            TokenValue::String("my_macro".to_string())
        );
    }

    #[test]
    fn test_32_whitespace_handling() {
        let tokens = parse("  ".to_string());
        assert_token(&tokens[0], TokenType::Whitespace, "  ");
    }

    #[test]
    fn test_33_newline_handling() {
        let tokens = parse("\n".to_string());
        assert_token(&tokens[0], TokenType::Newline, "\n");
    }

    #[test]
    fn test_34_mixed_whitespace_newline() {
        let tokens = parse(" \n ".to_string());
        // Expect: Whitespace(" "), Newline, Whitespace(" "), EOF
        assert_token(&tokens[0], TokenType::Whitespace, " ");
        assert_token(&tokens[1], TokenType::Newline, "\n");
        assert_token(&tokens[2], TokenType::Whitespace, " ");
    }

    #[test]
    fn test_35_identifier_position_start() {
        let tokens = parse("abc".to_string());
        assert_location(&tokens[0], 1, 1);
    }

    #[test]
    fn test_36_identifier_position_offset() {
        let tokens = parse("  abc".to_string());
        // Whitespace (1,1), abc (1,3)
        assert_token(&tokens[1], TokenType::Identifier, "abc");
        assert_location(&tokens[1], 1, 3);
    }

    #[test]
    fn test_37_identifier_position_multiline() {
        let tokens = parse("abc\ndef".to_string());
        // abc (1,1), newline (1,4), def (2,1)
        assert_token(&tokens[0], TokenType::Identifier, "abc");
        assert_token(&tokens[2], TokenType::Identifier, "def");
        assert_location(&tokens[2], 2, 1);
    }

    #[test]
    fn test_38_negative_number_parsing() {
        // The parser logic separates '-' as punctuation and the number as a literal
        let tokens = parse("-5".to_string());
        assert_token(&tokens[0], TokenType::Minus, "-");
        assert_token(&tokens[1], TokenType::IntegerLiteral, "5");
    }

    #[test]
    fn test_39_delimiter_parentheses() {
        let tokens = parse("()".to_string());
        assert_token(&tokens[0], TokenType::LeftParen, "(");
        assert_token(&tokens[1], TokenType::RightParen, ")");
    }

    #[test]
    fn test_40_delimiter_braces() {
        let tokens = parse("{}".to_string());
        assert_token(&tokens[0], TokenType::LeftBrace, "{");
        assert_token(&tokens[1], TokenType::RightBrace, "}");
    }

    #[test]
    fn test_41_semicolon_handling() {
        let tokens = parse("x;".to_string());
        assert_token(&tokens[0], TokenType::Identifier, "x");
        assert_token(&tokens[1], TokenType::Semicolon, ";");
    }

    #[test]
    fn test_42_type_keyword_parsing() {
        let tokens = parse("String Vector i32".to_string());
        assert_token(&tokens[0], TokenType::String, "String");
        assert_token(&tokens[2], TokenType::Vector, "Vector");
        assert_token(&tokens[4], TokenType::I32, "i32");
    }

    #[test]
    fn test_43_maximal_munch_precedence() {
        // Should parse as LessThanOrEqual (<=), not LessThan (<) and Assign (=)
        let tokens = parse("<=".to_string());
        assert_token(&tokens[0], TokenType::LessThanOrEqual, "<=");
        assert_eq!(tokens.len(), 2); // Includes EOF
    }

    #[test]
    fn test_44_maximal_munch_failure_case() {
        // "<" and "-" do not make a multi-char operator, should be two tokens
        let tokens = parse("<-".to_string());
        assert_token(&tokens[0], TokenType::LessThan, "<");
        assert_token(&tokens[1], TokenType::Minus, "-");
    }

    #[test]
    fn test_45_complex_expression() {
        let tokens = parse("if(x==10){return;}".to_string());
        assert_token(&tokens[0], TokenType::If, "if");
        assert_token(&tokens[1], TokenType::LeftParen, "(");
        assert_token(&tokens[2], TokenType::Identifier, "x");
        assert_token(&tokens[3], TokenType::Equals, "==");
        assert_token(&tokens[4], TokenType::IntegerLiteral, "10");
        // Parsed value should be an integer
        assert_eq!(tokens[4].parsed_value, TokenValue::Integer(10));
        assert_token(&tokens[5], TokenType::RightParen, ")");
        assert_token(&tokens[6], TokenType::LeftBrace, "{");
        assert_token(&tokens[7], TokenType::Return, "return");
        assert_token(&tokens[8], TokenType::Semicolon, ";");
        assert_token(&tokens[9], TokenType::RightBrace, "}");
    }

    #[test]
    fn test_46_comment_ends_at_eof() {
        // Ensure comment parsing doesn't crash if file ends without newline
        let tokens = parse("// comment at end".to_string());
        assert_token(&tokens[0], TokenType::Comment, " comment at end");
    }

    #[test]
    fn test_47_unterminated_string_at_eof() {
        // This tests the loop condition in handle_string_literal
        // Based on logic, it stops at EOF.
        let tokens = parse("\"unterminated".to_string());
        assert_token(&tokens[0], TokenType::StringLiteral, "unterminated");
    }

    #[test]
    fn test_48_float_starts_with_zero() {
        let tokens = parse("0.5".to_string());
        assert_token(&tokens[0], TokenType::FloatLiteral, "0.5");
        assert_eq!(tokens[0].parsed_value, TokenValue::Float(0.5));
    }

    #[test]
    fn test_49_multiple_operators_no_space() {
        let tokens = parse("a+b*c".to_string());
        assert_token(&tokens[0], TokenType::Identifier, "a");
        assert_token(&tokens[1], TokenType::Plus, "+");
        assert_token(&tokens[2], TokenType::Identifier, "b");
        assert_token(&tokens[3], TokenType::Multiply, "*");
        assert_token(&tokens[4], TokenType::Identifier, "c");
    }

    #[test]
    fn test_50_bitwise_ops() {
        let tokens = parse("& | ^".to_string());
        assert_token(&tokens[0], TokenType::BitwiseAnd, "&");
        assert_token(&tokens[2], TokenType::BitwiseOr, "|");
        assert_token(&tokens[4], TokenType::BitwiseXor, "^");
    }
}
