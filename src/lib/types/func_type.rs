use crate::types::lang_type::LangType;

#[derive(Clone)]
pub struct FuncType {
    pub name: String,
    pub body: Vec<LangType>,
}

impl Default for FuncType {
    fn default() -> Self {
        Self { 
            name: "".to_string(), 
            body: vec![], 
        }
    }
}

impl FuncType {
    pub fn new(name: String, body: Vec<LangType>) -> Self{
        Self { 
            name: name, 
            body: body, 
        }
    }
}