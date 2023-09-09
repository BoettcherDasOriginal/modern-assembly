use anyhow::{anyhow, Result, Ok};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::values::{BasicMetadataValueEnum, IntValue, FunctionValue, PointerValue};
use inkwell::types::BasicMetadataTypeEnum;
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

    fn compile_fn(&mut self, func_type: FuncType) -> Result<FunctionValue<'ctx>> {

        //-----------
        // Prototype
        //-----------

        let ret_type = self.context.i32_type();
        let args_types = std::iter::repeat(ret_type)
            .take(func_type.param.len())
            .map(|f| f.into())
            .collect::<Vec<BasicMetadataTypeEnum>>();
        let args_types = args_types.as_slice();
        
        let fn_type = self.context.i32_type().fn_type(&args_types, false);
        let fn_val = self.module.add_function(&func_type.name, fn_type, None);

        // set arguments names
        for (i, arg) in fn_val.get_param_iter().enumerate() {
            arg.into_float_value().set_name(func_type.param[i].name.as_str());
        }

        // got external function, returning only compiled prototype
        if func_type.body.is_empty() {
            return Ok(fn_val);
        }

        let entry = self.context.append_basic_block(fn_val, "entry");

        self.builder.position_at_end(entry);


        //------------
        // Body
        //------------

        //To do: Compile Body

        Ok(fn_val)
    }
}

