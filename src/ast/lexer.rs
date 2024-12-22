pub enum TokenKind {
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    EOF,
}

pub struct TextSpan {
    start: usize,
    end: usize,
}

impl TextSpan {
    pub fn new(start: usize, end: usize) -> Self {
        TextSpan { start, end }
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}
