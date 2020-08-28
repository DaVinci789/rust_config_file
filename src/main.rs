use std::io::Read;

#[derive(PartialEq, Clone, Copy)]
enum TokenType {
  TypeIdentifier,
  //Error,
  PossibleIdentifier,
  TypeList,
  Label,
  TypeAssignment,
  StringLiteral,
  NumberLiteral,
  BoolLiteral,
  Equals,
  CurlyBracketStart,
  CurlyBracketEnd,
  // SquareBracketStart,
  // SquareBracketEnd,
  TypeBool,
  TypeString,
  TypeNumber,
  TypeMap,
  //Identifier
}

#[derive(PartialEq, Clone)]
struct Token {
  token: String,
  tokentype: TokenType,
}

struct ASTNode {
  token: Token,
  children: std::vec::Vec<ASTNode>,
}

struct TokenTraverse {
  is_eof: bool,
  token_index: i64,
  current_token: Token,
}

impl TokenTraverse {
  pub fn new(tokens: &[Token]) -> TokenTraverse {
    TokenTraverse {
      is_eof : false,
      token_index : 0,
      current_token : tokens[0].clone(),
    }
  }
  pub fn token_is_literal(token: Token) -> bool {
    return token.tokentype == TokenType::NumberLiteral || token.tokentype == TokenType::StringLiteral || token.tokentype == TokenType::BoolLiteral;
  }
  fn accept(&mut self, tokentype: TokenType, tokens: &[Token]) -> bool {
    if self.current_token.tokentype == tokentype {
      self.next_token(tokens);
      return true;
    }
    return false;
  }
  fn expect(&mut self, tokentype: TokenType, tokens: &[Token]) -> bool {
    if self.accept(tokentype, tokens) {
      return true;
    }
    // error handling
    return false;
  }
  fn definition(&mut self, tokens: &[Token]) {
    print!("{} ", self.current_token.token);
    if !self.expect(TokenType::PossibleIdentifier, tokens) {
      println!("Error Invalid Syntax: {}", self.current_token);
      return;
    }

    self.next_token(tokens);
    loop {
      if self.current_token.tokentype == TokenType::Equals {
        self.next_token(tokens);
      }
      if TokenTraverse::token_is_literal(self.current_token.clone()) {
        break;
      }
    }
    println!("{}", self.current_token.token);
  }
  fn block(&mut self, tokens: &[Token]) {
    if self.accept(TokenType::TypeIdentifier, tokens) {
      println!("Type: {}", self.current_token.token);
      println!("-------------");
      let sym1 = self.expect(TokenType::PossibleIdentifier, tokens);
      let sym2 = self.expect(TokenType::CurlyBracketStart, tokens);
      if !sym1 || !sym2 {
        println!("Error! Invalid Syntax: {}", self.current_token);
        return;
      }
      loop {
        self.definition(tokens);
        self.next_token(tokens);
        if self.current_token.tokentype == TokenType::CurlyBracketEnd {
          break;
        }
      }
      println!("-------------");
    }
  }
  fn next_token(&mut self, tokens: &[Token]) {
    self.token_index += 1;
    self.current_token = tokens[self.token_index as usize].clone();
  }
}



impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.tokentype {
      TokenType::TypeIdentifier => write!(f, "TypeIdentifier: {}", self.token), 
      TokenType::PossibleIdentifier => write!(f, "PossibleIdentifier: {}", self.token),
      TokenType::StringLiteral => write!(f, "StringLiteral: {}", self.token),
      TokenType::NumberLiteral => write!(f, "NumberLiteral: {}", self.token),
      TokenType::BoolLiteral => write!(f, "BoolLiteral: {}", self.token),
      TokenType::Label => write!(f, "Label: {}", self.token),
      TokenType::TypeBool => write!(f, "TypeBool: {}", self.token),
      TokenType::TypeString => write!(f, "StringLiteral: {}", self.token),
      TokenType::TypeNumber => write!(f, "TypeNumber: {}", self.token),
      TokenType::TypeList => write!(f, "TypeList: {}", self.token),
      TokenType::TypeMap => write!(f, "TypeMap: {}", self.token),
      TokenType::CurlyBracketStart => write!(f, "CurlyBracketStart: {}", self.token),
      TokenType::CurlyBracketEnd => write!(f, "CurlyBracketEnd: {}", self.token),
      TokenType::Equals => write!(f, "Equals: {}", self.token),
      TokenType::TypeAssignment => write!(f, "TypeAssignment: {}", self.token),
      // TokenType::SquareBracketStart => write!(f, "SquareBracketStart: {}", self.token),
      // TokenType::SquareBracketEnd => write!(f, "SquareBracketEnd: {}", self.token),
    }
  }
}

fn main() {
  let command_args: std::vec::Vec<std::string::String> = std::env::args().collect();
  let filename = &command_args[1];

  let mut file = std::fs::File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let chars: std::vec::Vec<_> = contents.chars().collect();
  
  let mut current_token = String::new();
  let mut tokens = vec![String::new()];
  let mut is_string_literal = false;
  for index in 0..chars.len() {
    if (chars[index] == ' ') || (chars[index] == '\n') {
      let test = current_token.clone();
      if test.trim().is_empty() {
        continue;
      }
      if is_string_literal == false {
        tokens.push(current_token.clone());
        current_token.clear();
        continue;
      }
      // Add space to string literal with space.
      current_token.push_str(" ");
      continue;
    }

    if chars[index].to_string() == "\"" {
      is_string_literal = !is_string_literal;
    }
    current_token.push_str(&chars[index].to_string());
  }
  // HACK: There's like a newline or something that gets added as the first element? 
  tokens.remove(0);

  let mut symbols = vec![];
  for token in &tokens {
    symbols.push(consume_token(token.to_string()));
  }
  construct_ast(&symbols.as_slice());
}

fn consume_token(token: String) -> Token {
  let token_as_str: &str = &(token.clone())[..];
  match token_as_str.chars().nth(0) {
    Some('"') => return Token {token: token, tokentype: TokenType::StringLiteral},
    Some('[') => return Token {token: token, tokentype: TokenType::Label},
    Some('{') => return Token {token: token, tokentype: TokenType::CurlyBracketStart},
    Some('}') => return Token {token: token, tokentype: TokenType::CurlyBracketEnd},
    Some('=') => return Token {token: token, tokentype: TokenType::Equals},
    Some(':') => return Token {token: token, tokentype: TokenType::TypeAssignment},
    Some('#'..='9') => return Token {token: token, tokentype: TokenType::NumberLiteral},
    _ => (),
  }

  match &token_as_str[0..4] {
    "true" => return Token {token: token, tokentype: TokenType::BoolLiteral},
    // HACK: This is the worst thing i've ever done.
    "fals" => return Token {token: token, tokentype: TokenType::BoolLiteral},
    _ => (),
  }


  match token_as_str {
    "type" => return Token {token: token, tokentype: TokenType::TypeIdentifier},
    "number" => return Token {token: token, tokentype: TokenType::TypeNumber},
    "bool" => return Token {token: token, tokentype: TokenType::TypeBool},
    "string" => return Token {token: token, tokentype: TokenType::TypeString},
    "map" => return Token {token: token, tokentype: TokenType::TypeMap},
    "list" => return Token {token: token, tokentype: TokenType::TypeList},
    _ => return Token {token: token, tokentype: TokenType::PossibleIdentifier,
    },
  }
}

fn construct_ast(tokens: &[Token]) -> std::vec::Vec<ASTNode> {
  let mut ast = vec![];

  let mut token_traversal = TokenTraverse::new(tokens);
  
  // while !token_traversal.is_eof {
  //   token_traversal.next_token(tokens);
  //   token_traversal.block(tokens);
  // }
  token_traversal.block(tokens);
  ast
}
