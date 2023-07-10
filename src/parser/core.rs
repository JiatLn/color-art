use crate::ColorSpace;
use anyhow::Result;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Value,
    Identifier,
    LeftParen,
    RightParen,
    Comma,
    Whitespace,
    EOF,
}

#[derive(Debug)]
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
            kind: TokenKind::EOF,
            value: String::new(),
        });

        self
    }

    pub fn validate(&mut self) -> Result<()> {
        dbg!(&self.tokens);

        let mut stack = Vec::new();

        while let Some(token) = self.tokens.get(self.current) {
            match token.kind {
                TokenKind::LeftParen => {
                    stack.push(token);
                }
                TokenKind::RightParen => {
                    if let Some(_) = stack.pop() {
                        // do nothing
                    } else {
                        anyhow::bail!("Unmatched right parenthesis");
                    }
                }
                TokenKind::Value => {
                    if token.value.contains('%') {
                        let value = token.value.replace("%", "");
                        if let Ok(value) = value.parse::<f64>() {
                            self.values.push(value / 100.0);
                        } else {
                            anyhow::bail!("Invalid value");
                        }
                    } else {
                        if let Ok(value) = token.value.parse::<f64>() {
                            self.values.push(value);
                        } else {
                            anyhow::bail!("Invalid value");
                        }
                    }
                }
                TokenKind::Identifier => {
                    let color_space = ColorSpace::from(&token.value);
                    if color_space == ColorSpace::Unknown {
                        anyhow::bail!("Invalid input");
                    } else {
                        self.color_space = color_space;
                    }
                }
                _ => {}
            }

            self.current += 1;
        }

        match self.color_space {
            ColorSpace::Unknown => anyhow::bail!("No color space found"),
            _ => {
                if self.values.len() != self.color_space.value_count() {
                    anyhow::bail!("Invalid number of values");
                }
                self.color_space.valid(&self.values)?;
            }
        }

        if !self.values.is_empty() && stack.is_empty() {
            Ok(())
        } else if self.values.is_empty() {
            anyhow::bail!("No values found");
        } else {
            anyhow::bail!("Unmatched left parenthesis");
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