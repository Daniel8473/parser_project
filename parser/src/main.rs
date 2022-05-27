use std::env;
use std::fs;

extern crate regex;
use regex::Regex;

extern crate custom_error;
use custom_error::custom_error;

fn main() {
    // stage 3.1
    // read in file from command line
    let args: Vec<String> = env::args().collect();
    let file_name = args[1].clone();
    // create CStream object to place file into vec (separated by \n)
    let stream: CStream = CStream::new(&file_name);
    // tokenize vec
    let mut all_tokens: Vec<Token> = vec![];
    all_tokens = scanner(stream.f_vec);
    // stage 3.2
}

custom_error!{MyError
    InvalidString{pos: usize} = "Syntax error at character position {pos}"
    //Error at Line 2 Character 10. The syntax should be: DeclarationType := DataType Identifier.
}

// global current for storing current lexeme
struct Parser {

    //get_next() or error for terminal
    //non terminal, call fn
    // FIRST: 2 character look ahead example
    // if FIRST(X)
        // if FIRST (A)
        // else if FIRST (B)
    // else if FIRST(Y)
            // if FIRST (C)
        // else if FIRST (D)
}

impl Parser {

}

enum TokenType {
    INT_CONSTANT,
    FLOAT_CONSTANT,
    KEYWORD,
    OPERATOR,
    IDENTIFIER,
    INVALID,
    WHITESPACE
}

impl TokenType {
    
    fn as_str (&self) -> &'static str {
        // formats the type as string to help with printing
        match &self{
            TokenType::INT_CONSTANT => "INT_CONSTANT",
            TokenType::FLOAT_CONSTANT => "FLOAT_CONSTANT",
            TokenType::KEYWORD => "KEYWORD",
            TokenType::OPERATOR => "OPERATOR",
            TokenType::WHITESPACE => "WHITESPACE",
            TokenType::INVALID => "INVALID",
            TokenType::IDENTIFIER => "IDENTIFIER"
        }

    }
}

struct Token {
    text: String,
    token_type: TokenType,
    line_num: i32,
    char_pos: i32
}

impl Token {
    fn new(t: &str, l_pos: i32, c_pos: i32) -> Token {
    let mut typ = TokenType::INVALID;
    match t {
        // IDENTIFIER: (_|[A-Za-z])([_|\d|[A-Za-z]])*
        // INT_CONSTANT: [-]?\d+
        // FLOAT_CONSTANT: [-]?\d+\.\d+
        "unsigned" | "char" | "short" | "int" | "long" | "float" | "double" | "while" | "if" | "return" | "void" | "main" => typ = TokenType::KEYWORD,
        "(" | "," | ")" | "{" | "}" | "=" | "==" | "<" | ">" | ">=" | "<=" | "!=" | "+" | "-" | "*" | "/" | ";" => typ = TokenType::OPERATOR,
        _ => typ = TokenType::INVALID
    }
    if typ.as_str() == "INVALID" {
        if Regex::new(r"\s").unwrap().is_match(t) {typ = TokenType::WHITESPACE;}
        else if Regex::new(r"[-]?\d+\.\d+").unwrap().is_match(t){ typ = TokenType::FLOAT_CONSTANT;}
        else if Regex::new(r"[-]?\d+").unwrap().is_match(t) {typ = TokenType::INT_CONSTANT;}
        else if Regex::new(r"(_|[A-Za-z])([_|\d|[A-Za-z]])*").unwrap().is_match(t) {typ = TokenType::IDENTIFIER;}
    }
    println!("{} : {}", t, typ.as_str());

        Token {
            text: t.to_string(),
            token_type: typ,
            line_num: 0, // TODO MAYBE CHANGE TO -1
            char_pos: 0 // ^
        }

    }
}




fn scanner(f_vec: Vec<String>) -> Vec<Token> {
    // loop through file and tokenize
    let mut token_vec: Vec<Token> = vec![];
    for (line_pos, line) in f_vec.iter().enumerate() {
        let mut skip: bool = false;
        let mut text_to_token = ("").to_string();
        // parse char by char until token (type String) is built
        for (char_pos, c) in line.chars().enumerate() {
            // ensures double operators are added 2 at a time
            if skip {
                skip = false;
                continue;
            }
            // if next char is (whitespace) or (operator) or we're ar EOL, add create Token from text_to_token
            if char_pos == line.len() - 1 || Regex::new(r"\s|\(|\)|,|\{|\}|=|>|<|!|\+|-|\*|/|;").unwrap().is_match(&(line.chars().nth(char_pos + 1).unwrap().to_string())) {
                text_to_token += &c.to_string();
                let token: Token = Token::new(&text_to_token, line_pos as i32, char_pos as i32);
                token_vec.push(token);
                text_to_token = ("").to_string();
            }
            // add any white space as token
            else if c.is_whitespace() { // if (c == ' ') | (c == '\n') | (c == '\t') | (c == '\r') {
                text_to_token += &c.to_string();
                let token: Token = Token::new(&text_to_token, line_pos as i32, char_pos as i32);
                token_vec.push(token);
                text_to_token = ("").to_string();
            }
            // if letter/number, continue adding to token
            else if Regex::new(r"[A-Za-z]|\d").unwrap().is_match(&(c.to_string())) {
                text_to_token += &c.to_string();
            }
            // check for if the operator consists of 2 characters
            else if char_pos < line.len() - 1 && ((c == '=') | (c == '<') | (c == '>') | (c == '!')) {
                if line.chars().nth(char_pos + 1).unwrap() == '=' {
                    text_to_token += &c.to_string();
                    skip = true;
                    text_to_token += &(line.chars().nth(char_pos + 1).unwrap().to_string());
                }
            }
            else if Regex::new(r"\s|\(|\)|,|\{|\}|\+|-|\*|/|;|=").unwrap().is_match(&(c.to_string())) {
                text_to_token += &c.to_string();
                let token: Token = Token::new(&text_to_token, line_pos as i32, char_pos as i32);
                token_vec.push(token);
                text_to_token = ("").to_string();
            }
            // anything else
            else {
                text_to_token += &c.to_string();
            }
        }
    }
    return token_vec;
}

struct CStream {
    f_vec: Vec<String>,
    f_str: String
}

impl CStream {
    fn new(file_name: &str) -> CStream {
        // read entire file into string
        let f_s = fs::read_to_string(file_name).unwrap();
        // splits string at newlines and places substring into vector
        let mut f_v: Vec<String> = f_s.lines().map(String::from).collect();
        
        CStream{
            f_vec: f_v,
            f_str: f_s
        }
    }
}

