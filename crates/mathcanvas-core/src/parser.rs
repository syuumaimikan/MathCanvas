use crate::ast::Expression;
use crate::error::CoreError;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LParen,
    RParen,
    Comma,
}

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
                continue;
            }

            match c {
                '+' => {
                    self.chars.next();
                    return Some(Token::Plus);
                }
                '-' => {
                    self.chars.next();
                    return Some(Token::Minus);
                }
                '*' | '×' => {
                    self.chars.next();
                    return Some(Token::Multiply);
                }
                '/' | '÷' => {
                    self.chars.next();
                    return Some(Token::Divide);
                }
                '^' => {
                    self.chars.next();
                    return Some(Token::Power);
                }
                '(' => {
                    self.chars.next();
                    return Some(Token::LParen);
                }
                ')' => {
                    self.chars.next();
                    return Some(Token::RParen);
                }
                ',' => {
                    self.chars.next();
                    return Some(Token::Comma);
                }
                '0'..='9' | '.' => {
                    let mut num_str = String::new();
                    while let Some(&ch) = self.chars.peek() {
                        if ch.is_ascii_digit() || ch == '.' {
                            num_str.push(ch);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    return Some(Token::Number(num_str.parse().unwrap_or(0.0)));
                }
                'a'..='z' | 'A'..='Z' | 'α'..='ω' | 'Α'..='Ω' | '√' => {
                    let mut id = String::new();
                    while let Some(&ch) = self.chars.peek() {
                        if ch.is_alphanumeric() || ch == 'π' || ch == '√' {
                            id.push(ch);
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    return Some(Token::Identifier(id));
                }
                _ => {
                    self.chars.next(); // Skip unknown for now
                }
            }
        }
        None
    }

    fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(tok) = self.next_token() {
            tokens.push(tok);
        }
        tokens
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn parse(input: &str) -> Result<Expression, CoreError> {
        let lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser { tokens, pos: 0 };
        parser.parse_expression()
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn parse_expression(&mut self) -> Result<Expression, CoreError> {
        self.parse_addition()
    }

    fn parse_addition(&mut self) -> Result<Expression, CoreError> {
        let mut left = self.parse_multiplication()?;
        while let Some(tok) = self.current() {
            match tok {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    left = Expression::Add(Box::new(left), Box::new(right));
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    left = Expression::Subtract(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expression, CoreError> {
        let mut left = self.parse_power()?;
        while let Some(tok) = self.current() {
            match tok {
                Token::Multiply => {
                    self.advance();
                    let right = self.parse_power()?;
                    left = Expression::Multiply(Box::new(left), Box::new(right));
                }
                Token::Divide => {
                    self.advance();
                    let right = self.parse_power()?;
                    left = Expression::Divide(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expression, CoreError> {
        let mut left = self.parse_primary()?;
        while let Some(tok) = self.current() {
            if let Token::Power = tok {
                self.advance();
                let right = self.parse_primary()?; // Right associative usually, but simple left associative here for now
                left = Expression::Power(Box::new(left), Box::new(right));
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, CoreError> {
        let current = self.current().cloned();
        if let Some(tok) = current {
            match tok {
                Token::Number(n) => {
                    self.advance();
                    Ok(Expression::Number(n))
                }
                Token::Identifier(id) => {
                    self.advance();
                    // Check if it's a function call
                    if let Some(Token::LParen) = self.current() {
                        self.advance();
                        let mut args = Vec::new();
                        if let Some(Token::RParen) = self.current() {
                            self.advance();
                        } else {
                            args.push(self.parse_expression()?);
                            while let Some(Token::Comma) = self.current() {
                                self.advance();
                                args.push(self.parse_expression()?);
                            }
                            if let Some(Token::RParen) = self.current() {
                                self.advance();
                            } else {
                                return Err(CoreError::ParseError("Expected )".into()));
                            }
                        }
                        Ok(Expression::FunctionCall(id, args))
                    } else {
                        Ok(Expression::Variable(id))
                    }
                }
                Token::LParen => {
                    self.advance();
                    let expr = self.parse_expression()?;
                    if let Some(Token::RParen) = self.current() {
                        self.advance();
                        Ok(expr)
                    } else {
                        Err(CoreError::ParseError("Expected )".into()))
                    }
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_power()?;
                    Ok(Expression::Multiply(
                        Box::new(Expression::Number(-1.0)),
                        Box::new(right),
                    ))
                }
                _ => Err(CoreError::ParseError("Unexpected token".into())),
            }
        } else {
            Err(CoreError::ParseError("Unexpected end of input".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic() {
        let expr = Parser::parse("2 + 3 * 4").unwrap();
        assert_eq!(
            expr,
            Expression::Add(
                Box::new(Expression::Number(2.0)),
                Box::new(Expression::Multiply(
                    Box::new(Expression::Number(3.0)),
                    Box::new(Expression::Number(4.0))
                ))
            )
        );
    }

    #[test]
    fn test_parse_functions() {
        let expr = Parser::parse("sin(pi)").unwrap();
        assert_eq!(
            expr,
            Expression::FunctionCall(
                "sin".to_string(),
                vec![Expression::Variable("pi".to_string())]
            )
        );
    }
}
