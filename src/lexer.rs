pub struct Lexer {
    pub file: String,
    pub cursor: usize
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum PlatTokenKinds {
    None,
    EOF,

    Keyword,
    Identifier,
    Literal,

    // Bracket Kinds
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,

    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,

    Assign,
    Equals,
    Bang,
    BangEquals,
    Smaller,
    SmallerEquals,
    Greater,
    GreaterEquals,
    
    // Seperators
    Comma,
    Semicolon,
    Colon
}

#[derive(Debug)]
pub struct PlatToken {
    pub kind: PlatTokenKinds,
    pub value: String,
}

impl Lexer {
    pub fn new(file: String) -> Lexer {
        Lexer {file: file, cursor: 0}
    }

    fn next_char(&mut self) -> char {
        self.cursor += 1;
        self.file.chars().nth(self.cursor).unwrap_or_default()
    }

    fn peek_char(&mut self) -> char {
        self.file.chars().nth(self.cursor + 1).unwrap_or_default()
    }

    fn is_keyword(value: &str) -> bool {
        match value {
            "func"   => true,
            "let"    => true,
            "return" => true,
            _ => false
        }
    }

    pub fn next(&mut self) -> PlatToken {
        let mut token = PlatToken{
            kind: PlatTokenKinds::None,
            value: "".to_string()
        };

        if self.cursor > self.file.len() - 1 {
            return PlatToken {
                kind: PlatTokenKinds::EOF,
                value: "".to_string()
            } 
        }

        let mut current = self.file.chars().nth(self.cursor).unwrap_or_default();
        if current.is_alphabetic() {
            token.kind = PlatTokenKinds::Identifier;
            token.value = current.to_string();
            while {current = self.next_char(); current.is_alphanumeric()} {
                token.value.push(current);
            }
            if Lexer::is_keyword(&token.value) {token.kind = PlatTokenKinds::Keyword};
            self.cursor -= 1;
        } else if current.is_numeric() {
            let mut float = false;
            token.kind = PlatTokenKinds::Literal;
            token.value = current.to_string();
            while {current = self.next_char(); current.is_numeric() || current == '.'} {
                if current == '.' && !float {float = true;}
                else if current == '.' && float {assert!(false, "Invalid Number Literal!");}
                token.value.push(current);
            }
            self.cursor -= 1;
            assert!(!self.peek_char().is_alphabetic(), "Invalid Number Literal!")
        } else if current == '\'' {
            token.kind = PlatTokenKinds::Literal;
            token.value = self.next_char().to_string();
            
            assert_eq!(self.next_char(), '\'', "Invalid End Of Character Notation!");
        } else if current == '"' {
            token.kind = PlatTokenKinds::Literal;
            token.value = String::new();
            while {current = self.next_char(); current != '"'} {
                token.value.push(current)
            }
        } else if current == '(' {
            token.kind = PlatTokenKinds::OpenParen;
            token.value = current.to_string();
        } else if current == ')' {
            token.kind = PlatTokenKinds::CloseParen;
            token.value = current.to_string();
        } else if current == '[' {
            token.kind = PlatTokenKinds::OpenBracket;
            token.value = current.to_string();
        } else if current == ']' {
            token.kind = PlatTokenKinds::CloseBracket;
            token.value = current.to_string();
        } else if current == '{' {
            token.kind = PlatTokenKinds::OpenCurly;
            token.value = current.to_string();
        } else if current == '}' {
            token.kind = PlatTokenKinds::CloseCurly;
            token.value = current.to_string();
        } else if current == '+' {
            token.kind = PlatTokenKinds::Plus;
            token.value = current.to_string();
        } else if current == '-' {
            token.kind = PlatTokenKinds::Minus;
            token.value = current.to_string();
        } else if current == '*' {
            token.kind = PlatTokenKinds::Asterisk;
            token.value = current.to_string();
        } else if current == '/' {
            token.kind = PlatTokenKinds::Slash;
            token.value = current.to_string();
        } else if current == '=' {  // = or ==
            token.kind = PlatTokenKinds::Assign;
            token.value = current.to_string();
            if self.peek_char() == '=' {
                token.kind = PlatTokenKinds::Equals;
                current = self.next_char();
                token.value.push(current);
            }
        } else if current == '!' {  // ! or !=
            token.kind = PlatTokenKinds::Bang;
            token.value = current.to_string();
            if self.peek_char() == '=' {
                token.kind = PlatTokenKinds::BangEquals;
                current = self.next_char();
                token.value.push(current);
            }
        } else if current == '<' {  // < or <=
            token.kind = PlatTokenKinds::Smaller;
            token.value = current.to_string();
            if self.peek_char() == '=' {
                token.kind = PlatTokenKinds::SmallerEquals;
                current = self.next_char();
                token.value.push(current);
            }
        } else if current == '>' {  // > or >=
            token.kind = PlatTokenKinds::Greater;
            token.value = current.to_string();
            if self.peek_char() == '=' {
                token.kind = PlatTokenKinds::GreaterEquals;
                current = self.next_char();
                token.value.push(current);
            }
        } else if current == ',' {
            token.kind = PlatTokenKinds::Comma;
            token.value = current.to_string();
        } else if current == ';' {
            token.kind = PlatTokenKinds::Semicolon;
            token.value = current.to_string();
        } else if current == ':' {
            token.kind = PlatTokenKinds::Colon;
            token.value = current.to_string();
        } else if current.is_whitespace() {
            self.cursor += 1;
            return self.next()
        }
        self.cursor += 1;

        return token;
    }
}