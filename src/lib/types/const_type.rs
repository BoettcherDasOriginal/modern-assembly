#[derive(Clone, Debug)]
pub struct ConstType {
    pub name: String,
    pub value: String,
}

impl Default for ConstType {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            value: "".to_string(),
        }
    }
}

impl ConstType {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
