use crate::ast::{Position, Token, TokenKind, Word, ParseInfo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexError {
    UnexpectedEof,
    BadEscape(String, usize, usize),
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnexpectedEof => write!(f, "unexpected end of input"),
            LexError::BadEscape(filename, line, column) => write!(f, "invalid escape sequence at {}:{}:{}", filename, line, column),
        }
    }
}

impl std::error::Error for LexError {}

fn word_info(kind: TokenKind, _name: &str) -> ParseInfo {
    match kind {
        TokenKind::Comma => ParseInfo::new(1, 0, 1),
        TokenKind::Semicolon => ParseInfo::new(1, 0, 1),
        TokenKind::Equal => ParseInfo::new(2, 0, 2),
        TokenKind::ColonEqual => ParseInfo::new(2, 0, 2),
        TokenKind::Arrow => ParseInfo::new(3, 0, 3),
        TokenKind::LeftParen => ParseInfo::new(10, 0, 0),
        TokenKind::LeftBrace => ParseInfo::new(10, 0, 0),
        TokenKind::LeftBracket => ParseInfo::new(10, 0, 0),
        _ => ParseInfo::new(0, 0, 0),
    }
}

pub fn tokenize(input: &str, filename: impl Into<String>) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let filename = filename.into();
    let mut line = 1;
    let mut column = 1;
    let mut chars = input.chars().peekable();
    let mut last_was_newline = true;

    while let Some(&ch) = chars.peek() {
        match ch {
            c if c.is_whitespace() => {
                chars.next();
                if c == '\n' {
                    line += 1;
                    column = 1;
                    last_was_newline = true;
                } else {
                    column += 1;
                }
            }
            ';' => {
                let pos = Position::new(&filename, line, column);
                tokens.push(Token::new(
                    Word::new(";", TokenKind::Semicolon, word_info(TokenKind::Semicolon, ";")),
                    pos,
                    last_was_newline,
                ));
                chars.next();
                column += 1;
                last_was_newline = false;
            }
            ',' => {
                let pos = Position::new(&filename, line, column);
                tokens.push(Token::new(
                    Word::new(",", TokenKind::Comma, word_info(TokenKind::Comma, ",")),
                    pos,
                    last_was_newline,
                ));
                chars.next();
                column += 1;
                last_was_newline = false;
            }
            '(' => {
                let pos = Position::new(&filename, line, column);
                tokens.push(Token::new(
                    Word::new("(", TokenKind::LeftParen, word_info(TokenKind::LeftParen, "(")),
                    pos,
                    last_was_newline,
                ));
                chars.next();
                column += 1;
                last_was_newline = false;
            }
            ')' => {
                let pos = Position::new(&filename, line, column);
                tokens.push(Token::new(
                    Word::new(")", TokenKind::RightParen, word_info(TokenKind::RightParen, ")")),
                    pos,
                    last_was_newline,
                ));
                chars.next();
                column += 1;
                last_was_newline = false;
            }
            '{' => {
                let pos = Position::new(&filename, line, column);
                tokens.push(Token::new(
                    Word::new("{", TokenKind::LeftBrace, word_info(TokenKind::LeftBrace, "{")),
                    pos,
                    last_was_newline,
                ));
                chars.next();
                column += 1;
                last_was_newline = false;
            }
            '}' => {
                let pos = Position::new(&filename, line, column);
                tokens.push(Token::new(
                    Word::new("}", TokenKind::RightBrace, word_info(TokenKind::RightBrace, "}")),
                    pos,
                    last_was_newline,
                ));
                chars.next();
                column += 1;
                last_was_newline = false;
            }
            '[' => {
                let pos = Position::new(&filename, line, column);
                tokens.push(Token::new(
                    Word::new("[", TokenKind::LeftBracket, word_info(TokenKind::LeftBracket, "[")),
                    pos,
                    last_was_newline,
                ));
                chars.next();
                column += 1;
                last_was_newline = false;
            }
            ']' => {
                let pos = Position::new(&filename, line, column);
                tokens.push(Token::new(
                    Word::new("]", TokenKind::RightBracket, word_info(TokenKind::RightBracket, "]")),
                    pos,
                    last_was_newline,
                ));
                chars.next();
                column += 1;
                last_was_newline = false;
            }
            ':' => {
                chars.next();
                column += 1;
                if let Some(&'=') = chars.peek() {
                    let pos = Position::new(&filename, line, column - 1);
                    chars.next();
                    tokens.push(Token::new(
                        Word::new(":=", TokenKind::ColonEqual, word_info(TokenKind::ColonEqual, ":=")),
                        pos,
                        last_was_newline,
                    ));
                    column += 1;
                    last_was_newline = false;
                } else {
                    let pos = Position::new(&filename, line, column - 1);
                    tokens.push(Token::new(
                        Word::new(":", TokenKind::Symbol, word_info(TokenKind::Symbol, ":")),
                        pos,
                        last_was_newline,
                    ));
                    last_was_newline = false;
                }
            }
            '=' => {
                let pos = Position::new(&filename, line, column);
                tokens.push(Token::new(
                    Word::new("=", TokenKind::Equal, word_info(TokenKind::Equal, "=")),
                    pos,
                    last_was_newline,
                ));
                chars.next();
                column += 1;
                last_was_newline = false;
            }
            '-' => {
                let mut cursor = chars.clone();
                cursor.next();
                if cursor.next() == Some('>') {
                    let pos = Position::new(&filename, line, column);
                    chars.next();
                    chars.next();
                    tokens.push(Token::new(
                        Word::new("->", TokenKind::Arrow, word_info(TokenKind::Arrow, "->")),
                        pos,
                        last_was_newline,
                    ));
                    column += 2;
                } else {
                    let pos = Position::new(&filename, line, column);
                    tokens.push(Token::new(
                        Word::new("-", TokenKind::Symbol, word_info(TokenKind::Symbol, "-")),
                        pos,
                        last_was_newline,
                    ));
                    chars.next();
                    column += 1;
                }
                last_was_newline = false;
            }
            '.' => {
                let pos = Position::new(&filename, line, column);
                tokens.push(Token::new(
                    Word::new(".", TokenKind::Dot, word_info(TokenKind::Dot, ".")),
                    pos,
                    last_was_newline,
                ));
                chars.next();
                column += 1;
                last_was_newline = false;
            }
            '#' => {
                let pos = Position::new(&filename, line, column);
                tokens.push(Token::new(
                    Word::new("#", TokenKind::Hash, word_info(TokenKind::Hash, "#")),
                    pos,
                    last_was_newline,
                ));
                chars.next();
                column += 1;
                last_was_newline = false;
            }
            '"' => {
                let start_line = line;
                let start_col = column;
                chars.next();
                column += 1;
                let mut value = String::new();
                while let Some(c) = chars.next() {
                    column += 1;
                    match c {
                        '"' => break,
                        '\\' => {
                            let escaped = chars.next().ok_or_else(|| LexError::BadEscape(filename.clone(), line, column))?;
                            column += 1;
                            value.push(match escaped {
                                'n' => '\n',
                                'r' => '\r',
                                't' => '\t',
                                '"' => '"',
                                '\\' => '\\',
                                other => other,
                            });
                        }
                        _ => value.push(c),
                    }
                }
                let pos = Position::new(&filename, start_line, start_col);
                tokens.push(Token::new(
                    Word::new(value, TokenKind::StringLiteral, word_info(TokenKind::StringLiteral, "string")),
                    pos,
                    last_was_newline,
                ));
                last_was_newline = false;
            }
            c if c.is_ascii_digit() => {
                let pos = Position::new(&filename, line, column);
                let mut value = String::new();
                while let Some(&digit) = chars.peek() {
                    if digit.is_ascii_digit() {
                        value.push(digit);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new(
                    Word::new(value, TokenKind::Integer, word_info(TokenKind::Integer, "integer")),
                    pos,
                    last_was_newline,
                ));
                last_was_newline = false;
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let pos = Position::new(&filename, line, column);
                let mut value = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_alphanumeric() || ch == '_' || ch == '\'' {
                        value.push(ch);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }
                tokens.push(Token::new(
                    Word::new(value.clone(), TokenKind::Identifier, word_info(TokenKind::Identifier, &value)),
                    pos,
                    last_was_newline,
                ));
                last_was_newline = false;
            }
            _ => {
                let pos = Position::new(&filename, line, column);
                let ch = chars.next().unwrap();
                column += 1;
                tokens.push(Token::new(
                    Word::new(ch.to_string(), TokenKind::Symbol, word_info(TokenKind::Symbol, &ch.to_string())),
                    pos,
                    last_was_newline,
                ));
                last_was_newline = false;
            }
        }
    }
    let pos = Position::new(&filename, line, column);
    tokens.push(Token::new(
        Word::new("<EOF>", TokenKind::EndOfFile, word_info(TokenKind::EndOfFile, "<EOF>")),
        pos,
        false,
    ));
    Ok(tokens)
}
