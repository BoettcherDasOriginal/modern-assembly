use crate::types::lang_type::LangType;
use super::primitive_type::PrimitiveType;
use super::primitive_type::Primitives;

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
            lhs: Box::new(LangType::Primitive(PrimitiveType { value: "0".to_string(), primitive: Primitives::Int })),
            rhs: Box::new(LangType::Primitive(PrimitiveType { value: "0".to_string(), primitive: Primitives::Int })),
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