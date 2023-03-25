use crate::lexer;

pub struct Parser {
    pub lexer: lexer::Lexer,
    cursor: usize
}

#[derive(Debug)]
enum PlatTypes {
    Void,
    I32,
    I64,
    F32,
    F64
}

#[derive(Debug)]
struct FuncExprArg {
    name: String,
    kind: PlatTypes
}

#[derive(Debug)]
struct FuncExpr {
    name: String,
    args: Vec<FuncExprArg>,
    ret_kind: PlatTypes
}

enum AST {
    Func(FuncExpr)
}

impl Parser {
    pub fn new(lexer: lexer::Lexer) -> Self {
        Parser{lexer, cursor: 0 }
    }

    fn expect_next_kind(&mut self, kind: lexer::PlatTokenKinds) -> lexer::PlatToken {
        let token = self.lexer.next();
        assert_eq!(kind, token.kind, "Unexpected Token!\n{:?}", token);
        return token;
    }

    fn parse_type(&mut self) -> PlatTypes {
        let token = self.expect_next_kind(lexer::PlatTokenKinds::Identifier);
        match token.value.as_str() {
            "void" => PlatTypes::Void,
            "i32" => PlatTypes::I32,
            "i64" => PlatTypes::I64,
            "f32" => PlatTypes::F32,
            "f64" => PlatTypes::F64,
            _ => {assert!(false, "Invalid Type Provided!"); PlatTypes::Void},
        }
    }

    fn parse_func_args(&mut self) -> Vec<FuncExprArg> {
        let mut args = vec![];
        let mut token;

        while{
            token = self.lexer.next(); 
            token.kind != lexer::PlatTokenKinds::CloseParen
        } {
            if args.len() != 0
                {assert!(token.kind == lexer::PlatTokenKinds::Comma, "Invalid Argument Notation!");}
            let mut arg = FuncExprArg{name: self.expect_next_kind(lexer::PlatTokenKinds::Identifier).value, kind: PlatTypes::Void};
            self.expect_next_kind(lexer::PlatTokenKinds::Colon);
            arg.kind = self.parse_type();
            args.push(arg);
        }

        return args;
    }

    fn parse_func(&mut self) -> FuncExpr {
        let expr = FuncExpr{ 
            name: self.expect_next_kind(lexer::PlatTokenKinds::Identifier).value,
            args: self.parse_func_args(),
            ret_kind: self.parse_type()
        };

        return expr;
    }

    pub fn parse(&mut self) {
        let mut token;
        while {token = self.lexer.next(); token.kind != lexer::PlatTokenKinds::EOF} {
            match token.kind {
                crate::lexer::PlatTokenKinds::Keyword => {
                    if token.value == "func".to_string() {
                        println!("{:?}", self.parse_func());
                    };
                },
                _ => continue
            }
        }
    }
}
