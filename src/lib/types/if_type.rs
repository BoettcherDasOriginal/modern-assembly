use crate::types::lang_type::LangType;

#[derive(Clone)]
pub enum Operator {
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

#[derive(Clone)]
pub struct IfType{
    op: Operator,
    lhs: Box<LangType>,
    rhs: Box<LangType>,
    body: Vec<LangType>,
    else_body: Vec<LangType>,
}

impl Default for IfType {
    fn default() -> Self {
        Self { 
            op: Operator::Equal, 
            lhs: Box::new(LangType::Undefined(0)),
            rhs: Box::new(LangType::Undefined(0)),
            body: vec![], 
            else_body: vec![], 
        }
    }
}

impl IfType {
    pub fn new(op: Operator,lhs: LangType,rhs: LangType, body: Vec<LangType>, else_body: Vec<LangType>) -> Self{
        Self { op: op,lhs: Box::new(lhs),rhs: Box::new(rhs), body: body, else_body: else_body }
    }
}