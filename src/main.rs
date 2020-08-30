use std::fs;
use std::io::Read;

#[derive(PartialEq, Clone, Copy)]
enum TokenType {
    TypeIdentifier,
    //Error,
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
    PossibleIdentifier,
    EOF,
    Unintitialized,
}

#[derive(PartialEq, Clone)]
struct Token {
    token: String,
    tokentype: TokenType,
}

#[derive(PartialEq, Clone)]
struct Field {
    identifier: String,
    value: Token,
}

#[derive(Clone)]
struct Type {
    typename: String,
    fields: Vec<Field>,
}

#[derive(Clone)]
struct Object {
    object_name: String,
    object_type: Option<Type>,
    fields: Vec<Field>,
}

#[derive(Clone)]
struct ParsedFile {
    user_types: Vec<Type>,
    user_objects: Vec<Object>,
    user_fields: Vec<Field>,
}

struct TokenTraverse {
    current_type: Option<Type>,
    token_index: i64,
    current_token: Token,
    file: ParsedFile,
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
            TokenType::EOF => write!(f, "EOF"),
            TokenType::Unintitialized => write!(f, "Uninitialized Token"),
            // TokenType::SquareBracketStart => write!(f, "SquareBracketStart: {}", self.token),
            // TokenType::SquareBracketEnd => write!(f, "SquareBracketEnd: {}", self.token),
        }
    }
}

impl Field {
    pub fn new() -> Field {
        Field {
            identifier: String::new(),
            value: Token {
                token: String::new(),
                tokentype: TokenType::Unintitialized,
            },
        }
    }
}

impl Type {
    pub fn new() -> Type {
        Type {
            typename: String::new(),
            fields: Vec::new(),
        }
    }
}

impl std::cmp::PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.typename == other.typename
    }
}

impl Object {
    pub fn new() -> Object {
        Object {
            object_name: String::new(),
            object_type: None,
            fields: Vec::new(),
        }
    }
}

impl ParsedFile {
    pub fn new() -> ParsedFile {
        ParsedFile {
            user_types: Vec::new(),
            user_objects: Vec::new(),
            user_fields: Vec::new(),
        }
    }
}

impl TokenTraverse {
    pub fn new(tokens: &[Token]) -> TokenTraverse {
        TokenTraverse {
            token_index: 0,
            current_type: None,
            current_token: tokens[0].clone(),
            file: ParsedFile::new(),
        }
    }

