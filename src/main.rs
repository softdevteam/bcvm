use core::panic;
use either::Either::{Left, Right};
use llvm_ir::{
    constant::{Constant, Float, GetElementPtr},
    instruction::{Call, Instruction},
    name::{
        self,
        Name::{Name, Number},
    },
    types::FPType,
    BasicBlock, ConstantRef, Module,
    Operand::{self, ConstantOperand, LocalOperand, MetadataOperand},
    Terminator, Type, TypeRef,
};
use std::{collections::HashMap, convert::TryInto, env, mem};

enum BinOps {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

struct BFInterp {
    vars: HashMap<name::Name, Operand>,
    gl_vars: HashMap<name::Name, Constant>,
}

impl BFInterp {
    fn new() -> BFInterp {
        BFInterp {
            vars: HashMap::new(),
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
                Instruction::Add(add) => {
                    self.int_bin_operations(&add.operand0, &add.operand1, &add.dest, BinOps::Add)
                }
                Instruction::FAdd(fadd) => {
                    self.fl_bin_operations(&fadd.operand0, &fadd.operand1, &fadd.dest, BinOps::Add)
                }
                Instruction::FPExt(fpext) => {
                    self.fpext(&fpext.operand, &fpext.to_type, &fpext.dest)
                }
                Instruction::Sub(sub) => {
                    self.int_bin_operations(&sub.operand0, &sub.operand1, &sub.dest, BinOps::Sub)
                }
                Instruction::Mul(mul) => {
                    self.int_bin_operations(&mul.operand0, &mul.operand1, &mul.dest, BinOps::Mul)
                }
                Instruction::UDiv(udiv) => {
                    self.int_bin_operations(&udiv.operand0, &udiv.operand1, &udiv.dest, BinOps::Div)
                }
                Instruction::SDiv(sdiv) => {
                    self.int_bin_operations(&sdiv.operand0, &sdiv.operand1, &sdiv.dest, BinOps::Div)
                }
                Instruction::URem(urem) => {
                    self.int_bin_operations(&urem.operand0, &urem.operand1, &urem.dest, BinOps::Rem)
                }
                Instruction::SRem(srem) => {
                    self.int_bin_operations(&srem.operand0, &srem.operand1, &srem.dest, BinOps::Rem)
                }
                Instruction::FSub(fsub) => {
                    self.fl_bin_operations(&fsub.operand0, &fsub.operand1, &fsub.dest, BinOps::Sub)
                }
                Instruction::SExt(sext) => self.sext(&sext.operand, &sext.to_type, &sext.dest),
                Instruction::FMul(fmul) => {
                    self.fl_bin_operations(&fmul.operand0, &fmul.operand1, &fmul.dest, BinOps::Mul)
                }
                Instruction::FDiv(fdiv) => {
                    self.fl_bin_operations(&fdiv.operand0, &fdiv.operand1, &fdiv.dest, BinOps::Div)
                }
                _ => todo!(),
            }
        }
        match &bb.term {
            Terminator::Ret(ret) => match ret.return_operand.as_ref() {
                Some(op) => match op {
                    LocalOperand { .. } => todo!(),
                    ConstantOperand(con_ref) => match con_ref.as_ref() {
                        Constant::Int { bits: _, value } => {
                            return Some(*value);
                        }
                        _ => todo!(),
                    },
                    MetadataOperand => todo!(),
                },
                None => None,
            },
            _ => todo!(),
        }
    }

    fn store_var(&mut self, op: &Operand, val: &Operand) {
        match op {
            LocalOperand { name, .. } => {
                let op_name = name;
                match val {
                    LocalOperand { name, .. } => self
                        .vars
                        .insert(op_name.clone(), self.vars.get(name).unwrap().clone()),
                    ConstantOperand(..) => self.vars.insert(name.clone(), val.clone()),
                    MetadataOperand => todo!(),
                };
            }
            ConstantOperand(_) => todo!(),
            MetadataOperand => todo!(),
        }
    }

