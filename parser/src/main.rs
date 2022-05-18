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