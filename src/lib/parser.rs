use anyhow::Ok;
use anyhow::{anyhow, Result};

use crate::lexer::Token;
use crate::types::func_type::FuncType;
use crate::types::if_type::IfType;
use crate::types::lang_type::LangType;
use crate::types::op_type::OpType;
use crate::types::op_type::Operation;
use crate::types::primitive_type::PrimitiveType;
use crate::types::primitive_type::Primitives;
use crate::types::var_type::VarType;
use crate::types::const_type::ConstType;
use crate::types::call_type::CallType;

pub struct ParserResult {
    pub lang_t: LangType,
    pub pos: usize,
}

impl ParserResult {
    pub fn new(lang_t: LangType, pos: usize) -> Self {
        Self { lang_t, pos }
    }
}

// -----

#[derive(Debug)]
pub struct Parser {
    organized_tokenlist: Vec<Vec<Token>>, //Token list splitted by new line
}

impl Parser {
    pub fn new(tokenlist: Vec<Token>) -> Self {
        Self {
            organized_tokenlist: organize_tokenlist(&tokenlist),
        }
    }

    pub fn parse_file(&mut self) -> Result<Vec<LangType>> {
        let max_pos = self.organized_tokenlist.len();
        let mut ast = vec![];
        let mut pos = 0;

        while pos < max_pos {
            if pos >= max_pos {
                break;
            }

            let lang_t = self.parse_line(pos)?;
            if matches!(lang_t.lang_t, LangType::Eof) {
                break;
            }

            ast.append(&mut vec![lang_t.lang_t]);
            pos = lang_t.pos + 1;
        }

        Ok(ast)
    }

