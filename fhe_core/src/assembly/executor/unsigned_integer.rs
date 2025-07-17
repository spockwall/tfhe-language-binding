use crate::configs::errors::AsmError;
use crate::configs::instructions::{Instruction, InstructionLine};
use crate::fhe_traits::computable::{Computable, Shiftable};
use std::collections::HashMap;
use tfhe::FheUint64;

/// macro_rules that define asm execution functions for fhe unsigned integers
///
/// Input:
///     - asm: &Vec<InstructionLine> - the assembly code to be executed
///     - args: &HashMap<String, FheUint> - the arguments of the asm code

/// Output:
///     - Result<FheUint, AsmError> - the result of the asm execution
///
/// Example:
/// ```no_run
/// impl_execute_unsigned_int!(u64, FheUint64);
/// ```
macro_rules! impl_execute_unsigned_int {
    ($t:ty, $fhe_ty:ty) => {
        paste::item! {
            pub fn [<execute_asm_ $t>](
                asm: &Vec<InstructionLine>,
                args: &HashMap<String, $fhe_ty>,
            ) -> Result<$fhe_ty, AsmError> {
                let mut register_map: HashMap<&String, $fhe_ty> = HashMap::new();

                for line in asm {
                    let operands = &line.operands;
                    let register = &line.register;
                    match line.instruction {
                        Instruction::VAR => {
                            let arg = args.get(&operands[0]).unwrap();
                            register_map.insert(register, arg.clone());
                        }

                        Instruction::NEG => {
                            let arg = register_map.get(&operands[0]).unwrap();
                            let res = arg.neg();
                            register_map.insert(register, res);
                        }

                        Instruction::NOT => {
                            let arg = register_map.get(&operands[0]).unwrap();
                            let res = arg.not();
                            register_map.insert(register, res);
                        }

                        Instruction::ADD => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.add(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::SUB => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.sub(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::MUL => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.mul(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::DIV => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.div(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::REM => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.rem(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::AND => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.and(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::OR => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.or(arg2);
                            register_map.insert(register, res);
                        }

                        Instruction::XOR => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.xor(arg2);
                            register_map.insert(register, res);
                        }
                        Instruction::SHR => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.shr(arg2);
                            register_map.insert(register, res);
                        }
                        Instruction::SHL => {
                            let arg1 = register_map.get(&operands[0]).unwrap();
                            let arg2 = register_map.get(&operands[1]).unwrap();
                            let res = arg1.shl(arg2);
                            register_map.insert(register, res);
                        }
                        Instruction::OUT => {
                            if let Some(arg) = register_map.get(register) {
                                return Ok(arg.clone());
                            } else {
                                let msg = format!("Register {} not found", register);
                                return Err(AsmError::OutputError(msg));
                            }
                        }
                        _ => {
                            let msg = format!("Invalid instruction {:?}", line.instruction);
                            return Err(AsmError::InvalidInstruction(msg));
                        }
                    }
                }
                Err(AsmError::OutputError("Output not defined".to_string()))
            }
        }
    };
}

impl_execute_unsigned_int!(u64, FheUint64);
