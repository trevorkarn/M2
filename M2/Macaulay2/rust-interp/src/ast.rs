#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub filename: String,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(filename: impl Into<String>, line: usize, column: usize) -> Self {
        Self { filename: filename.into(), line, column }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    Integer,
    StringLiteral,
    Symbol,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Equal,
    ColonEqual,
    Arrow,
    Dot,
    Hash,
    EndOfFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseInfo {
    pub precedence: i32,
    pub unary_strength: i32,
    pub binary_strength: i32,
}

impl ParseInfo {
    pub const fn new(precedence: i32, unary_strength: i32, binary_strength: i32) -> Self {
        Self { precedence, unary_strength, binary_strength }
    }
}

impl Default for ParseInfo {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    pub name: String,
    pub kind: TokenKind,
    pub parse: ParseInfo,
}

impl Word {
    pub fn new(name: impl Into<String>, kind: TokenKind, parse: ParseInfo) -> Self {
        Self { name: name.into(), kind, parse }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub word: Word,
    pub position: Position,
    pub follows_newline: bool,
}

impl Token {
    pub fn new(word: Word, position: Position, follows_newline: bool) -> Self {
        Self { word, position, follows_newline }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseTree {
    Token(Token),
    Parentheses { left: Token, contents: Box<ParseTree>, right: Token },
    EmptyParentheses { left: Token, right: Token },
    Adjacent { lhs: Box<ParseTree>, rhs: Box<ParseTree> },
    Arrow { lhs: Box<ParseTree>, rhs: Box<ParseTree> },
    Unary { operator: Token, rhs: Box<ParseTree> },
    Binary { lhs: Box<ParseTree>, operator: Token, rhs: Box<ParseTree> },
    Postfix { lhs: Box<ParseTree>, operator: Token },
    Dummy(Position),
}

impl ParseTree {
    pub fn position(&self) -> Position {
        match self {
            ParseTree::Token(token) => token.position.clone(),
            ParseTree::Parentheses { left, .. } => left.position.clone(),
            ParseTree::EmptyParentheses { left, .. } => left.position.clone(),
            ParseTree::Adjacent { lhs, .. } => lhs.position(),
            ParseTree::Arrow { lhs, .. } => lhs.position(),
            ParseTree::Unary { operator, .. } => operator.position.clone(),
            ParseTree::Binary { lhs, .. } => lhs.position(),
            ParseTree::Postfix { lhs, .. } => lhs.position(),
            ParseTree::Dummy(pos) => pos.clone(),
        }
    }
}