    fn parse_line(&mut self, mut pos: usize) -> Result<ParserResult> {
        if pos >= self.organized_tokenlist.len() {
            return Err(anyhow!("Expected end of input at position {}", pos));
        }
        if self.organized_tokenlist[pos].is_empty() {
            return Err(anyhow!("Empty line at position {}", pos));
        }
        let tok = &self.organized_tokenlist[pos][0];

        match tok {
            //Op parser
            Token::Ident(op_name) => {
                if OpType::is_op(op_name) {
                    let op = OpType::get_op_by_string(op_name);
                    if self.organized_tokenlist[pos].len() > 3 {
                        let var_name;
                        if let Token::Ident(name) = &self.organized_tokenlist[pos][1] {
                            var_name = name.to_string();
                        } else {
                            return Err(anyhow!("Unexpected Operand at position {}", pos));
                        }

                        let dest = LangType::Var(VarType::new(var_name));
                        let lhs = get_hs(self.organized_tokenlist.to_vec(), pos, 2)?;
                        let rhs = get_hs(self.organized_tokenlist.to_vec(), pos, 3)?;

                        let result = LangType::Op(OpType::new(op, lhs, rhs));
                        Ok(ParserResult::new(
                            LangType::Op(OpType::new(Operation::Assign, dest, result)),
                            pos,
                        ))
                    } else {
                        let var_name;
                        if let Token::Ident(name) = &self.organized_tokenlist[pos][1] {
                            var_name = name.to_string();
                        } else {
                            return Err(anyhow!("Unexpected Operand at position {}", pos));
                        }

                        let lhs = LangType::Var(VarType::new(var_name));
                        let rhs = get_hs(self.organized_tokenlist.to_vec(), pos, 2)?;

                        let result = LangType::Op(OpType::new(op, lhs.clone(), rhs));
                        Ok(ParserResult::new(
                            LangType::Op(OpType::new(Operation::Assign, lhs, result)),
                            pos,
                        ))
                    }
                } 
                else if self.organized_tokenlist[pos].len() > 2 {
                    if self.organized_tokenlist[pos][1] == Token::Lparen {
                        let mut param = Vec::new();
                        let mut win = false;
                        let mut i = 1;
                        for p in self.organized_tokenlist[pos].clone() {
                            if !win && p == Token::Lparen {
                                win = true;
                                continue;
                            }
                            else if win && p == Token::Rparen {
                                break;
                            }

                            if win {
                                let arg = get_hs(self.organized_tokenlist.to_vec(), pos, i)?;
                                param.append(&mut vec![arg]);
                            }

                            i = i + 1;
                        }
                        Ok(ParserResult::new(LangType::Call(CallType::new(op_name.to_string(), param)), pos))
                    }
                    else {
                        Err(anyhow!("Invalid Operation at position {}", pos))
                    }
                }
                else {
                    Err(anyhow!("Invalid Operation at position {}", pos))
                }
            }

            //Var parser
            Token::Let => {
                // make sure var is var
                let var_name;
                if let Token::Ident(name) = &self.organized_tokenlist[pos][1] {
                    var_name = name.to_string();
                } else {
                    return Err(anyhow!("Unexpected Variable Name at position {}", pos));
                }

                let lhs = LangType::Var(VarType::new(var_name));
                let rhs = get_hs(self.organized_tokenlist.to_vec(), pos, 2)?;

                Ok(ParserResult::new(
                    LangType::Op(OpType::new(Operation::Assign, lhs, rhs)),
                    pos,
                ))
            }

            //Const parser
            Token::Const => {
                let con_name;
                if let Token::Ident(name) = &self.organized_tokenlist[pos][1] {
                    con_name = name.to_string();
                } else {
                    return Err(anyhow!("Unexpected Variable Name at position {}", pos));
                }

                let prim;
                if let LangType::Primitive(p) = get_hs(self.organized_tokenlist.to_vec(), pos, 2)? {
                    prim = p;
                }
                else {
                    return Err(anyhow!("Unexpected Constant value at position {}", pos));
                }

                Ok(ParserResult::new(
                    LangType::Const(ConstType::new(con_name,prim.value)),
                    pos,
                ))
            }

            //Function parser
            Token::Function => {
                //get name
                let fn_name;
                if let Token::Ident(name) = &self.organized_tokenlist[pos][1] {
                    fn_name = name.to_string();
                } else {
                    return Err(anyhow!("Unexpected Function Name at position {}", pos));
                }

                //get params
                let mut params: Vec<VarType> = vec![];
                for par in self.organized_tokenlist[pos].clone() {
                    match par {
                        Token::Colon => {
                            break;
                        }

                        Token::Ident(name) => {
                            if name == fn_name{ continue; }
                            params.append(&mut vec![VarType::new(name)])
                        },

                        _ => {}
                    }
                }

                //get body
                let mut fn_body: Vec<LangType> = vec![];
                loop {
                    let lang_t = self.parse_line(pos + 1)?;
                    if matches!(lang_t.lang_t, LangType::End) {
                        break;
                    }
                    if matches!(lang_t.lang_t, LangType::Eof) {
                        break;
                    }

                    fn_body.append(&mut vec![lang_t.lang_t]);
                    pos = lang_t.pos;
                }

                Ok(ParserResult::new(
                    LangType::Func(FuncType::new(fn_name,params, fn_body)),
                    pos,
                ))
            }

            //If/else parser
            Token::If => {
                //lhs & rhs for the condition
                let lhs = get_hs(self.organized_tokenlist.to_vec(), pos, 1)?;

                let rhs = get_hs(self.organized_tokenlist.to_vec(), pos, 3)?;

                //op for the condition
                let condition = match &self.organized_tokenlist[pos][2] {
                    Token::Equal => LangType::Op(OpType::new(Operation::Equal, lhs, rhs)),

                    Token::NotEqual => LangType::Op(OpType::new(Operation::NotEqual, lhs, rhs)),

                    Token::LessThan => LangType::Op(OpType::new(Operation::LessThan, lhs, rhs)),

                    Token::GreaterThan => {
                        LangType::Op(OpType::new(Operation::GreaterThan, lhs, rhs))
                    }

                    _ => return Err(anyhow!("Expected Operator at position {}", pos,)),
                };

                //get if/else bodys

                let mut if_body: Vec<LangType> = vec![];
                loop {
                    let lang_t = self.parse_line(pos + 1)?;
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
                    let lang_t = self.parse_line(pos + 1)?;
                    if matches!(lang_t.lang_t, LangType::End) {
                        break;
                    }
                    if matches!(lang_t.lang_t, LangType::Eof) {
                        break;
                    }

                    else_body.append(&mut vec![lang_t.lang_t]);
                    pos = lang_t.pos;
                }

                Ok(ParserResult::new(
                    LangType::If(IfType::new(condition, if_body, else_body)),
                    pos,
                ))
            }
            //Comment
            Token::Comment(c) => Ok(ParserResult::new(LangType::Comment(c.to_string()), pos)),

            //End
            Token::End => Ok(ParserResult::new(LangType::End, pos)),

            //File End
            Token::Eof => Ok(ParserResult::new(LangType::Eof, pos)),

            _ => Err(anyhow!("Unexpected token at position {}", pos)),
        }
    }
}

// -----------------

pub fn organize_tokenlist(tokenlist: &[Token]) -> Vec<Vec<Token>> {
    let mut organized_list = vec![vec![]];
    let mut level: Vec<Token> = vec![];

    for t in tokenlist {
        if t == &Token::NewLine {
            if organized_list[0] == vec![] && !level.is_empty() {
                organized_list[0] = level;
                level = vec![];
            } else if !level.is_empty() {
                organized_list.append(&mut vec![level]);
                level = vec![];
            }
        } else {
            level.append(&mut vec![t.clone()]);
        }
    }

    organized_list
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

        _ => return Err(anyhow!("Invalid token at position {}, {}", x_pos, y_pos)),
    };

    Ok(hs)
}

// ------------------------------------
// Tests

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::Parser;
    use crate::lexer::Lexer;
    use crate::lexer::Token;

    #[test]
    fn parse_string() -> Result<()> {
        let input = r#"fn main x y:
        let a 5
        if a != 4:
            add a a 6
            print(a)
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

        let lex = Lexer::new(input.into()).collect()?;

        let mut par = Parser::new(lex.clone());

        println!("{:?}", lex);
        println!("{:?}", par.organized_tokenlist);
        println!("{:?}", par.parse_file());

        return Ok(());
    }
}
