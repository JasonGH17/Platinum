use std::fs;

use crate::lexer;
use crate::parser;

pub struct Compiler {
    file: String,
    parser: parser::Parser,
    program: Program
}

struct Program {
    pub sec_type: Vec<u8>,
    pub n_types: u8,
    pub sec_func: Vec<u8>,
    pub n_funcs: u8,
    pub sec_expo: Vec<u8>,
    pub n_expos: u8,
    pub sec_code: Vec<u8>
}

impl Program {
    pub fn new() -> Self {
        Self{sec_type: vec![], n_types: 0, sec_func: vec![], n_funcs: 0, sec_expo: vec![], n_expos: 0, sec_code: vec![] }
    }

    pub fn unite(&mut self) -> Vec<u8> {
        self.sec_type = [&[0x01, <usize as TryInto<u8>>::try_into(self.sec_type.len()).unwrap() + self.n_types, self.n_types], &self.sec_type[..]].concat();
        self.sec_func = [&[0x03, <usize as TryInto<u8>>::try_into(self.sec_func.len()).unwrap() + self.n_funcs, self.n_funcs], &self.sec_func[..]].concat();
        self.sec_expo = [&[0x07, <usize as TryInto<u8>>::try_into(self.sec_expo.len()).unwrap() + self.n_expos, self.n_expos], &self.sec_expo[..]].concat();
        self.sec_code = [&[self.n_funcs], &self.sec_code[..]].concat();
        self.sec_code = [&[0x0A, <usize as TryInto<u8>>::try_into(self.sec_code.len()).unwrap()], &self.sec_code[..]].concat();

        [
            &vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00], // WASM Magic + Version
            &self.sec_type[..],
            &self.sec_func[..],
            &self.sec_expo[..],
            &self.sec_code[..]
        ].concat()
    }
}

fn plat_type_to_byte(kind: parser::PlatTypes) -> u8 {
    match kind {
        parser::PlatTypes::Void => 0x00,
        parser::PlatTypes::I32 => 0x7F,
        parser::PlatTypes::I64 => 0x7E,
        parser::PlatTypes::F32 => 0x7D,
        parser::PlatTypes::F64 => 0x7C,
    }
}

impl Compiler {
    pub fn new(filename: &str) -> Self {
        let file = Compiler::read_file(filename);

        let lexer = lexer::Lexer::new(file.clone());

        Self { file, parser: parser::Parser::new(lexer), program: Program::new() }
    }

    pub fn read_file(filename: &str) -> String {
        fs::read_to_string(filename).expect(&format!("File {} was not found!", filename))
    }

    pub fn write_file(filename: &str, data: Vec<u8>) {
        fs::write(filename, data).expect(&format!("Unable to write the compiled program {}", filename));
    }

    pub fn parse(&mut self) -> Vec<parser::AST> {
        self.parser.parse(None)
    }

    fn compile_fn(&mut self, expr: &parser::FuncExpr) {
        self.program.n_types += 1;
        self.program.sec_type.append(&mut vec![
            0x60,                                                                            // Function
            <usize as TryInto<u8>>::try_into(expr.args.len()).unwrap(),                      // Number of arguments
        ]);
        if expr.ret_kind == parser::PlatTypes::Void {
            self.program.sec_type.push(0x00);                                                // No returns
        } else {
            self.program.sec_type.append(&mut vec![0x01, plat_type_to_byte(expr.ret_kind)]); // Return type
        }

        self.program.sec_func.push(self.program.n_funcs);
        self.program.n_funcs += 1;

        self.program.sec_code.push(0x00); // Function size placeholder
        let func_size_index = self.program.sec_code.len() - 1;
        let code_size = self.program.sec_code.len();
        self.program.sec_code.push(0x00); // TODO: Add support for local declarations
        for node in expr.body.as_ref().as_ref().unwrap().iter() {
            self.compile_node(node);
        }
        self.program.sec_code.push(0x0B); // End of function
        self.program.sec_code[func_size_index] = <usize as TryInto<u8>>::try_into(self.program.sec_code.len() - code_size).unwrap();
    }

    fn compile_def(&mut self, expr: &parser::DefExpr) {
        let mut body: Vec<u8> = vec![];

        match expr.kind {
            parser::PlatTypes::I32 => {
                body.push(0x41);
                match expr.value.as_ref().unwrap() {
                    parser::Expr::Unary(_) => todo!(),
                    parser::Expr::Binary(_) => todo!(),
                    parser::Expr::Literal(expr) => {
                        let mut first = true;
                        for byte in expr.parse::<i32>().unwrap().to_le_bytes() { 
                            if byte == 0 {
                                if first {body.push(0x00);}
                                break;
                            } else {
                                first = false;
                                body.push(byte);
                            }
                        };
                    },
                    parser::Expr::Group(_) => todo!(),
                };
            },
            parser::PlatTypes::I64 => {
                body.push(0x42);
                match expr.value.as_ref().unwrap() {
                    parser::Expr::Unary(_) => todo!(),
                    parser::Expr::Binary(_) => todo!(),
                    parser::Expr::Literal(expr) => {
                        let mut first = true;
                        for byte in expr.parse::<i64>().unwrap().to_le_bytes() { 
                            if byte == 0 {
                                if first {body.push(0x00);}
                                break;
                            } else {
                                first = false;
                                body.push(byte);
                            }
                        };
                    },
                    parser::Expr::Group(_) => todo!(),
                };
            },
            parser::PlatTypes::F32 => {
                body.push(0x43);
                match expr.value.as_ref().unwrap() {
                    parser::Expr::Unary(_) => todo!(),
                    parser::Expr::Binary(_) => todo!(),
                    parser::Expr::Literal(expr) => {
                        for byte in expr.parse::<f32>().unwrap().to_le_bytes() { body.push(byte) };
                    },
                    parser::Expr::Group(_) => todo!(),
                };
            },
            parser::PlatTypes::F64 => {
                body.push(0x44);
                match expr.value.as_ref().unwrap() {
                    parser::Expr::Unary(_) => todo!(),
                    parser::Expr::Binary(_) => todo!(),
                    parser::Expr::Literal(expr) => {
                        for byte in expr.parse::<f64>().unwrap().to_le_bytes() { body.push(byte) };
                    },
                    parser::Expr::Group(_) => todo!(),
                };
            },
            _ => {assert!(false, "Invalid definition type! Type: {:?}", expr.kind)}
        };

        self.program.sec_code.append(&mut body);
    }

    pub fn compile_node(&mut self, node: &parser::AST) {
        match node {
            parser::AST::Func(expr) => self.compile_fn(expr),
            parser::AST::Def(expr) => self.compile_def(expr),
            parser::AST::Ret(_) => self.program.sec_code.push(0x0F),
        }
    }

    pub fn compile(&mut self, ast: Vec<parser::AST>) -> Vec<u8> {
        self.program.n_expos = 1;
        self.program.sec_expo.append(&mut vec![0x04, 0x6D, 0x61, 0x69, 0x6E, 0x00, 0x00]);

        for node in ast.iter() {
            self.compile_node(node);
        }
        
        self.program.unite()
    }
}