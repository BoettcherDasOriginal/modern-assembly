use crate::types::lang_type::LangType;

#[derive(Clone, Debug)]
pub struct IfType {
    pub condition: Box<LangType>,
    pub body: Vec<LangType>,
    pub else_body: Vec<LangType>,
}

impl Default for IfType {
    fn default() -> Self {
        Self {
            condition: Box::new(LangType::Undefined(0)),
            body: vec![],
            else_body: vec![],
        }
    }
}

impl IfType {
    pub fn new(condition: LangType, body: Vec<LangType>, else_body: Vec<LangType>) -> Self {
        Self {
            condition: Box::new(condition),
            body: body,
            else_body: else_body,
        }
    }
}
