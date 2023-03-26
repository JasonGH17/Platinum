#![allow(dead_code)]

use std::env;

mod lexer;
mod parser;
mod compiler;

fn main() {
    let file_name = env::args().nth(1).unwrap();
    let mut compiler = compiler::Compiler::new(env::current_dir().unwrap().join(file_name.clone()).to_str().unwrap());

    let ast = compiler.parse();
    for node in ast.iter() {
        print!("{node}");
    }

    compiler::Compiler::write_file(&file_name.replace(".plat", ".wasm"), compiler.compile(ast))
}
