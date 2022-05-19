use std::env;
use std::fs;

fn main() {
    // stage 3.1
    // read in file from command line
    let args: Vec<String> = env::args().collect();
    let file_name = args[1].clone();
    // create CStream object to place file into vec
    let f: CStream = CStream::new(&file_name);
    // print to screen
    f.print_vec();

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

// impl TokenType{
//     fn as_str (&self) -> &'static str {
//         // formats the type as string to help with printing
//         match &self{
//             TokenType::CONSTANT => "constant",
//             TokenType::OPERATOR => "operator",
//             TokenType::VARIABLE => "variable",
//             TokenType::SPECIAL => "special symbol"
//         }

//     }
// }

struct Token {
    text: String,
    token_type: TokenType
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
        // IDENTIFIER: ( _ | \d ) ( _ | \d | Alpha )*
        //(_|[A-Za-z])([_|\d|[A-Za-z]])*
        //INT_CONSTANT [-]?\d+
        //FIXME [-]?\d+[.[\d]+]?
        "unsigned" | "char" | "short" | "int" | "long" | "float" | "double" | "while" | "if" | "return" | "void" | "main" => typ = TokenType::KEYWORD,
        "(" | "," | ")" | "{" | "}" | "=" | "==" | "<" | ">" | ">=" | ">=" | "!=" | "+" | "-" | "*" | "/" | ";" => typ = TokenType::OPERATOR,
        t.chars().nth(0).unwrap().is_whitespace() => typ = TokenType::WHITESPACE,
        => typ = TokenType::INT_CONSTANT,
        => typ = TokenType::FLOAT_CONSTANT,
        => typ = TokenType::IDENTIFIER,
        _ => typ = TokenType::INVALID
    }

        Token {
            text: t.to_string(),
            token_type: typ,
            line_num: 0, // TODO MAYBE CHANGE TO -1
            char_pos: 0 // ^
        }

    }
}

struct Scanner {
    // parse file, calls token for each token it identifies
}

impl Scanner {
    fn new() -> Scanner {
        Scanner{}
        parse_input(){

            //look at string
            //make token "0", ":="
        }
    }
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

    fn print_vec(&self) {
        println!{"{:?}", self.f_str};
    }
}