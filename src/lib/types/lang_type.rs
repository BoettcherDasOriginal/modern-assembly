use super::const_type::ConstType;
use super::func_type::FuncType;
use super::if_type::IfType;
use super::op_type::OpType;
use super::primitive_type::PrimitiveType;
use super::var_type::VarType;
use super::call_type::CallType;

#[derive(Clone, Debug)]
pub enum LangType {
    Op(OpType),
    Primitive(PrimitiveType),
    Const(ConstType),
    Var(VarType),
    Call(CallType),
    Func(FuncType),
    If(IfType),

    // Parser helper
    Else,
    End,
    Eof,
    Comment(String),

    Undefined,
}
