use std::env;
use std::fs;

extern crate regex;
use regex::Regex;

fn main() {
    // stage 3.1
    // read in file from command line
    let args: Vec<String> = env::args().collect();
    let file_name = args[1].clone();
    // create CStream object to place file into vec (separated by \n)
    let stream: CStream = CStream::new(&file_name);
    // tokenize vec
    for 
    let all_tokens: Vec<Token> = scanner(stream.f_vec);
    // print to screen
    println!("{:?}", stream.f_str);
    for t in all_tokens.iter() {
        println!("{}", t.token_type.as_str());
    }
    //println!("{:?}", all_tokens);


    // stage 3.2
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

impl TokenType{
    
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
    fn new(t: &str) -> Token{
    
    let mut typ = TokenType::INVALID;
    //[[:alpha:]]
    //[[:digit:]]
    //     x*        zero or more of x (greedy)
    // x+        one or more of x (greedy)
    // x?        zero or one of x (greedy)
    // xy    concatenation (x followed by y)
    // x|y   alternation (x or y, prefer x)
    
    // assigns to appropriate type upon creation

    match t {
        // IDENTIFIER: (_|[A-Za-z])([_|\d|[A-Za-z]])*
        // INT_CONSTANT: [-]?\d+
        // FLOAT_CONSTANT: [-]?\d+[.\d+]?
        "unsigned" | "char" | "short" | "int" | "long" | "float" | "double" | "while" | "if" | "return" | "void" | "main" => typ = TokenType::KEYWORD,
        "(" | "," | ")" | "{" | "}" | "=" | "==" | "<" | ">" | ">=" | "<=" | "!=" | "+" | "-" | "*" | "/" | ";" => typ = TokenType::OPERATOR,
        _ => typ = TokenType::INVALID
    }
    if typ.as_str() == "INVALID" {
        if Regex::new(r"\s").unwrap().is_match(t) {typ = TokenType::WHITESPACE;}
        else if Regex::new(r"[-]?\d+").unwrap().is_match(t) {typ = TokenType::INT_CONSTANT;}
        else if Regex::new(r"[-]?\d+[.\d+]?").unwrap().is_match(t){ typ = TokenType::FLOAT_CONSTANT;}
        else if Regex::new(r"(_|[A-Za-z])([_|\d|[A-Za-z]])*").unwrap().is_match(t) {typ = TokenType::IDENTIFIER;}
    }
    println!("the {} token was assigned type {}", t, typ.as_str());

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
    for line in f_vec.iter() {
        //TODO
        for c in line {
            // soumyas hw

            // ...
            //let t: Token = Token::new(________);
            //token_vec.push(__________);
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

