use crate::{ColorSpace, Error};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TokenKind {
    Value,
    Identifier,
    LeftParen,
    RightParen,
    Comma,
    Whitespace,
    Eof,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub values: Vec<f64>,
    pub color_space: ColorSpace,
}

type PeekableChars<'a> = std::iter::Peekable<std::str::Chars<'a>>;

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
            values: Vec::new(),
            color_space: ColorSpace::Unknown,
        }
    }
    pub fn tokenize(&mut self, input: &str) -> &mut Self {
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' | '%' | '.' | '-' => self.tokenize_number(&mut chars),
                'a'..='z' | 'A'..='Z' => self.tokenize_identifier(&mut chars),
                '(' => self.tokenize_left_paren(&mut chars),
                ')' => self.tokenize_right_paren(&mut chars),
                ',' => self.tokenize_comma(&mut chars),
                ' ' | '\t' | '\n' => self.tokenize_whitespace(&mut chars),
                _ => self.tokenize_operator(&mut chars),
            }
        }

        self.tokens.push(Token {
            kind: TokenKind::Eof,
            value: String::new(),
        });

        self
    }

    pub fn validate(&mut self) -> Result<(), Error> {
        let mut stack = Vec::new();

        while let Some(token) = self.tokens.get(self.current) {
            match token.kind {
                TokenKind::LeftParen => {
                    stack.push(token);
                }
                TokenKind::RightParen => {
                    if stack.pop().is_some() {
                        // do nothing
                    } else {
                        return Err(Error::ColorParserError(
                            "Unmatched right parenthesis".to_string(),
                        ));
                    }
                }
                TokenKind::Value => {
                    if token.value.contains('%') {
                        let value = token.value.replace('%', "");
                        if let Ok(value) = value.parse::<f64>() {
                            self.values.push(value / 100.0);
                        } else {
                            return Err(Error::ColorParserError("Invalid value".to_string()));
                        }
                    } else if let Ok(value) = token.value.parse::<f64>() {
                        self.values.push(value);
                    } else {
                        return Err(Error::ColorParserError("Invalid value".to_string()));
                    }
                }
                TokenKind::Identifier => {
                    let color_space = ColorSpace::from(&token.value);
                    if color_space == ColorSpace::Unknown {
                        return Err(Error::ColorParserError("Invalid input".to_string()));
                    } else {
                        self.color_space = color_space;
                    }
                }
                _ => {}
            }

            self.current += 1;
        }

        match self.color_space {
            ColorSpace::Unknown => {
                return Err(Error::ColorParserError("No color space found".to_string()))
            }
            _ => {
                if self.values.len() != self.color_space.value_count() {
                    return Err(Error::ColorParserError(
                        "Invalid number of values".to_string(),
                    ));
                }
                self.color_space.valid(&self.values)?;
            }
        }

        if !self.values.is_empty() && stack.is_empty() {
            Ok(())
        } else if self.values.is_empty() {
            Err(Error::ColorParserError("No values found".to_string()))
        } else {
            Err(Error::ColorParserError(
                "Unmatched left parenthesis".to_string(),
            ))
        }
    }

    fn tokenize_number(&mut self, chars: &mut PeekableChars) {
        let mut value = String::new();

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' | '.' => {
                    value.push(c);
                    chars.next();
                }
                '%' => {
                    // % must be the last character in the string
                    value.push(c);
                    chars.next();
                    break;
                }
                '-' => {
                    if !value.is_empty() {
                        break;
                    }
                    // - must be the first character in the string
                    value.push(c);
                    chars.next();
                }
                _ => {
                    break;
                }
            }
        }

        self.tokens.push(Token {
            kind: TokenKind::Value,
            value,
        });
    }

    fn tokenize_identifier(&mut self, chars: &mut PeekableChars) {
        let mut value = String::new();

        while let Some(&c) = chars.peek() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    value.push(c);
                    chars.next();
                }
                _ => {
                    break;
                }
            }
        }

        self.tokens.push(Token {
            kind: TokenKind::Identifier,
            value,
        });
    }

    fn tokenize_left_paren(&mut self, chars: &mut PeekableChars) {
        chars.next();

        self.tokens.push(Token {
            kind: TokenKind::LeftParen,
            value: String::from("("),
        });
    }

    fn tokenize_right_paren(&mut self, chars: &mut PeekableChars) {
        chars.next();

        self.tokens.push(Token {
            kind: TokenKind::RightParen,
            value: String::from(")"),
        });
    }

    fn tokenize_comma(&mut self, chars: &mut PeekableChars) {
        chars.next();

        self.tokens.push(Token {
            kind: TokenKind::Comma,
            value: String::from(","),
        });
    }

    fn tokenize_whitespace(&mut self, chars: &mut PeekableChars) {
        chars.next();

        self.tokens.push(Token {
            kind: TokenKind::Whitespace,
            value: String::from(" "),
        });
    }

    fn tokenize_operator(&self, chars: &mut PeekableChars) {
        chars.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input = "rgb(255, 255, 255)";

        let mut parser = Parser::new();
        parser.tokenize(input);

        assert!(parser.validate().is_ok());

        let color_space = parser.color_space;
        assert_eq!(color_space, ColorSpace::RGB);

        let values = parser.values;
        assert_eq!(values, vec![255.0, 255.0, 255.0]);

        let input = "hsl(60, 80%, 50%)";

        let mut parser = Parser::new();
        parser.tokenize(input);

        assert!(parser.validate().is_ok());
    }
}
