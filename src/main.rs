#![allow(dead_code)]

use std::{fs, env};

struct Compiler {
    file: String,
}

impl Compiler {
    pub fn new(filename: &str) -> Self {
        let file = Compiler::read_file(filename);
        Self { file: file.clone() }
    }

    fn read_file(filename: &str) -> String {
        fs::read_to_string(filename).expect(&format!("File {} was not found!", filename))
    }

    fn write_file(filename: &str, data: Vec<u8>) {
        fs::write(filename, data).expect(&format!("Unable to write the compiled program {}", filename));
    }

    pub fn compile(&mut self) -> Vec<u8> {        
        let program: Vec<u8> = vec![0x00];

        return program;
    }
}

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let mut compiler = Compiler::new(env::current_dir().unwrap().join(file_name.clone()).to_str().unwrap());

    Compiler::write_file(&file_name.replace(".plat", ".wasm"), compiler.compile())
}
