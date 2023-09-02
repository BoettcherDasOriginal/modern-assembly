use crate::lexer::lexer::Token;
use crate::types::lang_type::LangType;

pub struct AstTree {
    tokenlist: Vec<Token>,
    organized_tokenlist: Vec<Vec<Token>>, //Token list splited by new line
}

impl AstTree {
    pub fn new(tokenlist: Vec<Token>) -> Self{
        Self {  tokenlist: tokenlist.clone(), organized_tokenlist: organize_tokenlist(tokenlist)}
    }
}

// -----------------

pub fn organize_tokenlist(tokenlist: Vec<Token>) -> Vec<Vec<Token>>{
    let mut organized_list = vec![vec![]];
    let mut level: Vec<Token> = vec![];

    for t in tokenlist  {
        if t == Token::NewLine{
            organized_list.append(&mut vec![level.clone()]);
            level = vec![];
        }
        else {
            level.append(&mut vec![t.clone()]);
        }
    }

    return organized_list;
}