use std::env;
use std::fs;
use std::collections::HashSet;
use std::hash::Hash;

extern crate regex;
use regex::Regex;

extern crate custom_error;
use custom_error::custom_error;

fn main() {
    // stage 3.1
    // read in file from command line
    // let args: Vec<String> = env::args().collect();
    // let file_name = args[1].clone();
    // create CStream object to place file into vec (separated by \n)
    // let stream: CStream = CStream::new(&file_name);

    // vector for debugging
    // let debug_vec: Vec<String> = vec!["*=!= <= ===i -5 -365.2 6".to_string()];
    // let debug_vec: Vec<String> = vec!["2-4 == -2.0".to_string()];
    // let debug_vec: Vec<String> = vec!["*=\t!= <= ===i -5 -365.2 2-4 == -2.0 1 -2222.2234  5 - 1 = 4 ".to_string()];
    // let debug_vec: Vec<String> = vec![    "5 - 1 = 4 ".to_string()];
    let debug_vec: Vec<String> = vec!["1 -2222.2234".to_string()];


    // stage 3.2
    // create vector of tokens
    let mut all_tokens: Vec<Token> = vec![];
    // all_tokens = scanner(stream.f_vec);
    all_tokens = scanner(debug_vec);


    // stage 3.2
    // parse tokens and determine if syntax is correct
    let mut result: Parser = Parser::new(all_tokens);
    result.program_();
}

custom_error!{MyError
    InvalidProgram{err_line_pos: i32, err_char_pos: i32} = "Syntax error at line {err_line_pos}, character position {err_char_pos}"
    // Error at Line 2 Character 10. The syntax should be: DeclarationType := DataType Identifier.
}

// global current for storing current lexeme
struct Parser {
    // TODO error case: EBNF tree has been fully traversed, but all_tokens vector hasn't: 
    // DONE error case: all_tokens vector has been fully traversed, but EBNF tree hasn't: handled in get_next()
    // TODO call error printing correctly
    curr_pos: usize,
    all_tokens: Vec<Token>,
    t_text: String,
    t_type: String,

    fir_declaration: HashSet< &'static str>,
    fir_main_declaration: HashSet< &'static str>,
    fir_function_definition: HashSet< &'static str>,
    fir_declaration_type: HashSet< &'static str>,
    fir_variable_declaration: HashSet< &'static str>,
    fir_function_declaration: HashSet< &'static str>,
    fir_block: HashSet< &'static str>,
    fir_parameter_block: HashSet< &'static str>,
    fir_data_type: HashSet< &'static str>,
    // fir_constant: HashSet< &'static str>,
    fir_statement: HashSet< &'static str>,
    fir_parameter: HashSet< &'static str>,
    fir_integer_type: HashSet< &'static str>,
    fir_float_type: HashSet< &'static str>,
    // fir_assignment: HashSet< &'static str>,
    fir_while_loop: HashSet< &'static str>,
    fir_if_statement: HashSet< &'static str>,
    fir_return_statement: HashSet< &'static str>,
    // fir_expression: HashSet< &'static str>,
    // fir_simple_expression: HashSet< &'static str>,
    // fir_term: HashSet< &'static str>,
    fir_factor: HashSet< &'static str>,
    fir_relation_operator: HashSet< &'static str>,
    fir_add_operator: HashSet< &'static str>,
    fir_mult_operator: HashSet< &'static str>

}

