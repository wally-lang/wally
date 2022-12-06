use crate::token::{
    Token,
    TokenKind,
};

pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = source.chars().peekable();
    let mut line = 1;
    let mut column = 1;

    let kws = {
        let mut m = std::collections::HashMap::new();
        m.insert("array", TokenKind::ArrayKw);
        m.insert("map", TokenKind::MapKw);
        m.insert("string", TokenKind::StringKw);
        m.insert("char", TokenKind::CharKw);
        m.insert("int", TokenKind::IntegerKw);
        m.insert("float", TokenKind::FloatKw);
        m.insert("bool", TokenKind::BoolKw);
        m.insert("true", TokenKind::TrueKw);
        m.insert("false", TokenKind::FalseKw);
        m.insert("null", TokenKind::NullKw);
        m.insert("var", TokenKind::VarKw);
        m.insert("const", TokenKind::ConstKw);
        m.insert("fn", TokenKind::FnKw);
        m.insert("class", TokenKind::ClassKw);
        m.insert("return", TokenKind::ReturnKw);
        m
    };

    while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Token::new(TokenKind::Plus, '+'.to_string(), line, column)),
            '-' => tokens.push(Token::new(TokenKind::Minus, '-'.to_string(), line, column)),
            '*' => tokens.push(Token::new(TokenKind::Star, '*'.to_string(), line, column)),
            '/' => tokens.push(Token::new(TokenKind::Slash, '/'.to_string(), line, column)),
            '%' => tokens.push(Token::new(TokenKind::Percent, '%'.to_string(), line, column)),
            '^' => tokens.push(Token::new(TokenKind::Caret, '^'.to_string(), line, column)),
            '!' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::new(TokenKind::BangEqual, "!=".to_string(), line, column));
                } else {
                    tokens.push(Token::new(TokenKind::Bang, '!'.to_string(), line, column));
                }
            }
            '=' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::new(TokenKind::EqualEqual, "==".to_string(), line, column));
                } else {
                    tokens.push(Token::new(TokenKind::Equal, '='.to_string(), line, column));
                }
            }
            '>' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::new(TokenKind::GreaterEqual, ">=".to_string(), line, column));
                } else {
                    tokens.push(Token::new(TokenKind::Greater, '>'.to_string(), line, column));
                }
            }
            '<' => {
                if let Some('=') = chars.peek() {
                    chars.next();
                    tokens.push(Token::new(TokenKind::LessEqual, "<=".to_string(), line, column));
                } else {
                    tokens.push(Token::new(TokenKind::Less, '<'.to_string(), line, column));
                }
            }
            '&' => {
                if let Some('&') = chars.peek() {
                    chars.next();
                    tokens.push(Token::new(TokenKind::And, "&&".to_string(), line, column));
                } else {
                    tokens.push(Token::new(TokenKind::Ampersand, '&'.to_string(), line, column));
                }
            }
            '|' => {
                if let Some('|') = chars.peek() {
                    chars.next();
                    tokens.push(Token::new(TokenKind::Or, "||".to_string(), line, column));
                } else {
                    tokens.push(Token::new(TokenKind::Pipe, '|'.to_string(), line, column));
                }
            }
            '(' => tokens.push(Token::new(TokenKind::LeftParen, '('.to_string(), line, column)),
            ')' => tokens.push(Token::new(TokenKind::RightParen, ')'.to_string(), line, column)),
            '{' => tokens.push(Token::new(TokenKind::LeftBrace, '{'.to_string(), line, column)),
            '}' => tokens.push(Token::new(TokenKind::RightBrace, '}'.to_string(), line, column)),
            '[' => tokens.push(Token::new(TokenKind::LeftBracket, '['.to_string(), line, column)),
            ']' => tokens.push(Token::new(TokenKind::RightBracket, ']'.to_string(), line, column)),
            ',' => tokens.push(Token::new(TokenKind::Comma, ','.to_string(), line, column)),
            '.' => tokens.push(Token::new(TokenKind::Dot, '.'.to_string(), line, column)),
            ':' => tokens.push(Token::new(TokenKind::Colon, ':'.to_string(), line, column)),
            ';' => tokens.push(Token::new(TokenKind::Semicolon, ';'.to_string(), line, column)),
            '?' => tokens.push(Token::new(TokenKind::Question, '?'.to_string(), line, column)),
            '@' => tokens.push(Token::new(TokenKind::At, '@'.to_string(), line, column)),
            '#' => tokens.push(Token::new(TokenKind::Hash, '#'.to_string(), line, column)),
            '$' => tokens.push(Token::new(TokenKind::Dollar, '$'.to_string(), line, column)),
            '~' => tokens.push(Token::new(TokenKind::Tilde, '~'.to_string(), line, column)),
            '\'' => {
                let mut value = String::new();
                while let Some(c) = chars.next() {
                    if c == '\'' {
                        break;
                    } else if c == '\\' {
                        if let Some(c) = chars.next() {
                            match c {
                                'n' => value.push('\n'),
                                'r' => value.push('\r'),
                                't' => value.push('\t'),
                                '0' => value.push('\0'),
                                '\'' => value.push('\''),
                                '"' => value.push('"'),
                                '\\' => value.push('\\'),
                                _ => {
                                    tokens.push(Token::new(
                                        TokenKind::Error(format!("Invalid escape sequence: \\{}", c.to_string())),
                                        c.to_string(),
                                        line,
                                        column,
                                    ));
                                    return tokens;
                                }
                            }
                        } else {
                            tokens.push(Token::new(TokenKind::Error("Invalid escape sequence: \\ at end of file".to_string()), c.to_string(), line, column));
                            return tokens;
                        }
                    } else {
                        value.push(c);
                    }
                }

                if value.len() == 1 {
                    tokens.push(Token::new(TokenKind::Character(value.chars().next().unwrap()), value.to_string(), line, column));
                } else {
                    tokens.push(Token::new(TokenKind::Error(format!("Invalid character literal: '{}'", value)), value.to_string(), line, column));
                }
            }
            '"' => {
                let mut value = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    } else if c == '\\' {
                        if let Some(c) = chars.next() {
                            match c {
                                'n' => value.push('\n'),
                                'r' => value.push('\r'),
                                't' => value.push('\t'),
                                '0' => value.push('\0'),
                                '\'' => value.push('\''),
                                '"' => value.push('"'),
                                '\\' => value.push('\\'),
                                _ => {
                                    tokens.push(Token::new(TokenKind::Error(format!("Invalid escape sequence: '\\{}'", c)), c.to_string(), line, column));
                                    break;
                                }
                            }
                        } else {
                            tokens.push(Token::new(TokenKind::Error("Invalid escape sequence: '\\'".to_string()), c.to_string(), line, column));
                            break;
                        }
                    } else {
                        value.push(c);
                    }
                }
                tokens.push(Token::new(TokenKind::String(value.clone()), value.to_string(), line, column));
            }
            '_' => tokens.push(Token::new(TokenKind::Underscore, '_'.to_string(), line, column)),
            ' ' => (),
            '\t' => (),
            '\r' => (),
            '\n' => {
                line += 1;
                column = 1;
            }
            _ => {
                if c.is_digit(10) {
                    let mut number = String::new();
                    number.push(c);
                    while let Some(&c) = chars.peek() {
                        if c.is_digit(10) {
                            number.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    if let Some('.') = chars.peek() {
                        number.push('.');
                        chars.next();
                        while let Some(&c) = chars.peek() {
                            if c.is_digit(10) {
                                number.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        tokens.push(Token::new(TokenKind::Float(number.parse().unwrap()), number, line, column));
                    } else {
                        tokens.push(Token::new(TokenKind::Integer(number.parse().unwrap()), number, line, column));
                    }
                } else if c.is_alphabetic() || c == '_' {
                    let mut identifier = String::new();
                    identifier.push(c);
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            identifier.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    if kws.contains_key(&identifier.as_str()) {
                        tokens.push(Token::new(kws.get(identifier.as_str()).unwrap().clone(), identifier, line, column));
                    } else {
                        tokens.push(Token::new(TokenKind::Identifier(identifier.clone()), identifier, line, column));
                    }
                } else {
                    tokens.push(Token::new(TokenKind::Error(format!("Invalid character: '{}'", c)), c.to_string(), line, column));
                }
            }
        }
        column += 1;
    }

    // tokens.push(Token::new(TokenKind::Eof, "".to_string(), line, column));

    return tokens;
}