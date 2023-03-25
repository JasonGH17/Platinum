#![allow(dead_code)]

use std::{fs, env};

mod lexer;

struct Compiler {
    file: String,
    lexer: lexer::Lexer,
}

impl Compiler {
    pub fn new(filename: &str) -> Self {
        let file = Compiler::read_file(filename);
        Self { file: file.clone(), lexer: lexer::Lexer::new(file) }
    }

    fn read_file(filename: &str) -> String {
        fs::read_to_string(filename).expect(&format!("File {} was not found!", filename))
    }

    fn write_file(filename: &str, data: Vec<u8>) {
        fs::write(filename, data).expect(&format!("Unable to write the compiled program {}", filename));
    }

    pub fn compile(&mut self) -> Vec<u8> {        
        let mut sec_type: Vec<u8> = vec![0x01, 0x60, 0x00, 0x01, 0x7F];
        let mut sec_func: Vec<u8> = vec![];
        let mut sec_expo: Vec<u8> = vec![];
        let mut sec_code: Vec<u8> = vec![];

        // Add Section Sizes
        sec_type = [&[0x01, sec_type.len().try_into().unwrap()], &sec_type[..]].concat();
        sec_func = [&[0x01, sec_func.len().try_into().unwrap()], &sec_func[..]].concat();
        sec_expo = [&[0x01, sec_expo.len().try_into().unwrap()], &sec_expo[..]].concat();
        sec_code = [&[0x01, sec_code.len().try_into().unwrap()], &sec_code[..]].concat();

        let program = [
            &vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00], // WASM Magic + Version
            &sec_type[..], 
            &sec_func[..], 
            &sec_expo[..], 
            &sec_code[..]
        ].concat();

        return program;
    }
}

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let mut compiler = Compiler::new(env::current_dir().unwrap().join(file_name.clone()).to_str().unwrap());

    let mut token = compiler.lexer.next();
    while token.kind != lexer::PlatTokenKinds::EOF {
        println!("{:?}", token);
        token = compiler.lexer.next();
    }
    println!("{:?}", token);

    Compiler::write_file(&file_name.replace(".plat", ".wasm"), compiler.compile())
}
