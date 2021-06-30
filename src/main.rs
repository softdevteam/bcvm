use std::{collections::HashMap, env, option::Option::Some};

use llvm_ir::{
    constant::Constant,
    instruction::Instruction,
    name, BasicBlock, Module,
    Operand::{self, ConstantOperand, LocalOperand, MetadataOperand},
    Terminator,
};

struct BFInterp {
    mem: HashMap<name::Name, u64>,
}

impl BFInterp {
    fn new() -> BFInterp {
        BFInterp {
            mem: HashMap::new(),
        }
    }

    fn interpret(&mut self, module: Module) -> Result<(), &'static str> {
        let main_func = match module.get_func_by_name("main") {
            Some(main) => main,
            None => return Err("No main function"),
        };

        match self.it_bb(&main_func.basic_blocks[0]) {
            Some(_ret) => return Ok(()),
            None => return Ok(()),
        }
    }

    fn it_bb(&mut self, bb: &BasicBlock) -> Option<u64> {
        for inst in bb.instrs.iter() {
            match inst {
                Instruction::Alloca(_) => {}
                Instruction::Store(store) => self.store_var(&store.address, &store.value),
                _ => todo!(),
            }
        }
        match &bb.term {
            Terminator::Ret(rec) => match rec.return_operand.as_ref().unwrap() {
                LocalOperand { .. } => todo!(),
                ConstantOperand(con_ref) => match con_ref.as_ref() {
                    Constant::Int { bits: _, value } => {
                        return Some(*value);
                    }
                    _ => todo!(),
                },
                MetadataOperand => todo!(),
            },
            _ => todo!(),
        }
    }

    fn store_var(&mut self, op: &Operand, val: &Operand) {
        match op {
            LocalOperand { name, .. } => match val {
                LocalOperand { .. } => todo!(),
                ConstantOperand(con_ref) => match con_ref.as_ref() {
                    Constant::Int { bits: _, value } => {
                        self.mem.insert(name.clone(), *value);
                    }
                    _ => todo!(),
                },
                MetadataOperand => todo!(),
            },
            ConstantOperand(_) => todo!(),
            MetadataOperand => todo!(),
        }
    }
}

fn main() {
    let mut bf = BFInterp::new();
    let args: Vec<String> = env::args().collect();
    match create_module(&args[1]) {
        Ok(module) => match bf.interpret(module) {
            Ok(_) => {}
            Err(str) => println!("{}", str),
        },
        Err(error_message) => println!("{}", error_message),
    };
}

fn create_module(path_s: &str) -> Result<Module, String> {
    Module::from_bc_path(path_s)
}
