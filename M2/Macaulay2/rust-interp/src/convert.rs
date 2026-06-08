use crate::ast::{ParseTree, TokenKind, Token, Word, Position};

#[derive(Debug, Clone, PartialEq)]
pub enum Code {
    Null,
    Integer(String, Position),
    StringLiteral(String, Position),
    Identifier(String, Position),
    Unary { operator: Word, rhs: Box<Code>, position: Position },
    Binary { lhs: Box<Code>, operator: Word, rhs: Box<Code>, position: Position },
    Sequence { items: Vec<Code>, position: Position },
}

pub fn convert(tree: ParseTree) -> Code {
    convert_internal(tree)
}

fn convert_internal(tree: ParseTree) -> Code {
    match tree {
        ParseTree::Token(token) => convert_token(token),
        ParseTree::Parentheses { contents, left, .. } => {
            Code::Sequence { items: vec![convert_internal(*contents)], position: left.position.clone() }
        }
        ParseTree::EmptyParentheses { left, .. } => Code::Sequence { items: vec![], position: left.position },
        ParseTree::Adjacent { lhs, rhs } => {
            let lhs_code = convert_internal(*lhs);
            let rhs_code = convert_internal(*rhs);
            let rhs_position = rhs_code.position();
            let mut items = vec![];
            match lhs_code {
                Code::Sequence { items: mut lhs_items, .. } => items.append(&mut lhs_items),
                other => items.push(other),
            }
            match rhs_code {
                Code::Sequence { items: mut rhs_items, .. } => items.append(&mut rhs_items),
                other => items.push(other),
            }
            Code::Sequence { items, position: rhs_position }
        }
        ParseTree::Arrow { lhs, rhs } => {
            let position = lhs.position();
            Code::Binary { lhs: Box::new(convert_internal(*lhs)), operator: Word::new("->", TokenKind::Arrow, Default::default()), rhs: Box::new(convert_internal(*rhs)), position }
        }
        ParseTree::Unary { operator, rhs } => {
            let position = operator.position.clone();
            Code::Unary { operator: operator.word.clone(), rhs: Box::new(convert_internal(*rhs)), position }
        }
        ParseTree::Binary { lhs, operator, rhs } => {
            let position = lhs.position();
            Code::Binary { lhs: Box::new(convert_internal(*lhs)), operator: operator.word.clone(), rhs: Box::new(convert_internal(*rhs)), position }
        }
        ParseTree::Postfix { lhs, operator } => {
            let position = lhs.position();
            let lhs = convert_internal(*lhs);
            Code::Binary { lhs: Box::new(lhs), operator: operator.word.clone(), rhs: Box::new(Code::Null), position }
        }
        ParseTree::Dummy(_pos) => Code::Null,
    }
}

fn convert_token(token: Token) -> Code {
    match token.word.kind {
        TokenKind::Integer => Code::Integer(token.word.name, token.position),
        TokenKind::StringLiteral => Code::StringLiteral(token.word.name, token.position),
        TokenKind::Identifier => Code::Identifier(token.word.name, token.position),
        _ => Code::Identifier(token.word.name, token.position),
    }
}

trait CodePosition {
    fn position(&self) -> Position;
}

impl CodePosition for Code {
    fn position(&self) -> Position {
        match self {
            Code::Null => Position::new("<null>", 0, 0),
            Code::Integer(_, pos) => pos.clone(),
            Code::StringLiteral(_, pos) => pos.clone(),
            Code::Identifier(_, pos) => pos.clone(),
            Code::Unary { position, .. } => position.clone(),
            Code::Binary { position, .. } => position.clone(),
            Code::Sequence { position, .. } => position.clone(),
        }
    }
}
