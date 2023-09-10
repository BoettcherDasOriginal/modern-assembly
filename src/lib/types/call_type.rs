use crate::types::lang_type::LangType;

#[derive(Clone, Debug)]
pub struct CallType {
    pub name: String,
    pub param: Vec<LangType>,
}

impl Default for CallType {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            param: vec![],
        }
    }
}

impl CallType {
    pub fn new(name: String, param: Vec<LangType>) -> Self {
        Self { name, param }
    }
}