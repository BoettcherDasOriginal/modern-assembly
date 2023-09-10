use std::collections::HashMap;
use anyhow::{anyhow, Result, Ok};

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::values::{BasicMetadataValueEnum, IntValue, FunctionValue, PointerValue, BasicValueEnum, BasicValue};
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::module::Module;
use inkwell::basic_block::BasicBlock;

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

    variables: HashMap<String, PointerValue<'ctx>>,

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
            else if let LangType::Const(con) = func_type {
                // To do: Const handling
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

        // build variables map
        self.variables.reserve(func_type.param.len());

        for (i, arg) in fn_val.get_param_iter().enumerate() {
            let arg_name = func_type.param[i].name.as_str();
            let alloca = self.create_entry_block_alloca(fn_val.get_first_basic_block().unwrap(),arg_name);

            self.builder.build_store(alloca, arg).unwrap();

            self.variables.insert(func_type.param[i].name.clone(), alloca);
        }

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

    fn create_entry_block_alloca(&self,entry: BasicBlock<'ctx>, name: &str) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(self.context.f64_type(), name).unwrap()
    }

    fn compile_expr(&mut self,expr_type: LangType) -> Result<BasicValueEnum<'ctx>> {
        Ok(match expr_type {
            LangType::Primitive(p) => {
                self.primitive(p)?
            },
            _ => BasicValueEnum::IntValue(self.context.i8_type().const_int(0, false))
        })
    }

    fn primitive(&mut self,prim_type: PrimitiveType) -> Result<BasicValueEnum<'ctx>> {
        Ok(match prim_type.primitive {
            Primitives::Int => {
                BasicValueEnum::IntValue(self.context.i32_type().const_int(prim_type.value.parse::<u64>()?, false))
            }
            Primitives::Bool => {
                if prim_type.value == "true"{
                    BasicValueEnum::IntValue(self.context.bool_type().const_int(0, false))
                }
                else {
                    BasicValueEnum::IntValue(self.context.bool_type().const_int(1, false))
                }
            }
            Primitives::String => {
                let global_str = self.builder.build_global_string_ptr(&prim_type.value, "str")?;

                global_str.as_basic_value_enum()
            }
        })
    }
}

