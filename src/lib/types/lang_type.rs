use super::func_type::FuncType;
use super::primitive_type::PrimitiveType;
use super::if_type::IfType;
use super::var_type::VarType;
use super::op_type::OpType;

#[derive(Clone)]
pub enum LangType {
    Op(OpType),
    Primitive(PrimitiveType),
    Var(VarType),
    Func(FuncType),
    If(IfType),
    Undefined(u64),
}