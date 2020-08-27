use std::io::Read;

#[derive(PartialEq)]
enum TokenType {
  TypeIdentifier,
  //Error,
  PossibleIdentifier,
  TypeList,
  Label,
  TypeAssignment,
  StringLiteral,
  NumberLiteral,
  Equals,
  CurlyBracketStart,
  CurlyBracketEnd,
  // SquareBracketStart,
  // SquareBracketEnd,
  TypeString,
  TypeNumber,
  TypeMap,
  //Identifier
}

struct Token {
  token: String,
  tokentype: TokenType,
}

struct ASTNode {
  token: Token,
  children: std::vec::Vec<ASTNode>,
}

impl std::fmt::Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.tokentype {
      TokenType::TypeIdentifier => write!(f, "TypeIdentifier: {}", self.token), 
      TokenType::PossibleIdentifier => write!(f, "PossibleIdentifier: {}", self.token),
      TokenType::StringLiteral => write!(f, "StringLiteral: {}", self.token),
      TokenType::NumberLiteral => write!(f, "NumberLiteral: {}", self.token),
      TokenType::Label => write!(f, "Label: {}", self.token),
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
  match token_as_str {
    "type" => return Token {token: token, tokentype: TokenType::TypeIdentifier},
    "number" => return Token {token: token, tokentype: TokenType::TypeNumber},
    "string" => return Token {token: token, tokentype: TokenType::TypeString},
    "map" => return Token {token: token, tokentype: TokenType::TypeMap},
    "list" => return Token {token: token, tokentype: TokenType::TypeList},
    _ => return Token {token: token, tokentype: TokenType::PossibleIdentifier,
    },
  }
}

fn construct_ast(tokens: &[Token]) -> std::vec::Vec<ASTNode> {
  let mut ast = vec![];
  for token in tokens {
  }
  ast
}
