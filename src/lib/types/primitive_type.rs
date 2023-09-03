//use crate::types::lang_type::LangType;

#[derive(Clone, Debug)]
pub enum Primitives {
    Int,
    String,
    Bool,
}

#[derive(Clone, Debug)]
pub struct PrimitiveType {
    pub value: String,
    pub primitive: Primitives,
}

impl Default for PrimitiveType {
    fn default() -> Self {
        Self {
            value: "".to_string(),
            primitive: Primitives::Int,
        }
    }
}

impl PrimitiveType {
    pub fn new(value: String, primitive: Primitives) -> Self {
        Self {
            value: value,
            primitive: primitive,
        }
    }

    pub fn is_int(&self) -> bool {
        matches!(self.primitive, Primitives::Int)
    }

    pub fn is_string(&self) -> bool {
        matches!(self.primitive, Primitives::String)
    }

    pub fn is_bool(&self) -> bool{
        matches!(self.primitive, Primitives::Bool)
    }
}
