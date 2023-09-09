use crate::types::lang_type::LangType;
use super::var_type::VarType;

#[derive(Clone, Debug)]
pub struct FuncType {
    pub name: String,
    pub param: Vec<VarType>,
    pub body: Vec<LangType>,
}

impl Default for FuncType {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            param: vec![],
            body: vec![],
        }
    }
}

impl FuncType {
    pub fn new(name: String, param: Vec<VarType>,body: Vec<LangType>) -> Self {
        Self { name, param, body }
    }
}
