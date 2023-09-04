use anyhow::Ok;
use anyhow::Result;

use crate::lexer::lexer::Token;
use crate::types::func_type::FuncType;
use crate::types::if_type::IfType;
use crate::types::lang_type::LangType;
use crate::types::op_type::OpType;
use crate::types::op_type::Operation;
use crate::types::primitive_type::PrimitiveType;
use crate::types::primitive_type::Primitives;
use crate::types::var_type::VarType;

pub struct ParserResult {
    pub lang_t: LangType,
    pub pos: usize,
}

impl ParserResult {
    pub fn new(lang_t: LangType, pos: usize) -> Self {
        Self {
            lang_t: lang_t,
            pos: pos,
        }
    }
}

// -----

#[derive(Debug)]
pub struct Parser {
    tokenlist: Vec<Token>,
    organized_tokenlist: Vec<Vec<Token>>, //Token list splited by new line
}

impl Parser {
    pub fn new(tokenlist: Vec<Token>) -> Self {
        Self {
            tokenlist: tokenlist.clone(),
            organized_tokenlist: organize_tokenlist(tokenlist),
        }
    }

    pub fn parse_file(&mut self) -> Result<Vec<LangType>> {
        let Self {
            tokenlist,
            organized_tokenlist,
        } = self;

        *organized_tokenlist = organize_tokenlist(tokenlist.to_vec());
        let mut ast: Vec<LangType> = vec![];
        let mut pos = 0;
        let max_pos = organized_tokenlist.len();

        loop {
            if pos >= max_pos {
                break;
            }

            let lang_t = self.parse_line(pos).unwrap();
            if matches!(lang_t.lang_t, LangType::Eof) {
                break;
            }

            ast.append(&mut vec![lang_t.lang_t]);
            pos = lang_t.pos + 1;
        }

        return Ok(ast);
    }

    fn parse_line(&mut self, mut pos: usize) -> Result<ParserResult> {
        let Self {
            tokenlist: _,
            organized_tokenlist,
        } = self;
        if pos >= organized_tokenlist.len() {
            return Ok(ParserResult::new(LangType::Undefined(0), pos));
        }
        if &0 == &organized_tokenlist[pos].len() {
            return Ok(ParserResult::new(LangType::Undefined(0), pos));
        }
        let tok = &organized_tokenlist[pos][0];

        match tok {
            //Op parser
            Token::Ident(op_name) => {
                if OpType::is_op(op_name) {
                    let op = OpType::get_op_by_string(op_name);
                    if organized_tokenlist[pos].len() > 3 {
                        let mut var_name = "".to_string();
                        if let Token::Ident(name) = &organized_tokenlist[pos][1] {
                            var_name = name.to_string();
                        } else {
                            return Ok(ParserResult::new(LangType::Undefined(0), pos));
                        }

                        let dest = LangType::Var(VarType::new(var_name));
                        let lhs = get_hs(organized_tokenlist.to_vec(), pos, 2).unwrap();
                        let rhs = get_hs(organized_tokenlist.to_vec(), pos, 3).unwrap();

                        let result = LangType::Op(OpType::new(op, lhs, rhs));
                        return Ok(ParserResult::new(
                            LangType::Op(OpType::new(Operation::Assign, dest, result)),
                            pos,
                        ));
                    } else {
                        let mut var_name = "".to_string();
                        if let Token::Ident(name) = &organized_tokenlist[pos][1] {
                            var_name = name.to_string();
                        } else {
                            return Ok(ParserResult::new(LangType::Undefined(0), pos));
                        }

                        let lhs = LangType::Var(VarType::new(var_name));
                        let rhs = get_hs(organized_tokenlist.to_vec(), pos, 2).unwrap();

                        let result = LangType::Op(OpType::new(op, lhs.clone(), rhs));
                        return Ok(ParserResult::new(
                            LangType::Op(OpType::new(Operation::Assign, lhs, result)),
                            pos,
                        ));
                    }
                } else {
                    return Ok(ParserResult::new(LangType::Undefined(0), pos));
                }
            }

            //Var parser
            Token::Var | Token::Let => {
                // make sure var is var
                let mut var_name = "".to_string();
                if let Token::Ident(name) = &organized_tokenlist[pos][1] {
                    var_name = name.to_string();
                } else {
                    return Ok(ParserResult::new(LangType::Undefined(0), pos));
                }

                let lhs = LangType::Var(VarType::new(var_name));
                let rhs = get_hs(organized_tokenlist.to_vec(), pos, 2).unwrap();

                return Ok(ParserResult::new(
                    LangType::Op(OpType::new(Operation::Assign, lhs, rhs)),
                    pos,
                ));
            }

            //Function parser
            Token::Function => {
                let mut fn_name = "".to_string();
                if let Token::Ident(name) = &organized_tokenlist[pos][1] {
                    fn_name = name.to_string();
                } else {
                    return Ok(ParserResult::new(LangType::Undefined(0), pos));
                }

                let mut fn_body: Vec<LangType> = vec![];
                loop {
                    let lang_t = self.parse_line(pos + 1).unwrap();
                    if matches!(lang_t.lang_t, LangType::End) {
                        break;
                    }
                    if matches!(lang_t.lang_t, LangType::Eof) {
                        break;
                    }

                    fn_body.append(&mut vec![lang_t.lang_t]);
                    pos = lang_t.pos;
                }

                return Ok(ParserResult::new(
                    LangType::Func(FuncType::new(fn_name, fn_body)),
                    pos,
                ));
            }

            //If/else parser
            Token::If => {
                //lhs & rhs for the condition
                let lhs = get_hs(organized_tokenlist.to_vec(), pos, 1).unwrap();

                let rhs = get_hs(organized_tokenlist.to_vec(), pos, 3).unwrap();

                //op for the condition
                let condition = match &organized_tokenlist[pos][2] {
                    Token::Equal => LangType::Op(OpType::new(Operation::Equal, lhs, rhs)),

                    Token::NotEqual => LangType::Op(OpType::new(Operation::NotEqual, lhs, rhs)),

                    Token::LessThan => LangType::Op(OpType::new(Operation::LessThan, lhs, rhs)),

                    Token::GreaterThan => {
                        LangType::Op(OpType::new(Operation::GreaterThan, lhs, rhs))
                    }

                    _ => LangType::Undefined(0),
                };

                //get if/else bodys

                let mut if_body: Vec<LangType> = vec![];
                loop {
                    let lang_t = self.parse_line(pos + 1).unwrap();
                    if matches!(lang_t.lang_t, LangType::End) {
                        break;
                    }
                    if matches!(lang_t.lang_t, LangType::Else) {
                        break;
                    }
                    if matches!(lang_t.lang_t, LangType::Eof) {
                        break;
                    }

                    if_body.append(&mut vec![lang_t.lang_t]);
                    pos = lang_t.pos;
                }

                let mut else_body: Vec<LangType> = vec![];
                loop {
                    let lang_t = self.parse_line(pos + 1).unwrap();
                    if matches!(lang_t.lang_t, LangType::End) {
                        break;
                    }
                    if matches!(lang_t.lang_t, LangType::Eof) {
                        break;
                    }

                    else_body.append(&mut vec![lang_t.lang_t]);
                    pos = lang_t.pos;
                }

                return Ok(ParserResult::new(
                    LangType::If(IfType::new(condition, if_body, else_body)),
                    pos,
                ));
            }

            //End
            Token::End => {
                return Ok(ParserResult::new(LangType::End, pos));
            }

            //File End
            Token::Eof => {
                return Ok(ParserResult::new(LangType::Eof, pos));
            }

            _ => {
                return Ok(ParserResult::new(LangType::Undefined(0), pos));
            }
        }
    }
}

