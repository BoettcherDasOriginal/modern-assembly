use super::func_type::FuncType;
use super::if_type::IfType;
use super::op_type::OpType;
use super::primitive_type::PrimitiveType;
use super::var_type::VarType;

#[derive(Clone)]
pub enum LangType {
    Op(OpType),
    Primitive(PrimitiveType),
    Var(VarType),
    Func(FuncType),
    If(IfType),
    Undefined(u64),
}