    fn load(&mut self, op: &Operand, dest: &name::Name) {
        match op {
            LocalOperand { name, .. } => self.load_loc_op(name, dest),
            ConstantOperand(_) => todo!(),
            MetadataOperand => todo!(),
        }
    }

    fn load_loc_op(&mut self, name: &name::Name, dest: &name::Name) {
        self.vars
            .insert(dest.clone(), self.vars.get(name).unwrap().clone());
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
            LocalOperand { name, ty } => {
                let op = self.vars.get(name).unwrap();
                match ty.as_ref() {
                    Type::IntegerType { bits } => match bits {
                        8 => todo!(),
                        16 => todo!(),
                        32 => (self.get_int_op(op) as i32).to_string(),
                        64 => (self.get_int_op(op) as i64).to_string(),
                        128 => todo!(),
                        _ => todo!(),
                    },
                    Type::FPType(fptype) => match fptype {
                        FPType::Single => self.get_single_fl_op(op).to_string(),
                        FPType::Double => self.get_double_fl_op(op).to_string(),
                        _ => todo!(),
                    },
                    _ => todo!(),
                }
            }
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

    fn int_bin_operations(
        &mut self,
        op0: &Operand,
        op1: &Operand,
        dest: &name::Name,
        operation_type: BinOps,
    ) {
        match op0 {
            LocalOperand { name, ty } => match ty.as_ref() {
                Type::IntegerType { bits } => match bits {
                    8 => todo!(),
                    16 => todo!(),
                    32 => {
                        let op0 = self.get_int_op(self.vars.get(name).unwrap()) as i32;
                        let op1 = self.get_int_op(op1) as i32;
                        self.int_bin_operation(op0, op1, dest, operation_type)
                    }
                    64 => {
                        let op0 = self.get_int_op(self.vars.get(name).unwrap()) as i64;
                        let op1 = self.get_int_op(op1) as i64;
                        self.int_bin_operation(op0, op1, dest, operation_type)
                    }
                    128 => todo!(),
                    _ => todo!(),
                },
                _ => todo!(),
            },
            ConstantOperand(con_op) => match con_op.as_ref() {
                Constant::Int { .. } => todo!(),
                Constant::Vector(_) => todo!(),
                _ => todo!(),
            },
            MetadataOperand => todo!(),
        }
    }

    fn get_int_op(&self, op: &Operand) -> u64 {
        match op {
            LocalOperand { name, .. } => self.get_int_op(self.vars.get(name).unwrap()),
            ConstantOperand(con_op) => match con_op.as_ref() {
                Constant::Int { value, .. } => *value,
                Constant::Undef(..) => panic!(),
                _ => unreachable!(),
            },
            MetadataOperand => todo!(),
        }
    }

    fn int_bin_operation<
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Rem<Output = T>
            + num::cast::AsPrimitive<u64>,
    >(
        &mut self,
        op0: T,
        op1: T,
        dest: &name::Name,
        operation_type: BinOps,
    ) {
        let val;
        match operation_type {
            BinOps::Add => val = op0 + op1,
            BinOps::Sub => val = op0 - op1,
            BinOps::Mul => val = op0 * op1,
            BinOps::Div => val = op0 / op1,
            BinOps::Rem => val = op0 % op1,
        };
        let val = val.as_();
        let bytes: u32 = match mem::size_of::<T>().try_into() {
            Ok(by) => by,
            Err(_) => unreachable!(),
        };

        let constant = Constant::Int {
            bits: bytes * 8,
            value: val,
        };
        self.vars
            .insert(dest.clone(), ConstantOperand(ConstantRef::new(constant)));
    }

    fn fl_bin_operations(
        &mut self,
        op0: &Operand,
        op1: &Operand,
        dest: &name::Name,
        operation_type: BinOps,
    ) {
        match op0 {
            LocalOperand { name, ty } => match ty.as_ref() {
                Type::FPType(fptype) => match fptype {
                    FPType::Single => {
                        let op0 = self.get_single_fl_op(self.vars.get(name).unwrap());
                        let op1 = self.get_single_fl_op(op1);
                        let val = self.fl_bin_operation(op0, op1, operation_type);
                        self.vars.insert(
                            dest.clone(),
                            ConstantOperand(ConstantRef::new(Constant::Float(Float::Single(val)))),
                        );
                    }
                    FPType::Double => {
                        let op0 = self.get_double_fl_op(self.vars.get(name).unwrap());
                        let op1 = self.get_double_fl_op(op1);
                        let val = self.fl_bin_operation(op0, op1, operation_type);
                        self.vars.insert(
                            dest.clone(),
                            ConstantOperand(ConstantRef::new(Constant::Float(Float::Double(val)))),
                        );
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            },
            ConstantOperand(..) => todo!(),
            MetadataOperand => todo!(),
        }
    }

    fn get_single_fl_op(&self, op: &Operand) -> f32 {
        match op {
            LocalOperand { name, .. } => self.get_single_fl_op(self.vars.get(name).unwrap()),
            ConstantOperand(con_op) => match con_op.as_ref() {
                Constant::Vector(_) => todo!(),
                Constant::Float(float) => match float {
                    Float::Single(val) => *val,
                    _ => todo!(),
                },
                Constant::Undef(..) => panic!(),
                _ => todo!(),
            },
            MetadataOperand => todo!(),
        }
    }

    fn get_double_fl_op(&self, op: &Operand) -> f64 {
        match op {
            LocalOperand { name, .. } => self.get_double_fl_op(self.vars.get(name).unwrap()),
            ConstantOperand(con_op) => match con_op.as_ref() {
                Constant::Vector(_) => todo!(),
                Constant::Float(float) => match float {
                    Float::Double(val) => *val,
                    _ => todo!(),
                },
                Constant::Undef(..) => panic!(),
                _ => todo!(),
            },
            MetadataOperand => todo!(),
        }
    }

    fn fl_bin_operation<
        T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Rem<Output = T>,
    >(
        &mut self,
        op0: T,
        op1: T,
        operation_type: BinOps,
    ) -> T {
        let val;
        match operation_type {
            BinOps::Add => val = op0 + op1,
            BinOps::Sub => val = op0 - op1,
            BinOps::Mul => val = op0 * op1,
            BinOps::Div => val = op0 / op1,
            BinOps::Rem => todo!(),
        };
        val
    }

    fn fpext(&mut self, op: &Operand, to_type: &TypeRef, dest: &name::Name) {
        match op {
            LocalOperand { name, ty } => match ty.as_ref() {
                Type::FPType(fptype) => match fptype {
                    FPType::Single => match to_type.as_ref() {
                        Type::FPType(fptype) => {
                            match fptype {
                                FPType::Double => self.vars.insert(
                                    dest.clone(),
                                    ConstantOperand(ConstantRef::new(Constant::Float(
                                        Float::Double(
                                            self.get_single_fl_op(self.vars.get(name).unwrap())
                                                as f64,
                                        ),
                                    ))),
                                ),
                                _ => todo!(),
                            };
                        }
                        _ => todo!(),
                    },
                    _ => todo!(),
                },
                _ => todo!(),
            },
            ConstantOperand(_) => todo!(),
            MetadataOperand => todo!(),
        }
    }

    fn sext(&mut self, op: &Operand, to_type: &TypeRef, dest: &name::Name) {
        match op {
            LocalOperand { name, ty } => match ty.as_ref() {
                Type::IntegerType { bits } => match bits {
                    32 => match to_type.as_ref() {
                        Type::IntegerType { bits } => {
                            match bits {
                                64 => {
                                    let constant = Constant::Int {
                                        bits: 64,
                                        value: self.get_int_op(self.vars.get(name).unwrap()) as i32
                                            as i64
                                            as u64,
                                    };
                                    self.vars.insert(
                                        dest.clone(),
                                        ConstantOperand(ConstantRef::new(constant)),
                                    );
                                }
                                _ => todo!(),
                            };
                        }
                        _ => todo!(),
                    },
                    _ => todo!(),
                },
                _ => todo!(),
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
