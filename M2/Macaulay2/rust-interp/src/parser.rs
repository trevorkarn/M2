use crate::ast::{ParseTree, Position, Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedEof(Position),
    UnexpectedToken(Token),
    MismatchedDelimiter(Position),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedEof(pos) => write!(f, "unexpected end of input at {}:{}:{}", pos.filename, pos.line, pos.column),
            ParseError::UnexpectedToken(token) => write!(f, "unexpected token {:?} at {}:{}:{}", token.word.name, token.position.filename, token.position.line, token.position.column),
            ParseError::MismatchedDelimiter(pos) => write!(f, "mismatched delimiter at {}:{}:{}", pos.filename, pos.line, pos.column),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse(tokens: &[Token]) -> Result<ParseTree, ParseError> {
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expression(0)?;
    if parser.peek().word.kind != TokenKind::EndOfFile {
        return Err(ParseError::UnexpectedToken(parser.peek().clone()));
    }
    Ok(expr)
}

struct Parser<'a> {
    tokens: &'a [Token],
    offset: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, offset: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.offset]
    }

    fn bump(&mut self) {
        if self.offset + 1 < self.tokens.len() {
            self.offset += 1;
        }
    }

    fn next(&mut self) -> Token {
        let token = self.peek().clone();
        self.bump();
        token
    }

    fn parse_expression(&mut self, precedence: i32) -> Result<ParseTree, ParseError> {
        let mut left = self.parse_prefix()?;
        while self.peek().word.kind != TokenKind::EndOfFile
            && self.peek().word.parse.precedence > precedence
        {
            left = self.parse_infix(left)?;
        }
        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<ParseTree, ParseError> {
        let token = self.next();
        match token.word.kind {
            TokenKind::Identifier | TokenKind::Integer | TokenKind::StringLiteral | TokenKind::Symbol => Ok(ParseTree::Token(token)),
            TokenKind::LeftParen => self.parse_parentheses(token),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn parse_parentheses(&mut self, left: Token) -> Result<ParseTree, ParseError> {
        if self.peek().word.kind == TokenKind::RightParen {
            let right = self.next();
            return Ok(ParseTree::EmptyParentheses { left, right });
        }

        let mut expr = self.parse_expression(0)?;
        while self.peek().word.kind == TokenKind::Comma {
            let operator = self.next();
            let rhs = self.parse_expression(operator.word.parse.precedence)?;
            expr = ParseTree::Adjacent { lhs: Box::new(expr), rhs: Box::new(rhs) };
        }

        if self.peek().word.kind != TokenKind::RightParen {
            return Err(ParseError::MismatchedDelimiter(left.position));
        }
        let right = self.next();
        Ok(ParseTree::Parentheses { left, contents: Box::new(expr), right })
    }

    fn parse_infix(&mut self, lhs: ParseTree) -> Result<ParseTree, ParseError> {
        let operator = self.next();
        let _prec = operator.word.parse.precedence;
        let rhs = self.parse_expression(operator.word.parse.binary_strength)?;
        Ok(ParseTree::Binary { lhs: Box::new(lhs), operator, rhs: Box::new(rhs) })
    }
}
