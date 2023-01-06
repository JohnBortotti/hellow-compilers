#[derive(Debug)]
pub enum TokenType {
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
  Bang, Equal, EqualEqual, BangEqual, LessEqual, GreaterEqual, Greater,  Less,

  Identifier, String, Number,

  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Let, While,

  Space,

  Break,

  Ignore,

  Eof
}

#[derive(Debug)]
pub struct Token {
  pub _type: TokenType,
  pub lexeme: String,
  line: u32,
}

impl Token {
  pub fn new(_type: TokenType, lexeme: String, line: u32) -> Self {
    Token {_type, lexeme, line}
  }
}
