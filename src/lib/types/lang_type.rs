use super::func_type::FuncType;
use super::primitive_type::PrimitiveType;
use super::if_type::IfType;
use super::var_type::VarType;

#[derive(Clone)]
pub enum LangType {
    Primitive(PrimitiveType),
    Var(VarType),
    Func(FuncType),
    If(IfType),
}