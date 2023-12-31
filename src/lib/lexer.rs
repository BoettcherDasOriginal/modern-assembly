use std::fmt::Display;

use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Int(String),
    String(String),
    Bool(bool),
    Comment(String),

    Illegal,
    NewLine,
    Eof,

    Bang,
    Colon,
    Lparen,
    Rparen,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,

    Function,
    Const,
    Let,

    If,
    Else,
    Return,

    End,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(x) => write!(f, "Ident({})", x),
            Token::Int(x) => write!(f, "Int({})", x),
            Token::String(x) => write!(f, "String({})", x),
            Token::Bool(x) => write!(f, "Bool({})", x),
            Token::Comment(x) => write!(f, "Comment({})", x),
            Token::Illegal => write!(f, "Illegal"),
            Token::NewLine => write!(f, "NewLine"),
            Token::Eof => write!(f, "Eof"),
            Token::Bang => write!(f, "Bang"),
            Token::Equal => write!(f, "Equal"),
            Token::NotEqual => write!(f, "NotEqual"),
            Token::LessThan => write!(f, "LessThan"),
            Token::GreaterThan => write!(f, "GreaterThan"),
            Token::Colon => write!(f, "Colon"),
            Token::Lparen => write!(f, "Lparen"),
            Token::Rparen => write!(f, "Rparen"),
            Token::Function => write!(f, "Function"),
            Token::Const => write!(f, "Const"),
            Token::Let => write!(f, "Let"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::Return => write!(f, "Return"),
            Token::End => write!(f, "End"),
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };
        lex.read_char();

        lex
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let tok = match self.ch {
            b':' => Token::Colon,
            b'!' => {
                if self.peek() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'>' => Token::GreaterThan,
            b'<' => Token::LessThan,
            b'=' => {
                self.read_char();
                Token::Equal
            }
            b'"' => {
                let string_literal = self.read_string()?;
                Token::String(string_literal)
            }
            b'#' => {
                let comment = self.read_comment()?;
                Token::Comment(comment)
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return Ok(match ident.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "const" => Token::Const,
                    "if" => Token::If,
                    "false" => Token::Bool(false),
                    "true" => Token::Bool(true),
                    "return" => Token::Return,
                    "else" => Token::Else,
                    "end" => Token::End,
                    _ => Token::Ident(ident),
                });
            }
            b'0'..=b'9' => return Ok(Token::Int(self.read_int())),
            b'\n' => Token::NewLine,
            0 => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();
        Ok(tok)
    }

    fn peek(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' {
            self.read_char();
        }
    }

    fn read_string(&mut self) -> Result<String> {
        let mut string_literal = String::new();
        self.read_char();
        while self.ch != b'"' {
            if self.ch == 0 {
                return Err(anyhow::anyhow!("Unclosed string literal"));
            }
            string_literal.push(self.ch as char);
            self.read_char();
        }
        Ok(string_literal)
    }

    fn read_comment(&mut self) -> Result<String> {
        let mut comment = String::new();
        while !(self.peek() == b'\n' || self.peek() == 0) {
            self.read_char();
            comment.push(self.ch as char);
        }
        Ok(comment)
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    fn read_int(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    pub fn collect(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            if token == Token::Eof {
                break;
            }
            tokens.push(token);
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::{Lexer, Token};

    #[test]
    fn get_next_token() -> Result<()> {
        let input = "let b 15";
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::Let,
            Token::Ident(String::from("b")),
            Token::Int(String::from("15")),
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }

    #[test]
    fn get_next_complete() -> Result<()> {
        let input = r#"fn main:
            let a 5
            if a != 4:
                print("too baad!")
            end
        end
        "#;

        let mut lex = Lexer::new(input.into());

        let tokens = vec![
            Token::Function,
            Token::Ident(String::from("main")),
            Token::Colon,
            Token::NewLine,
            Token::Let,
            Token::Ident(String::from("a")),
            Token::Int(String::from("5")),
            Token::NewLine,
            Token::If,
            Token::Ident(String::from("a")),
            Token::NotEqual,
            Token::Int(String::from("4")),
            Token::Colon,
            Token::NewLine,
            Token::Ident(String::from("print")),
            Token::Lparen,
            Token::String(String::from("too baad!")),
            Token::Rparen,
            Token::NewLine,
            Token::End,
            Token::NewLine,
            Token::End,
            Token::NewLine,
            Token::Eof,
        ];

        for token in tokens {
            let next_token = lex.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }

    #[test]
    fn get_next_print() -> Result<()> {
        let input = r#"fn main:
            const msg "Hello, world!"
            print(msg)
                    
            let x 1
            add x 2
            print(x)
        
            if msg == x:
                print("???")
            else:
                move x 1
            end
            
            let aw
            move aw openDir("./test/RT.pdf")
        end

        fn openDir asd:
            add asd "something" # does some imaginary stuff here
            return asd
        end
        "#;

        let mut lex = Lexer::new(input.into());

        print!("{:?}", lex.collect());

        return Ok(());
    }
}