impl Parser {
    fn new(all_tokens_input: Vec<Token>) -> Parser {
        // copy/paste to create first sets  ->  fir_: HashSet::from([]),
        // TODO: FIRST sets arent all disjoint, is this ok?
        // TOOD: DEAL WITH GET PREVIOUS
        // TODO: SKIP WHITEPSACE IN GET NEXT
        // TODO: UN COMMENT OUT FILE READING
        Parser {
            all_tokens: all_tokens_input,
            curr_pos: 0,

            t_text: "".to_string(),
            t_type: "".to_string(),

            // fir_identifier: HashSet::from(["_", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]), // "_", alpha
            // fir_int_constant: HashSet::from(["-", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]), // "-", digit]),
            // fir_float_constant: HashSet::from(["-", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]), // "-", digit
            // fir_digit: HashSet::from(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]),
            // fir_alpha: HashSet::from(["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]),
            fir_declaration: HashSet::from(["unsigned", "char", "short", "int", "long", "float", "double"]), // declaration_type .
            fir_main_declaration: HashSet::from(["void"]), 
            fir_function_definition: HashSet::from(["unsigned", "char", "short", "int", "long", "float", "double"]), // declaration_type
            fir_declaration_type: HashSet::from(["unsigned", "char", "short", "int", "long", "float", "double"]), // data_type .
            fir_variable_declaration: HashSet::from(["="]), 
            fir_function_declaration: HashSet::from(["("]), // parameter_block
            fir_block: HashSet::from(["{"]), 
            fir_parameter_block: HashSet::from(["("]), 
            fir_data_type: HashSet::from(["unsigned", "char", "short", "int", "long", "float", "double"]), // integer_type, floaself.t_type .
            // fir_constant: int_constant, float_constant
            fir_statement: HashSet::from(["while", "if", "return"]), // assignment, while_loop, if_statement, return_statement, expression
                // additional regex: identifier, int_constant, float_constant, identifier
            fir_parameter: HashSet::from(["unsigned", "char", "short", "int", "long", "float", "double"]), // data_type .
            fir_integer_type: HashSet::from(["unsigned", "char", "short", "int", "long"]), 
            fir_float_type: HashSet::from(["float", "double"]), 
            // fir_assignment: identifier
            fir_while_loop: HashSet::from(["while"]), 
            fir_if_statement: HashSet::from(["if"]), 
            fir_return_statement: HashSet::from(["return"]), 
            // fir_expression: int_constant, float_constant, identifier 
            // fir_simple_expression: int_constant, float_constant, identifier
            // fir_term: int_constant, float_constant, identifier
            fir_factor: HashSet::from(["("]), // "(", constant, identifier 
                // additional regex: int_constant, float_constant, identifier
            
            fir_relation_operator: HashSet::from(["==", "<", ">", "<=", ">=", "!="]), 
            fir_add_operator: HashSet::from(["+", "-"]),
            fir_mult_operator: HashSet::from(["*", "/"])
        }
    }

