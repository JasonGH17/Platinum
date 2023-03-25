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

// Function Structs
#[derive(Debug)]
struct FuncExprArg {
    name: String,
    kind: PlatTypes
}

#[derive(Debug)]
pub struct FuncExpr {
    name: String,
    args: Vec<FuncExprArg>,
    ret_kind: PlatTypes,
    body: Box<Option<Vec<AST>>>
}

impl std::fmt::Display for FuncExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Function Definition:\nName: {}\nArgs:", self.name).expect("ERROR");
        for arg in self.args.iter() {
            writeln!(f, "\t- Function Argument: {} ({:?})", arg.name, arg.kind).expect("ERROR");
        }
        writeln!(f, "Ret-Type: {:?}\nBody:", self.ret_kind).expect("ERROR");
        for node in self.body.as_ref().as_ref().unwrap().iter() {
            writeln!(f, "{}", node).expect("ERROR");
        }

        Ok(())
    }
}

// Structs
#[derive(Debug)]
pub struct BinaryExpr{
    left: Expr,
    operator: lexer::PlatToken,
    right: Expr
}

#[derive(Debug)]
pub struct UnaryExpr {
    operator: lexer::PlatToken,
    expr: Expr
}

#[derive(Debug)]
pub enum Expr {
    Unary(Box<UnaryExpr>),
    Binary(Box<BinaryExpr>),
    Literal(String),
    Group(Box<Expr>)
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Unary(expr) => writeln!(f, "Unary Expr: {:?}{}", expr.operator.kind, expr.expr),
            Expr::Binary(expr) => writeln!(f, "Binary Expr: {}{:?}{}", expr.left, expr.operator.kind, expr.right),
            Expr::Literal(expr) => writeln!(f, "Literal Expr: {}", expr),
            Expr::Group(expr) => writeln!(f, "Group Expr: {}", expr),
        }    
    }
}

#[derive(Debug)]
pub struct DefExpr {
    name: String,
    kind: PlatTypes,
    value: Option<Expr>
}

impl std::fmt::Display for DefExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Definition Expression:\n\tName: {}\n\tType: {:?}\n\tValue: {}", self.name, self.kind, self.value.as_ref().unwrap())
    }
}

// AST
#[derive(Debug)]
pub enum AST {
    Func(FuncExpr),
    Def(DefExpr),
    Ret(Option<Expr>)
}

impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AST::Func(expr) => writeln!(f, "AST Function Node:\n{}", expr),
            AST::Def(expr) => writeln!(f, "AST Definition Node:\n{}", expr),
            AST::Ret(expr) => writeln!(f, "AST Return Node:\n{}", expr.as_ref().unwrap()),
        }
    }
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
        let mut expr = FuncExpr{ 
            name: self.expect_next_kind(lexer::PlatTokenKinds::Identifier).value,
            args: self.parse_func_args(),
            ret_kind: self.parse_type(),
            body: Box::new(None)
        };
        self.expect_next_kind(lexer::PlatTokenKinds::OpenCurly);
        expr.body = Box::new(Some(self.parse(Some(lexer::PlatTokenKinds::CloseBracket))));

        return expr;
    }

    fn parse_def(&mut self) -> DefExpr {
        let mut expr = DefExpr{
            name: self.expect_next_kind(lexer::PlatTokenKinds::Identifier).value,
            kind: PlatTypes::Void, 
            value: None
        };
        self.expect_next_kind(lexer::PlatTokenKinds::Colon);
        expr.kind = self.parse_type();
        self.expect_next_kind(lexer::PlatTokenKinds::Assign);
        expr.value = self.parse_expr();

        return expr;
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        let token = self.lexer.next();
        match token.kind {
            lexer::PlatTokenKinds::Literal | lexer::PlatTokenKinds::Identifier => Some(Expr::Literal(token.value)),
            lexer::PlatTokenKinds::Bang | lexer::PlatTokenKinds::Minus => Some(Expr::Unary(Box::new(UnaryExpr { operator: token, expr: self.parse_expr().unwrap() }))),
            _ => None
        }
    }

    pub fn parse(&mut self, end: Option<lexer::PlatTokenKinds>) -> Vec<AST> {
        let end_token = end.unwrap_or(lexer::PlatTokenKinds::EOF);
        let mut ast = Vec::<AST>::new();
        let mut token;
        while {token = self.lexer.next(); token.kind != end_token && token.kind != lexer::PlatTokenKinds::EOF} {
            match token.kind {
                lexer::PlatTokenKinds::Keyword => {
                    if token.value == "func".to_string() {
                        ast.push(AST::Func(self.parse_func()));
                    } else if token.value == "let".to_string() {
                        ast.push(AST::Def(self.parse_def()));
                    } else if token.value == "return".to_string() {
                        ast.push(AST::Ret(self.parse_expr()));
                    };
                },
                _ => continue
            }
        }

        return ast;
    }
}