// -----------------

pub fn organize_tokenlist(tokenlist: Vec<Token>) -> Vec<Vec<Token>> {
    let mut organized_list = vec![vec![]];
    let mut level: Vec<Token> = vec![];

    for t in tokenlist {
        if t == Token::NewLine {
            if organized_list[0] == vec![] && level.len() != 0 {
                organized_list[0] = level.clone();
                level = vec![];
            } else if level.len() != 0 {
                organized_list.append(&mut vec![level.clone()]);
                level = vec![];
            }
        } else {
            level.append(&mut vec![t.clone()]);
        }
    }

    return organized_list;
}

fn get_hs(organized_tokenlist: Vec<Vec<Token>>, x_pos: usize, y_pos: usize) -> Result<LangType> {
    let hs = match &organized_tokenlist[x_pos][y_pos] {
        Token::Ident(ident) => LangType::Var(VarType::new(ident.to_string())),

        Token::Int(value) => {
            LangType::Primitive(PrimitiveType::new(value.to_string(), Primitives::Int))
        }

        Token::String(value) => {
            LangType::Primitive(PrimitiveType::new(value.to_string(), Primitives::String))
        }

        Token::Bool(value) => {
            LangType::Primitive(PrimitiveType::new(value.to_string(), Primitives::Bool))
        }

        _ => LangType::Undefined(0),
    };

    Ok(hs)
}

// ------------------------------------
// Tests

#[cfg(test)]
mod test {
    use anyhow::Ok;
    use anyhow::Result;

    use super::Parser;
    use crate::lexer::lexer::Lexer;
    use crate::lexer::lexer::Token;

    #[test]
    fn parse_string() -> Result<()> {
        let input = r#"fn main:
        let a 5
        if a != 4:
            add a a 6
        end
    end"#;

        let _tokens = vec![
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
            Token::Ident(String::from("Add")),
            Token::Ident(String::from("a")),
            Token::Int(String::from("6")),
            Token::NewLine,
            Token::End,
            Token::NewLine,
            Token::End,
            Token::NewLine,
            Token::Eof,
            Token::NewLine,
        ];

        let mut lex = Lexer::new(input.into());

        let mut par = Parser::new(lex.collect().unwrap());

        println!("{:?}", par.tokenlist);
        println!("{:?}", par.organized_tokenlist);
        println!("{:?}", par.parse_file());

        return Ok(());
    }
}