    // fn okay(&mut self, res: Result<(), MyError>) -> bool {
    //     match res {
    //         Ok(()) => return true,
    //         _ => return false
    //     }
    // }
    fn get_next(&mut self) -> Result<(), MyError> {
        if self.curr_pos < self.all_tokens.len() - 1 {
            self.curr_pos += 1;
            self.t_text = self.all_tokens[self.curr_pos].text.clone();
            self.t_type = self.all_tokens[self.curr_pos].token_type.as_str().clone().to_string();
            return Ok(());
        } else {
            // all_tokens vector has been fully traversed, but EBNF tree hasn't
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
    }

    fn get_prev(&mut self) {
        self.curr_pos -= 1;
        self.t_text = self.all_tokens[self.curr_pos].text.clone();
        self.t_type = self.all_tokens[self.curr_pos].token_type.as_str().clone().to_string();

    }
    // DOTHISTOGETHERTEAMWORK {declaration} main_declaration {function_definition}
    fn program_(&mut self) {
        // init 
        self.t_text = self.all_tokens[self.curr_pos].text.clone();
        self.t_type = self.all_tokens[self.curr_pos].token_type.as_str().clone().to_string();

        
        let result = self.mult_operator_();
        match result {
            Ok(()) => println!("Program is valid"),
            Err(err_line_pos) => eprintln!("{}", err_line_pos) // TODO FIXME (needs err_char_pos)
            // Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos})
        }
    }
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////
    // declarartion_type GROUP(variable_declaration | function_declaration)
    fn declaration_(&mut self) -> Result<(), MyError> {
        self.declaration_type_();
        if self.fir_variable_declaration.contains(self.t_text.as_str()) {
            self.variable_declaration_();
        }
        else if self.fir_function_declaration.contains(self.t_text.as_str()) {
            self.function_declaration_();
        }
        else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        return Ok(());
    }
    // VOID MAIN ( ) block
    fn main_declaration_(&mut self) -> Result<(), MyError> {
        if self.t_text == "void".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }   
        if self.t_text == "main".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        } 
        if self.t_text == "(".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        } 
        if self.t_text == ")".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        } 
        self.block_();
        return Ok(());
    }
    // declaration_type parameter_block block 
    fn function_definition(&mut self) -> Result<(), MyError> {
        self.declaration_type_();
        self.parameter_block();
        self.block_();
        return Ok(());
    }
    // data_type identifier
    fn declaration_type_(&mut self) -> Result<(), MyError> {
        self.data_type_();
        if self.t_type == "IDENTIFIER".to_string() {
            self.get_next();
        }
        else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        return Ok(());
    }
    // [= constant] ;
    fn variable_declaration_(&mut self) -> Result<(), MyError> {
        if self.t_text == "=".to_string() {
            self.get_next();
            self.constant_();
        }
        if self.t_text == ";".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        } 
        return Ok(());
    }
    // parameter_block ;
    fn function_declaration_(&mut self) -> Result<(), MyError> {
        self.parameter_block();
        if self.t_text == ";".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        } 
        return Ok(());
    }
    // { {decleration} {statement} {function_definition} } - OUTSIDE {} ARE TERMINAL
    // sec(f_def) = fir(param_block) = "("
    // sec(dec) = fir(var_dec) OR fir(func_dec) "=" or ";" or  "("
        // third(f_def) = fir(block) = "{"
        // third(dec) = fir(const) OR ; = 
    fn block_(&mut self) -> Result<(), MyError> {
        if self.t_text == "{".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        while self.fir_declaration.contains(self.t_text.as_str()) {
            self.declaration_();
        }
        while self.fir_statement.contains(self.t_text.as_str()) {
            self.statement_();
        }
        while self.fir_function_definition.contains(self.t_text.as_str()) {
            self.function_definition();
        }
        if self.t_text == "}".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        return Ok(());
    }
    // ( [parameter {, parameter}] )
    fn parameter_block(&mut self) -> Result<(), MyError> {
        if self.t_text == "(".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        if self.fir_parameter.contains(self.t_text.as_str()) {
            self.parameter_();
            while self.t_text == "," {
                self.get_next();
                self.parameter_();
            }    
        }
        if self.t_text == ")".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }   
        return Ok(());
    }
    
    // integer_type | float_type
    fn data_type_(&mut self) -> Result<(), MyError> {
        if self.fir_float_type.contains(self.t_text.as_str()) {
            self.float_type_();
        }
        else if self.fir_integer_type.contains(self.t_text.as_str()) {
            self.integer_type_();
        }
        else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        return Ok(());
    }
    // int_constant | float_constant
    fn constant_(&mut self) -> Result<(), MyError> {
        if self.t_type == "FLOAT_CONSTANT".to_string() 
        || self.t_type == "INT_CONSTANT".to_string() {
            self.get_next();        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        } 
        return Ok(());
    }

    // assignment | while_loop | if_statement | return_statement | expression ;
    fn statement_(&mut self) -> Result<(), MyError> {
        if self.fir_while_loop.contains(self.t_text.as_str()) {
            self.while_loop_();
        }
        else if self.fir_if_statement.contains(self.t_text.as_str()) {
            self.if_statement_();
        }
        else if self.fir_return_statement.contains(self.t_text.as_str()) {
            self.return_statement_();
        }
        // assignment
        else if self.t_type == "IDENTIFIER".to_string() && self.all_tokens[self.curr_pos + 1].text.clone() == "=".to_string() {
            self.assignment_();
        }
        // expression: int_constant, float_constant, identifier
        else if self.t_type == "IDENTIFIER".to_string() || self.t_type == "FLOAT_CONSTANT".to_string() 
        || self.t_type == "INT_CONSTANT".to_string() {
            self.expression_();
            if self.t_text == ";".to_string() {
                self.get_next();
            }
            else{
                let err_token = &self.all_tokens[self.curr_pos];
                return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
            }
        }
        else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        return Ok(());
    }
    // data_type identifier
    fn parameter_(&mut self) -> Result<(), MyError> {
        self.data_type_();
        if self.t_type == "IDENTIFIER".to_string() {
            self.get_next();
        }
        else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        } 
        return Ok(());
    }
    // [ UNSIGNED ] group( CHAR | SHORT | INT | LONG)
    fn integer_type_(&mut self) -> Result<(), MyError> {
        if self.t_text == "unsigned".to_string() {
            self.get_next();
        }
        if self.t_text == "char".to_string() {
            self.get_next();
        }
        else if self.t_text == "short".to_string() {
            self.get_next();
        }
        else if self.t_text == "int".to_string() {
            self.get_next();
        }
        else if self.t_text == "long".to_string() {
            self.get_next();
        }
        else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        return Ok(());
    }
    // FLOAT | DOUBLE
    fn float_type_(&mut self) -> Result<(), MyError> {
        if self.t_text == "float".to_string() {
            self.get_next();
        }
        else if self.t_text == "double".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        return Ok(());
    }
    // identifier = {identifier =} expression ;
    fn assignment_(&mut self) -> Result<(), MyError> {
        if self.t_type == "IDENTIFIER".to_string() {
            self.get_next();
        }
        else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }        
        if self.t_text == "=".to_string() {
            self.get_next();
        }
        else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        while self.t_type == "IDENTIFIER".to_string(){
            self.get_next();
            if self.t_text == "=".to_string() {
                self.get_next();
            }
            else {
                let err_token = &self.all_tokens[self.curr_pos];
                return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
            }  
        }
        self.expression_(); 
        if self.t_text == ";".to_string() {
            self.get_next();
        }
        else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }    
        return Ok(());
        }
    // WHILE ( expression ) block
    fn while_loop_(&mut self) -> Result<(), MyError> {
        if self.t_text == "while".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        if self.t_text == "(".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }        
        self.expression_();
        if self.t_text == ")".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        self.block_();
        return Ok(());
    }
    // IF ( expression ) block
    fn if_statement_(&mut self) -> Result<(), MyError> {
        if self.t_text == "if".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        if self.t_text == "(".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }        
        self.expression_();
        if self.t_text == ")".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        self.block_();
        return Ok(());
    }
    // RETURN expression ;
    fn return_statement_(&mut self) -> Result<(), MyError> {
        if self.t_text == "return".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        self.expression_();
        if self.t_text == ";".to_string() {
            self.get_next();
        }
        else{
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        return Ok(());
    }

    // simple_expression [ relation_operator simple_expression ]
    fn expression_(&mut self) -> Result<(), MyError> {
        self.simple_expression_();
        if self.fir_relation_operator.contains(self.t_text.as_str()) {
                self.relation_operator_();
                self.simple_expression_();
        }
        return Ok(());
    }
    // term { add_operator term }
    fn simple_expression_(&mut self) -> Result<(), MyError> {
        self.term_();
        while self.fir_add_operator.contains(self.t_text.as_str()) {
                self.add_operator_();
                self.term_();
        }
        return Ok(());
    }
    // for options on which func to call |:
    //      call FIRST
    //      return error at last else
    // for funcs that MUST be called
    //      call func
    // GET NEXT before every okay

    // factor { mult_operator factor }
    fn term_(&mut self) -> Result<(), MyError> {
        self.factor_();
        while self.fir_mult_operator.contains(self.t_text.as_str()) {
                self.mult_operator_();
                self.factor_();
        }
        return Ok(());
    }
    // ( expression )     |   constant    |   identifier [ ( [ expression {, expression } ] ) ]
    fn factor_(&mut self) -> Result<(), MyError> {
        // ( expression )
        if self.t_text == "(".to_string() {
            self.get_next();
            self.expression_();
            if self.t_text == ")".to_string() {
                self.get_next();
            } else {
                let err_token = &self.all_tokens[self.curr_pos];
                return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
            }
        }

        // constant
        else if self.t_type == "INT_CONSTANT".to_string() {
            self.get_next();
        }

        // identifier [ ( [ expression {, expression } ] ) ]
        else if self.t_type == "IDENTIFIER".to_string() {
            self.get_next();
            // 1 case []
            if self.t_text == "(".to_string() {
                self.get_next();
                // 1 case []
                self.expression_();
                // 1+ case {}
                while self.t_text == "," {
                    self.get_next();
                    self.expression_();
                }    
                // check matching )
                if self.t_text == ")".to_string() {
                    self.get_next();
                } 
                else {
                let err_token = &self.all_tokens[self.curr_pos];
                return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
                }
            }
            else {
                self.get_next();
            }
        }
        else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
        return Ok(());
    }
    // ==   |   <   |   >   |   <=  |   >=  |   !=
    fn relation_operator_(&mut self) -> Result<(), MyError> {
        if self.t_text == "==".to_string() || self.t_text == "<".to_string()
        || self.t_text == ">".to_string() || self.t_text == "<=".to_string() 
        || self.t_text == ">=".to_string() || self.t_text == "!=".to_string() {
            self.get_next();
            return Ok(());
        } else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
    }
    // +    |   -
    fn add_operator_(&mut self) -> Result<(), MyError> {
        if self.t_text == "+".to_string() || self.t_text == "-".to_string() {
            self.get_next();
            return Ok(());
        } else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
    }
    // *    |   /
    fn mult_operator_(&mut self) -> Result<(), MyError> {
        if self.t_text == "*".to_string() || self.t_text == "/".to_string() {
            self.get_next();
            return Ok(());
        } else {
            let err_token = &self.all_tokens[self.curr_pos];
            return Err(MyError::InvalidProgram{err_line_pos: err_token.line_num, err_char_pos: err_token.char_pos});
        }
    }
}
    // terminal: get_next() or error
    // non terminal: call fn

    // FIRST: 2 character look ahead example
    // if FIRST(X)
        // if FIRST (A)
        // else if FIRST (B)
    // else if FIRST(Y)
            // if FIRST (C)
        // else if FIRST (D)

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
        match &self {
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
    // TODO: -1 parsed incorrectly
    // TODO: tabs are currently read as 4 whitepsaces (should be read as 1)
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
        "(" | "," | ")" | "{" | "}" | "=" | "==" | "<" | ">" | ">=" | "<=" | "!=" | "+" | /*"-"|*/ "*" | "/" | ";" => typ = TokenType::OPERATOR,
        _ => typ = TokenType::INVALID
    }
    if typ.as_str() == "INVALID" {
        if Regex::new(r"\s").unwrap().is_match(t) {typ = TokenType::WHITESPACE;}
        else if Regex::new(r"[-]?\d+\.\d+").unwrap().is_match(t){ typ = TokenType::FLOAT_CONSTANT;}
        else if Regex::new(r"[-]?\d+").unwrap().is_match(t) {typ = TokenType::INT_CONSTANT;}
        else if Regex::new(r"(_|[A-Za-z])([_|\d|[A-Za-z]])*").unwrap().is_match(t) {typ = TokenType::IDENTIFIER;}
        else if t == "-" {typ = TokenType::OPERATOR;}
    }
    println!("{}\t{}", t, typ.as_str());

        Token {
            text: t.to_string(),
            token_type: typ,
            line_num: l_pos,
            char_pos: c_pos
        }

    }
}




