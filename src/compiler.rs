use std::fs;

use crate::lexer;
use crate::parser;

pub struct Compiler {
    parser: parser::Parser,
    program: Program,
    functions: Vec<Function>
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

struct Function {
    _name: String,
    vars: Vec<Variable>,
    ret_kind: parser::PlatTypes
}

struct Variable {
    name: String,
    index: u8,
    _mutable: bool,
    kind: parser::PlatTypes
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

        let lexer = lexer::Lexer::new(file);

        Self { parser: parser::Parser::new(lexer), program: Program::new(), functions: vec![] }
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
        for arg in expr.args.iter() {
            self.program.sec_type.push(plat_type_to_byte(arg.kind));
        }

        let mut ret_kind: parser::PlatTypes = parser::PlatTypes::Void;
        if expr.ret_kind == parser::PlatTypes::Void {
            self.program.sec_type.push(0x00);                                                // No returns
        } else {
            self.program.sec_type.append(&mut vec![0x01, plat_type_to_byte(expr.ret_kind)]); // Return type
            ret_kind = expr.ret_kind;
        }
        self.functions.push(Function{_name: expr.name.clone(), vars: vec![], ret_kind: ret_kind});

        self.program.sec_func.push(self.program.n_funcs);
        self.program.n_funcs += 1;

        self.program.sec_code.push(0x00); // Function size placeholder
        let func_size_index = self.program.sec_code.len() - 1;
        let code_size = self.program.sec_code.len();

        self.program.sec_code.push(0x00); // Local declarations placeholder
        let mut func_locals_index = self.program.sec_code.len() - 1;
        for node in expr.body.as_ref().as_ref().unwrap().iter() {
            match node {
                parser::AST::Func(_) => assert!(false, "Nested functions aren't supported..."),
                parser::AST::Def(def) => {
                    let current_fn = self.functions.last_mut().unwrap();
                    current_fn.vars.push(Variable { name: def.name.clone(), index: (current_fn.vars.len() + expr.args.len()).try_into().unwrap(), _mutable: true, kind: def.kind });
                },
                _ => {}
            }
            self.compile_node(node);
        }
        self.program.sec_code.push(0x0B); // End of function
        
        // Add local declarations
        self.program.sec_code[func_locals_index] = <usize as TryInto<u8>>::try_into(self.functions.last().unwrap().vars.len()).unwrap();  // Number of local decls
        func_locals_index += 1;
        for node in self.functions.last().unwrap().vars.iter() {    
            self.program.sec_code.insert({let temp = func_locals_index; func_locals_index += 1; temp}, 1);
            self.program.sec_code.insert({let temp = func_locals_index; func_locals_index += 1; temp}, plat_type_to_byte(node.kind)); // Decl type
        }

        self.program.sec_code[func_size_index] = <usize as TryInto<u8>>::try_into(self.program.sec_code.len() - code_size).unwrap();
    }

    fn compile_def(&mut self, expr: &parser::DefExpr) {
        let mut body: Vec<u8> = vec![];

        let var_index: u8 = self.functions.last().unwrap().vars.iter().find(|var| var.name == expr.name).expect(&format!("The variable {} does not exist...", expr.name)).index.try_into().unwrap();

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
                        body.append(&mut vec![0x21, var_index]);
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
                        body.append(&mut vec![0x21, var_index]);
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
                        body.append(&mut vec![0x21, var_index]);
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
                        body.append(&mut vec![0x21, var_index]);
                    },
                    parser::Expr::Group(_) => todo!(),
                };
            },
            _ => {assert!(false, "Invalid definition type! Type: {:?}", expr.kind)}
        };

        self.program.sec_code.append(&mut body);
    }

    fn compile_ret(&mut self, expr: &Option<parser::Expr>) {
        match expr.as_ref().unwrap() {
            parser::Expr::Literal(name) => {
                let var: &Variable = self.functions.last().unwrap().vars.iter().find(|var| &var.name == name).expect(&format!("The variable {} does not exist...", name));
                let ret_kind: parser::PlatTypes = self.functions.last().unwrap().ret_kind;
                if  ret_kind != var.kind {assert!(false, "Invalid return type, expected {:?} got {:?}", ret_kind, var.kind)}
                self.program.sec_code.append(&mut vec![0x20, var.index, 0x0F]); // local.get (var_index) return
            },
            parser::Expr::Unary(expr) => {println!("ret unary  {:?}", expr); todo!()},
            parser::Expr::Binary(expr) => {println!("ret binary  {:?}", expr); todo!()},
            parser::Expr::Group(expr) => {println!("ret group  {:?}", expr); todo!()},
        };
    }

    pub fn compile_node(&mut self, node: &parser::AST) {
        match node {
            parser::AST::Func(expr) => self.compile_fn(expr),
            parser::AST::Def(expr) => self.compile_def(expr),
            parser::AST::Ret(expr) => self.compile_ret(expr),
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