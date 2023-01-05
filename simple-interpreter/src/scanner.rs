mod token;

pub struct Scanner {
  source: String,
  pub tokens: Vec<token::Token>,
  pointer: u32,
  line: u32,
}

impl Scanner {
  pub fn new(source: String) -> Self {
    Scanner { source, tokens: vec![], pointer: 0, line: 1 }
  }

  fn peek(&self, position: u32) -> char {
    if self.is_at_end(self.pointer) { return '\0' };
    let position = position.try_into().unwrap();

    return self.source.chars().nth(position).unwrap();
  }

  fn match_next(&self, expected: char) -> bool {
    if self.is_at_end(self.pointer) { return false };
    if self.peek(self.pointer) != expected { return false };

    return true;
  }

  fn get_source_substring(&self, start: u32, end: u32) -> String {
    let s = usize::try_from(start).unwrap();
    let e = usize::try_from(end).unwrap();

    return self.source[s..=e].to_string()
  }

  fn create_token(&self, token_type: token::TokenType, value: String) -> token::Token {
    token::Token::new(token_type, value, self.line)
  }

  fn is_at_end(&self, position: u32) -> bool {
    let len = self.source.len().try_into().unwrap();

    return position >= len;
  }

  fn scan_lexeme(&self) -> (token::TokenType, String) {
    match self.peek(self.pointer) {
      '(' => (token::TokenType::LeftParen, "(".to_string()),
      ')' => (token::TokenType::RightParen, ")".to_string()),
      '{' => (token::TokenType::LeftBrace, "{".to_string()),
      '}' => (token::TokenType::RightBrace, "}".to_string()),
      ',' => (token::TokenType::Comma, ",".to_string()),
      '.' => (token::TokenType::Dot, ".".to_string()),
      '-' => (token::TokenType::Minus, "-".to_string()),
      '+' => (token::TokenType::Plus, "+".to_string()),
      ';' => (token::TokenType::Semicolon, ";".to_string()),
      '*' => (token::TokenType::Star, "*".to_string()),
      '!' => if self.match_next('=') { (token::TokenType::BangEqual, "!=".to_string()) } else { (token::TokenType::Bang, "!".to_string()) },
      '=' => if self.match_next('=') { (token::TokenType::EqualEqual, "==".to_string()) } else { (token::TokenType::Equal, "=".to_string()) },
      '<' => if self.match_next('=') { (token::TokenType::LessEqual, "<=".to_string()) } else { (token::TokenType::Less, "<".to_string()) },
      '>' => if self.match_next('=') { (token::TokenType::GreaterEqual, ">=".to_string()) } else { (token::TokenType::Greater, ">".to_string()) },
      '"' => {
               let mut pointer = self.pointer+1;
               let start = self.pointer;

               while self.peek(pointer) != '"' && !self.is_at_end(pointer) {
                 pointer += 1;
               }

               if self.is_at_end(pointer) {
                   self.error(self.line, self.pointer, "Unterminated String.".to_string());
               }

               (token::TokenType::String, self.get_source_substring(start, pointer)) 
      },
      ' ' => (token::TokenType::Ignore, "".to_string()),
      '\r' => (token::TokenType::Ignore, "".to_string()),
      '\t' => (token::TokenType::Ignore, "".to_string()),
      '\n' => (token::TokenType::Break, "".to_string()),
      a => 
         {
          if a.is_numeric() { 
              let mut pointer = self.pointer;
              let start = self.pointer;

              while self.peek(pointer).is_numeric() {
                  pointer +=1;
              } 

              if self.peek(pointer) == '.' && self.peek(pointer+1).is_numeric() {
                  pointer +=1;

                  while self.peek(pointer).is_numeric() { 
                      self.peek(pointer);
                      pointer +=1;
                  };
              }

              return (token::TokenType::Number, self.get_source_substring(start, pointer)) 
          } else { panic!("token not found at line {}, token: {}", self.line, self.peek(self.pointer)) }
        }
      }
  }

  fn error(&self, line: u32, position: u32, message: String) {
    println!("[line {}] Error: {} in position {}", line, message, position);
    std::process::exit(1);
  }

  pub fn scan_tokens(mut self) {
    while !self.is_at_end(self.pointer) {
      let (_type, value) = self.scan_lexeme();

      match &_type {
        | token::TokenType::String | token::TokenType::Number => self.pointer += u32::try_from(value.len()).unwrap(),
        | _   => self.pointer+=1,
      }

      let token = self.create_token(_type, value);
      self.tokens.push(token);
    }

    self.tokens.push(token::Token::new(token::TokenType::Eof, "".to_string(), self.line));

    dbg!(self.tokens);
  }


}
