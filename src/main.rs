use std::fs;

mod parser;

// Basic Colors
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const ORANGE: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";

// "Bright" / "Bold" Variants (Better visibility)
const BRIGHT_RED: &str = "\x1b[91m";
const BRIGHT_GREEN: &str = "\x1b[92m";
const BRIGHT_ORANGE: &str = "\x1b[93m";
const BRIGHT_YELLOW: &str = "\x1b[93m";
const BRIGHT_BLUE: &str = "\x1b[94m";
const BRIGHT_MAGENTA: &str = "\x1b[95m";
const BRIGHT_CYAN: &str = "\x1b[96m";

// Special
const GRAY: &str = "\x1b[90m"; // Perfect for comments

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut source_files: Vec<String> = Vec::new();
    let mut print_hightlight = false;
    let mut show_token_start = false;
    let mut print_tokens = false;

    let mut i = 0;
    while i < args.len() - 1 {
        // Skip the first argument, which is the program name
        i += 1;
        let arg = args[i].as_str();
        match arg {
            "-o" => {
                let mut i = i;
                i -= 1;
                let output = args[i].as_str();
                println!("output: {}", output);
            }
            "-h" | "--help" => {
                println!("usage: zincc [options] <file.zc>");
                println!("options:");
                println!("  -o <output file>");
            }
            "--print-highlight" => {
                print_hightlight = true;
            }
            "--show-token-start" => {
                show_token_start = true;
            }
            "--print-tokens" => {
                print_tokens = true;
            }
            _ => {
                // Assume it's a file
                let file_name = arg;
                let contents = fs::read_to_string(file_name).unwrap();
                source_files.push(contents);
            }
        }
    }

    for source_code in source_files {
        if print_hightlight {
            println!();
            let tokens = parser::parse(source_code.clone());
            let lines: Vec<&str> = source_code.lines().collect();

            // Group tokens by line for faster lookup
            // We map line_number -> Vec<Token>
            let mut tokens_by_line: std::collections::HashMap<usize, Vec<&parser::Token>> =
                std::collections::HashMap::new();
            for token in &tokens {
                tokens_by_line.entry(token.line).or_default().push(token);
            }

            for (i, line_content) in lines.iter().enumerate() {
                let line_number = i + 1;

                // 1. Build the colored string for the source code line
                let mut colored_line = String::new();
                let line_tokens = tokens_by_line
                    .get(&line_number)
                    .map(|v| v.as_slice())
                    .unwrap_or(&[]);

                // We need to know which characters belong to which token to color them
                // This simple map tracks: index -> color_code
                let mut color_map = vec![RESET; line_content.len()];

                for token in line_tokens {
                    // Determine color based on token type
                    let color = match token.token_type {
                        parser::TokenType::Fn
                        | parser::TokenType::If
                        | parser::TokenType::Else
                        | parser::TokenType::While
                        | parser::TokenType::For
                        | parser::TokenType::Return
                        | parser::TokenType::Break
                        | parser::TokenType::Continue => BRIGHT_MAGENTA,

                        parser::TokenType::I32
                        | parser::TokenType::I64
                        | parser::TokenType::F32
                        | parser::TokenType::F64
                        | parser::TokenType::Bool
                        | parser::TokenType::String
                        | parser::TokenType::Char => CYAN,

                        parser::TokenType::StringLiteral | parser::TokenType::CharLiteral => GREEN,

                        parser::TokenType::IntegerLiteral | parser::TokenType::FloatLiteral => {
                            BRIGHT_GREEN
                        }

                        parser::TokenType::BoolLiteral => BRIGHT_BLUE,

                        parser::TokenType::Identifier => BRIGHT_YELLOW,

                        parser::TokenType::Comment
                        | parser::TokenType::DocComment
                        | parser::TokenType::BlockComment => GRAY,

                        parser::TokenType::Macro => BRIGHT_RED,

                        // Default
                        _ => RESET,
                    };

                    if color != RESET {
                        let start = token.column.saturating_sub(1);

                        // CALCULATE EXTRA LENGTH FOR DELIMITERS & ESCAPES
                        let mut extra_len = 0;

                        match token.token_type {
                            parser::TokenType::StringLiteral => {
                                extra_len = 2; // Account for surrounding quotes ""
                                // Scan the string for chars that are escaped in source
                                for c in token.value.chars() {
                                    match c {
                                        '\n' | '\t' | '\r' | '\\' | '"' => extra_len += 1,
                                        _ => {}
                                    }
                                }
                            }
                            parser::TokenType::Char => {
                                extra_len = 2; // Account for surrounding quotes ''
                                for c in token.value.chars() {
                                    match c {
                                        '\n' | '\t' | '\r' | '\\' | '\'' => extra_len += 1,
                                        _ => {}
                                    }
                                }
                            }
                            parser::TokenType::Comment => extra_len = 2, // //
                            parser::TokenType::DocComment => extra_len = 3, // ///
                            parser::TokenType::BlockComment => extra_len = 4, // /* */
                            _ => {}
                        };

                        // Add extra_len to the total end position
                        let end = start + token.value.len() + extra_len;

                        // Apply color to the range in the map
                        for j in start..std::cmp::min(end, color_map.len()) {
                            color_map[j] = color;
                        }
                    }
                }

                // Construct the colored string
                let mut current_color = RESET;
                for (idx, ch) in line_content.char_indices() {
                    let target_color = if idx < color_map.len() {
                        color_map[idx]
                    } else {
                        RESET
                    };

                    if target_color != current_color {
                        colored_line.push_str(target_color);
                        current_color = target_color;
                    }
                    colored_line.push(ch);
                }
                colored_line.push_str(RESET);

                // 2. Build the marker line (Same logic as yours)
                let len = std::cmp::max(line_content.len(), 1);
                let mut marker_line = vec![' '; len];

                for token in line_tokens {
                    if token.token_type == parser::TokenType::EOF {
                        continue;
                    }
                    let col_idx = token.column.saturating_sub(1);

                    if col_idx < marker_line.len() {
                        marker_line[col_idx] = '^';
                    } else {
                        marker_line.resize(col_idx + 1, ' ');
                        marker_line[col_idx] = '^';
                    }
                }

                // Print output
                println!("{:>4}: {}", line_number, colored_line);
                // We use the same color logic for markers to make it look cool (optional)
                if show_token_start {
                    println!("      {}", marker_line.into_iter().collect::<String>());
                }
            }
            println!();
        }
        if print_tokens {
            let tokens = parser::parse(source_code.clone());
            for token in tokens {
                println!("{:?}", token);
            }
        }
    }
}
