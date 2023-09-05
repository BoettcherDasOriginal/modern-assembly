use super::lang_type::LangType;

#[derive(Clone, Debug)]
pub enum Operation {
    //Basic Assignment Operator
    Assign, // let x 0 or move x 0 or in c: x = 0

    //Arithmetic Operators
    Add, // + -> Add x 1 2
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %

    //Relational Operators
    Equal,       // ==
    NotEqual,    // !=
    LessThan,    // <
    GreaterThan, // >

    //Debug
    Error,
}

#[derive(Clone, Debug)]
pub struct OpType {
    pub op: Operation,
    pub lhs: Box<LangType>,
    pub rhs: Box<LangType>,
}

impl Default for OpType {
    fn default() -> Self {
        Self {
            op: Operation::Assign,
            lhs: Box::new(LangType::Undefined(0)),
            rhs: Box::new(LangType::Undefined(0)),
        }
    }
}

impl OpType {
    pub fn new(op: Operation, lhs: LangType, rhs: LangType) -> Self {
        Self {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn get_op_by_string(op_name: &str) -> Operation {
        match op_name {
            "add" => Operation::Add,
            "sub" => Operation::Sub,
            "mul" => Operation::Mul,
            "div" => Operation::Div,
            "mod" => Operation::Mod,
            _ => Operation::Error,
        }
    }

    pub fn is_op(op_name: &str) -> bool {
        let op = OpType::get_op_by_string(op_name);
        !matches!(op, Operation::Error)
    }
}