    pub fn token_is_literal(token: Token) -> bool {
        return token.tokentype == TokenType::NumberLiteral
            || token.tokentype == TokenType::StringLiteral
            || token.tokentype == TokenType::BoolLiteral;
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

    fn definition(&mut self, tokens: &[Token]) -> Field {
        let mut field = Field::new();
        print!("{} ", self.current_token.token);
        field.identifier = self.current_token.token.clone();
        if !self.expect(TokenType::PossibleIdentifier, tokens) {
            println!("Error Invalid Syntax: {}", self.current_token);
        }

        self.next_token(tokens);
        loop {
            if self.current_token.tokentype == TokenType::Equals {
                self.next_token(tokens);
            }
            if TokenTraverse::token_is_literal(self.current_token.clone()) {
                field.value = self.current_token.clone();
                break;
            }
        }
        println!("{}", self.current_token.token);
        field
    }

    fn block(&mut self, tokens: &[Token]) -> ParsedFile {
        loop {
            // type
            if self.accept(TokenType::TypeIdentifier, tokens) {
                let mut usertype = Type::new();
                usertype.typename = self.current_token.token.clone();

                println!("Type: {}", self.current_token.token);
                println!("-------------");

                let sym1 = self.expect(TokenType::PossibleIdentifier, tokens);
                let sym2 = self.expect(TokenType::CurlyBracketStart, tokens);
                if !sym1 || !sym2 {
                    println!("Error! Invalid Syntax: {}", self.current_token);
                }
                loop {
                    usertype.fields.push(self.definition(tokens));
                    self.next_token(tokens);
                    if self.current_token.tokentype == TokenType::CurlyBracketEnd {
                        break;
                    }
                }
                println!("-------------");
                self.file.user_types.push(usertype);
            }

            // [Type Label]
            if self.current_token.tokentype == TokenType::Label {
                let manage_string = |type_string: String| -> String {
                    let mut correct_string = String::new();
                    for character in type_string.chars() {
                        match character {
                            '[' => continue,
                            ']' => break,
                            _ => (),
                        }
                        correct_string.push(character);
                    }
                    correct_string
                };

                let corrected_type = manage_string(self.current_token.token.clone());

                if !self.file.user_types.iter().any(|v| v.typename == corrected_type) {
                    panic!("Type {} not found", corrected_type);
                }

                println!(
                    "Setting type of rest of file to {}",
                    self.current_token.token
                );
                println!("-------------");

                self.current_type = Some(self.file.user_types.iter().filter(|v| v.typename == corrected_type).nth(0).unwrap().clone());
            }

            // Any top level identifier
            if self.current_token.tokentype == TokenType::PossibleIdentifier {
                if self.look_at_next_token(tokens).tokentype == TokenType::Equals {
                    let mut userfield = Field::new();
                    userfield.identifier = self.current_token.token.clone();
                    self.next_token(tokens);
                    self.next_token(tokens);
                    userfield.value = self.current_token.clone();
                    self.file.user_fields.push(userfield.clone());
                } else {
                    // Otherwise, this is an object
                    let mut userobject = Object::new();
                    userobject.object_name = self.current_token.token.clone();
                    print!("Object: {} ", self.current_token.token);

                    self.next_token(tokens);
                    if self.expect(TokenType::TypeAssignment, tokens) {
                        if !self.file.user_types.iter().any(|v: &Type| v.typename == self.current_token.token) {
                            panic!("Type {} does not exist", self.current_token.token);
                        }
                        let found_type = self.file.user_types.iter().filter(|v| v.typename == self.current_token.token).nth(0).unwrap();
                        userobject.object_type = Some(found_type.clone());//Some(self.current_token.token.clone());
                        println!("of type: {}", self.current_token.token);
                        self.next_token(tokens);
                    } else {
                        //userobject.object_type = Some(self.current_type.clone());
                        match &self.current_type {
                            Some(_) => userobject.object_type = self.current_type.clone(),
                            None => (),
                        }
                        print!("\n");
                    }
                    println!("-------------");

                    if !self.expect(TokenType::CurlyBracketStart, tokens) {
                        println!("Error! Missing Start Bracket");
                    }

                    loop {
                        userobject.fields.push(self.definition(tokens));
                        self.next_token(tokens);
                        if self.current_token.tokentype == TokenType::CurlyBracketEnd {
                            break;
                        }
                    }
                    println!("-------------");
                    self.file.user_objects.push(userobject);
                }
            }
            self.next_token(tokens);
            if self.current_token.tokentype == TokenType::EOF {
                break;
            }
        }
        self.file.clone()
    }

    fn look_at_next_token(&self, tokens: &[Token]) -> Token {
        tokens[(self.token_index + 1) as usize].clone()
    }
    
    fn next_token(&mut self, tokens: &[Token]) {
        self.token_index += 1;
        self.current_token = tokens[self.token_index as usize].clone();
    }
}

fn consume_token(token: String) -> Token {
    let token_as_str: &str = &(token.clone())[..];
    match token_as_str.chars().nth(0) {
        Some('"') => {
            // Remove quotes from string
            let mut next_token = token_as_str;
            if next_token.chars().nth(0) == Some('"') {
                next_token = &next_token[1..token_as_str.len()];
            }
            if next_token.chars().nth(next_token.len() - 1) == Some(',') {
                next_token = &next_token[0..token_as_str.len() - 3];
            } else if next_token.chars().nth(next_token.len() - 1) == Some('"') {
                next_token = &next_token[0..token_as_str.len() - 2];
            }
            return Token {
                token: next_token.to_string(),
                tokentype: TokenType::StringLiteral,
            }
        }
        Some('[') => {
            return Token {
                token: token,
                tokentype: TokenType::Label,
            }
        }
        Some('{') => {
            return Token {
                token: token,
                tokentype: TokenType::CurlyBracketStart,
            }
        }
        Some('}') => {
            return Token {
                token: token,
                tokentype: TokenType::CurlyBracketEnd,
            }
        }
        Some('=') => {
            return Token {
                token: token,
                tokentype: TokenType::Equals,
            }
        }
        Some(':') => {
            return Token {
                token: token,
                tokentype: TokenType::TypeAssignment,
            }
        }
        Some('#'..='9') => {
            let mut next_token = token_as_str;
            if token_as_str.chars().nth(token_as_str.len() - 1) == Some(',') {
                next_token = &token_as_str[0..token_as_str.len() - 1];
            }
            return Token {
                token: next_token.to_string(),
                tokentype: TokenType::NumberLiteral,
            }
        }
        _ => (),
    }

    match &token_as_str[0..4] {
        "true" => {
            return Token {
                token: "true".to_string(),
                tokentype: TokenType::BoolLiteral,
            }
        }
        // HACK: This is the worst thing i've ever done.
        "fals" => {
            return Token {
                token: "false".to_string(),
                tokentype: TokenType::BoolLiteral,
            }
        }
        _ => (),
    }

    match token_as_str {
        "type" => {
            return Token {
                token: token,
                tokentype: TokenType::TypeIdentifier,
            }
        }
        "number" => {
            return Token {
                token: token,
                tokentype: TokenType::TypeNumber,
            }
        }
        "bool" => {
            return Token {
                token: token,
                tokentype: TokenType::TypeBool,
            }
        }
        "string" => {
            return Token {
                token: token,
                tokentype: TokenType::TypeString,
            }
        }
        "map" => {
            return Token {
                token: token,
                tokentype: TokenType::TypeMap,
            }
        }
        "list" => {
            return Token {
                token: token,
                tokentype: TokenType::TypeList,
            }
        }
        _ => {
            // If a field is typed, we still capture the colon at the end.
            // This remove that colon at the end.
            let mut next_token = token_as_str;
            if token_as_str.chars().nth(token_as_str.len() - 1) == Some(':') {
                next_token = &token_as_str[0..token_as_str.len() - 1];
            }
            return Token {
                token: next_token.to_string(),
                tokentype: TokenType::PossibleIdentifier,
            }
        }
    }
}

fn construct_ast(tokens: &[Token]) -> ParsedFile {
    let mut token_traversal = TokenTraverse::new(tokens);
    let ast_result = token_traversal.block(tokens);
    println!();
    println!("User Types:");
    println!("-------------");

    for usertype in &ast_result.user_types {
        println!("{}", usertype.typename);
    }

    println!("\nUser Objects:");
    println!("-------------");
    for userobject in &ast_result.user_objects {
        print!("{}", userobject.object_name);

        // Print Object Type
        match &userobject.object_type {
            Some(object_type) => println!(": {} {{", object_type.typename),
            None => println!(" {{"),
        }

        // Print Fields
        for field in &userobject.fields {
            println!("\t{} = {}", field.identifier, field.value.token);
        }

        println!("}}");
    }
    println!();
    ast_result
}

// TODO: If type has a field that must be assigned by the children and that
// value isn't assigned by the children, panic.
/// Checks if each of the user's objects implements all the necessary values of its type
/// and expands each object's fields to have a field of its parent if that field isn't there.
fn fill_object_fields(file: &ParsedFile) -> Vec<Object> {
    let mut typed_objects: Vec<Object> = vec![];
    for object in &mut file.user_objects.clone() {
        if object.object_type.is_none() {
            continue;
        }

        let mut field_found = false;
        println!("{}", object.object_name);
        for type_field in object.object_type.clone().unwrap().fields {
            for object_field in &object.fields {
                if type_field.identifier == object_field.identifier {
                    println!("{}: {}", object_field.identifier, object_field.value.token);
                    field_found = true;
                    break;
                } 
            }
            if field_found == true {
                field_found = false;
            } else {
                object.fields.push(type_field.clone());
                println!("{}: {}", type_field.identifier, type_field.value.token);
            }
        }
        println!();
        typed_objects.push(object.clone());
    }
    typed_objects
}

fn emit_json(typed_objects: &Vec<Object>, user_fields: &Vec<Field>) -> String {
    let mut data = json::JsonValue::new_object();
    for field in user_fields {
        let value: String = field.value.token.clone();
        if value.parse::<i64>().is_ok() {
            data[field.identifier.clone()] = value.parse::<i64>().unwrap().into();
            continue;
        } else if value.parse::<bool>().is_ok() {
            data[field.identifier.clone()] = value.parse::<bool>().unwrap().into();
            continue;
        }
        data[field.identifier.clone()] = value.into();
    }
    for object in typed_objects {
        data[object.object_name.clone()] = json::JsonValue::new_object();
        for field in &object.fields {
            let value: String = field.value.token.clone();
            if value.parse::<i64>().is_ok() {
                data[object.object_name.clone()][field.identifier.clone()] = value.parse::<i64>().unwrap().into();
                continue;
            } else if value.parse::<bool>().is_ok() {
                data[object.object_name.clone()][field.identifier.clone()] = value.parse::<bool>().unwrap().into();
                continue;
            }
            data[object.object_name.clone()][field.identifier.clone()] = value.into();
        }
    }
    println!("{}", data);
    json::stringify(data)
}

fn main() {
    let command_args: Vec<String> = std::env::args().collect();

    if command_args.len() != 2 {
        panic!("No filename supplied.");
    }

    let filename = &command_args[1];
    let filepath = std::path::Path::new(&filename);

    let mut file = std::fs::File::open(filepath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let chars: Vec<_> = contents.chars().collect();

    let mut current_token = String::new();
    let mut tokens = vec![];
    let mut is_string_literal = false;
    let mut is_comment = false;
    for index in 0..chars.len() {
        // TODO: Multi-line comments would be cool
        // Single Line Comment Checking
        if (chars[index] == '/') && (chars[index + 1] == '/') {
            is_comment = true;
            continue;
        }
        if (chars[index] == '\n') && (is_comment == true) {
            is_comment = false;
            continue;
        }
        if is_comment == true {
            continue;
        }

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

    let mut symbols = vec![];
    for token in &tokens {
        symbols.push(consume_token(token.to_string()));
    }
    symbols.push(Token {
        token: "".to_string(),
        tokentype: TokenType::EOF,
    });

    // TODO: Make this an option type
    let ast = construct_ast(&symbols.as_slice());
    let typed_objects = fill_object_fields(&ast);

    let output_path = format!("{}.{}", filepath.file_stem().unwrap().to_str().unwrap().to_string(), "json");
    fs::write(output_path, emit_json(&typed_objects, &ast.user_fields)).expect("Unable to write to file");
}
