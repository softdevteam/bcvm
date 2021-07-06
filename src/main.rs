use either::Either::{Left, Right};
use llvm_ir::{
    constant::{Constant, GetElementPtr},
    instruction::{Call, Instruction},
    name::{
        self,
        Name::{Name, Number},
    },
    BasicBlock, ConstantRef, Module,
    Operand::{self, ConstantOperand, LocalOperand, MetadataOperand},
    Terminator,
};
use std::{collections::HashMap, env, option::Option::Some};

struct BFInterp {
    mem: HashMap<name::Name, u64>,
    gl_vars: HashMap<name::Name, Constant>,
}

impl BFInterp {
    fn new() -> BFInterp {
        BFInterp {
            mem: HashMap::new(),
            gl_vars: HashMap::new(),
        }
    }

    fn interpret(&mut self, module: Module) -> Result<(), &'static str> {
        self.store_gl_var(&module);

        let main_func = match module.get_func_by_name("main") {
            Some(main) => main,
            None => return Err("No main function"),
        };

        match self.it_bb(&main_func.basic_blocks[0]) {
            Some(_ret) => return Ok(()),
            None => return Ok(()),
        }
    }

    fn store_gl_var(&mut self, module: &Module) {
        for gl_var in module.global_vars.iter() {
            match &gl_var.initializer {
                Some(init) => {
                    self.gl_vars
                        .insert(gl_var.name.clone(), init.as_ref().clone());
                }
                None => todo!(),
            }
        }
    }

    fn it_bb(&mut self, bb: &BasicBlock) -> Option<u64> {
        for inst in bb.instrs.iter() {
            match inst {
                Instruction::Alloca(_) => {}
                Instruction::Store(store) => self.store_var(&store.address, &store.value),
                Instruction::Load(load) => self.load(&load.address, &load.dest),
                Instruction::Call(call) => self.call_func(call),
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

    fn load(&mut self, op: &Operand, dest: &name::Name) {
        match op {
            LocalOperand { name, .. } => {
                self.mem.insert(dest.clone(), *self.mem.get(name).unwrap());
            }
            ConstantOperand(_) => todo!(),
            MetadataOperand => todo!(),
        }
    }

    fn call_func(&mut self, call: &Call) {
        match &call.function {
            Left(_) => todo!(),
            Right(op) => match op {
                LocalOperand { .. } => todo!(),
                ConstantOperand(con_op) => match con_op.as_ref() {
                    Constant::GlobalReference { name, .. } => match name {
                        Name(n) => {
                            if n.as_str() == "printf" {
                                self.printf(call);
                            }
                        }
                        Number(_) => todo!(),
                    },
                    _ => todo!(),
                },
                MetadataOperand => todo!(),
            },
        }
    }

    fn printf(&mut self, call: &Call) {
        let constant = match &call.arguments[0].0 {
            LocalOperand { .. } => todo!(),
            ConstantOperand(const_op) => match const_op.as_ref() {
                Constant::GetElementPtr(GetElementPtr { address, .. }) => match address.as_ref() {
                    Constant::GlobalReference { name, .. } => self.gl_vars.get(name).unwrap(),
                    _ => todo!(),
                },
                _ => todo!(),
            },
            MetadataOperand => todo!(),
        };

        let mut string = String::new();
        let mut arg_it = call.arguments[1..].into_iter();
        match constant {
            Constant::Array { elements, .. } => {
                let mut i = 0;
                while i < elements.len() {
                    match elements[i].as_ref() {
                        Constant::Int { value, .. } => {
                            let ch = *value as u8 as char;
                            if ch == '%' {
                                string.push_str(
                                    self.arg_to_string(&arg_it.next().unwrap().0).as_str(),
                                );
                                i += 2;
                                continue;
                            } else if value == &0u64 {
                                i += 1;
                                continue;
                            }
                            string.push(ch);
                        }
                        _ => todo!(),
                    }
                    i += 1;
                }
            }
            _ => todo!(),
        }

        println!("{}", string);
    }

    fn arg_to_string(&self, arg: &Operand) -> String {
        match arg {
            LocalOperand { name, .. } => self.mem.get(&name).unwrap().to_string(),
            ConstantOperand(con_op) => match con_op.as_ref() {
                Constant::Int { value, .. } => value.to_string(),
                Constant::GetElementPtr(GetElementPtr { address, .. }) => match address.as_ref() {
                    Constant::GlobalReference { name, .. } => {
                        match self.gl_vars.get(name).unwrap() {
                            Constant::Array { elements, .. } => {
                                assert!(&name.to_string()[1..5] == ".str");
                                self.arr_to_string(elements)
                            }
                            _ => todo!(),
                        }
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            },
            MetadataOperand => todo!(),
        }
    }

    fn arr_to_string(&self, elements: &Vec<ConstantRef>) -> String {
        let mut string = String::new();
        for elem in elements.iter() {
            match elem.as_ref() {
                Constant::Int { value, .. } => {
                    if value != &0u64 {
                        string.push(*value as u8 as char)
                    }
                }
                _ => todo!(),
            }
        }
        string
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
