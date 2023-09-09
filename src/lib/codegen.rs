use anyhow::{anyhow, Result, Ok};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::values::{BasicMetadataValueEnum, IntValue, FunctionValue, PointerValue};
use inkwell::module::Module;

use crate::types::func_type::FuncType;
use crate::types::if_type::IfType;
use crate::types::lang_type::LangType;
use crate::types::op_type::OpType;
use crate::types::op_type::Operation;
use crate::types::primitive_type::PrimitiveType;
use crate::types::primitive_type::Primitives;
use crate::types::var_type::VarType;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,

    module_ast: Vec<LangType>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn compile_module(&mut self, name: String, ast: Vec<LangType>) -> Result<Module<'ctx>> {
        self.module = self.context.create_module(&name.to_string());
        self.module_ast = ast;

        for func_type in self.module_ast.clone() {
            if let LangType::Func(func) = func_type {
                let new_fn = self.compile_fn(func)?;
                self.module.add_function(new_fn.get_name().to_str()?, new_fn.get_type(), Option::None);
            }
            else {
                return Err(anyhow!("Expression outside of function!"));
            }
        }

        Ok(self.module.to_owned())
    }

    fn compile_fn(&mut self, fn_type: FuncType) -> Result<FunctionValue<'ctx>> {
        let fn_type = self.context.f32_type().fn_type(&[], false);
        let fn_val = self.module.add_function("my_function", fn_type, None);
        Ok(fn_val)
    }
}

