#[derive(Clone, Debug)]
pub struct VarType {
    name: String,
}

impl Default for VarType {
    fn default() -> Self {
        Self {
            name: "".to_string(),
        }
    }
}

impl VarType {
    pub fn new(name: String) -> Self {
        Self { name: name }
    }
}
