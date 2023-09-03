use anyhow::Result;

use crate::lexer::lexer::Token;
use crate::types::func_type::FuncType;
use crate::types::lang_type::LangType;

pub struct ParserResult {
    pub lang_t: LangType,
    pub pos: usize,
}

impl ParserResult {
    pub fn new(lang_t: LangType,pos: usize) -> Self {
        Self { lang_t: lang_t, pos: pos }
    }
}

// -----

pub struct Parser {
    tokenlist: Vec<Token>,
    organized_tokenlist: Vec<Vec<Token>>, //Token list splited by new line
    ast: Vec<LangType>,
}

impl Parser {
    pub fn new(tokenlist: Vec<Token>) -> Self {
        Self {
            tokenlist: tokenlist.clone(),
            organized_tokenlist: organize_tokenlist(tokenlist),
            ast: vec![],
        }
    }

    pub fn generate_ast(&mut self) -> Result<Vec<LangType>>{
        let Self { tokenlist, organized_tokenlist, ast } = self;
        *ast = vec![];

        

        return Ok(ast.to_vec())
    }

    pub fn parse_line(&mut self,mut pos: usize) -> Result<ParserResult> {
        let Self { tokenlist, organized_tokenlist, ast } = self;
        let tok = &organized_tokenlist[pos][0];

        match tok {
            Token::Function => {
                let mut fn_name = "".to_string();
                if let Token::Ident(name) = &organized_tokenlist[pos][1] {
                    fn_name = name.to_string();
                }
                else {
                    return Ok(ParserResult::new(LangType::Undefined(0), pos));
                }

                let mut fn_body: Vec<LangType> = vec![];
                loop {
                    let lang_t = self.parse_line(pos + 1).unwrap();
                    if matches!(lang_t.lang_t,LangType::End){
                        break;
                    }
                    
                    fn_body.append(&mut vec![lang_t.lang_t]);
                    pos = lang_t.pos;
                }

                return Ok(ParserResult::new(LangType::Func(FuncType::new(fn_name, fn_body)), pos));
            },

            

            _ => {
                return Ok(ParserResult::new(LangType::Undefined(0), pos));
            }
        }
    }
}

// example token list
/* 
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
            Token::String(String::from("too baad!")),
            Token::NewLine,
            Token::End,
            Token::NewLine,
            Token::End,
            Token::NewLine,
            Token::Eof,
        ];*/

// -----------------

pub fn organize_tokenlist(tokenlist: Vec<Token>) -> Vec<Vec<Token>> {
    let mut organized_list = vec![vec![]];
    let mut level: Vec<Token> = vec![];

    for t in tokenlist {
        if t == Token::NewLine {
            organized_list.append(&mut vec![level.clone()]);
            level = vec![];
        } else {
            level.append(&mut vec![t.clone()]);
        }
    }

    return organized_list;
}