fn scanner(f_vec: Vec<String>) -> Vec<Token> {
    // loop through file and tokenize
    let mut token_vec: Vec<Token> = vec![];
    for (line_pos, line) in f_vec.iter().enumerate() {
        let mut operator_skip: bool = false;
        let mut last_char_was_digit: bool = false;
        let mut text_to_token = ("").to_string();
        // parse char by char until token (type String) is built
        for (char_pos, c) in line.chars().enumerate() {
            // ensures double operators are added 2 at a time
            if operator_skip {
                operator_skip = false;
                continue;
            }
            // operator with 2 chars
            if char_pos < line.len() - 1 && ((c == '=') | (c == '<') | (c == '>') | (c == '!')) && line.chars().nth(char_pos + 1).unwrap() == '=' {
                text_to_token += &c.to_string();
                text_to_token += &(line.chars().nth(char_pos + 1).unwrap().to_string());
                let token: Token = Token::new(&text_to_token, line_pos as i32, char_pos as i32);
                token_vec.push(token);
                text_to_token = ("").to_string();
                operator_skip = true;
            }
            // TODO: negative nums 
            else if c == '-' {
                // treat as operator
                if last_char_was_digit {
                    text_to_token += &c.to_string();
                    let token: Token = Token::new(&text_to_token, line_pos as i32, char_pos as i32);
                    token_vec.push(token);
                    text_to_token = ("").to_string();
                } 
                // treat as negative sign in front of a number
                else { 
                    text_to_token += &c.to_string();
                }

            }
            // if next char is (whitespace) or (operator) or (we're ar EOL), add create Token from text_to_token
            else if char_pos == line.len() - 1 || Regex::new(r"\s|\(|\)|,|\{|\}|=|>|<|!|\+|-|\*|/|;").unwrap().is_match(&(line.chars().nth(char_pos + 1).unwrap().to_string())) {
                text_to_token += &c.to_string();
                let token: Token = Token::new(&text_to_token, line_pos as i32, char_pos as i32);
                token_vec.push(token);
                text_to_token = ("").to_string();

                // set last_char_was_digit, to know how to interpret the next negative sign
                if Regex::new(r"\d").unwrap().is_match(&(c.to_string())) {last_char_was_digit = true;}
                else if !c.is_whitespace() {last_char_was_digit = false;}
                else {continue;}
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
            // operator with 1 char
            else if Regex::new(r"\s|\(|\)|,|\{|\}|\+|-|\*|/|;|=").unwrap().is_match(&(c.to_string())) {
                text_to_token += &c.to_string();
                let token: Token = Token::new(&text_to_token, line_pos as i32, char_pos as i32);
                token_vec.push(token);
                text_to_token = ("").to_string();
            }
            // anything else
            else {
                text_to_token += &c.to_string();

                // set last_char_was_digit, to know how to interpret the next negative sign
                if Regex::new(r"\d").unwrap().is_match(&(c.to_string())) {last_char_was_digit = true;}
                else if !c.is_whitespace() {last_char_was_digit = false;}
                else {continue;}
            }
        }
    }
    return token_vec;
}

struct CStream {
    f_vec: Vec<String>,
}

impl CStream {
    fn new(file_name: &str) -> CStream {
        // read entire file into string
        let f_s = fs::read_to_string(file_name).unwrap();
        // splits string at newlines and places substring into vector
        let f_v: Vec<String> = f_s.lines().map(String::from).collect();
        
        CStream {f_vec: f_v}
    }
}
