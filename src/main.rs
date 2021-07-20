mod interp;
use interp::LLVMIRInterpreter;

use llvm_ir::Module;
use std::env;

fn main() {
    let mut lii = LLVMIRInterpreter::new();
    let args: Vec<String> = env::args().collect();
    match create_module(&args[1]) {
        Ok(module) => match lii.interpret(module) {
            Ok(_) => {}
            Err(str) => println!("{}", str),
        },
        Err(error_message) => println!("{}", error_message),
    };
}

fn create_module(path_s: &str) -> Result<Module, String> {
    Module::from_bc_path(path_s)
}
