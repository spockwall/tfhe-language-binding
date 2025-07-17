use crate::configs::errors::AsmError;

use crate::configs::instructions::{Instruction, InstructionLine};

/// Parse the assembly code from input string
/// to a vector of InstructionLine
///
/// Example:
/// ```no_run
/// input_asm = "VAR a r0\nVAR b r1\nMOV 123 r2\nMOV 456 r3\nNEG r3 r4\nADD r2 r4 r5\nOUT r5"
/// output_asm = [
///      InstructionLine { { instruction: VAR, operands: ["a"], register: "r0" },
///      InstructionLine { instruction: VAR, operands: ["b"], register: "r1" },
///      InstructionLine { instruction: MOV, operands: ["123"], register: "r2" },
///      InstructionLine { instruction: MOV, operands: ["456"], register: "r3" },
///      InstructionLine { instruction: NEG, operands: ["r3"], register: "r4" },
///      InstructionLine { instruction: ADD, operands: ["r2", "r4"], register: "r5" },
///      InstructionLine { instruction: OUT, operands: [], register: "r5" }
/// ]
pub fn parse_asm_from_str(s: &str) -> Result<Vec<InstructionLine>, AsmError> {
    let inst_lines: Vec<InstructionLine> = s
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let instruction = parts.next().unwrap();
            let operands: Vec<String> = parts.map(|s| s.to_string()).collect();
            let register = operands.last().unwrap().to_string();
            let operands = operands[..operands.len() - 1].to_vec();
            InstructionLine {
                instruction: Instruction::from_str(instruction).unwrap(),
                operands,
                register,
            }
        })
        .collect();

    for inst_line in inst_lines.iter() {
        if !validate_instruction_line(inst_line) {
            return Err(AsmError::InvalidInstruction(inst_line.to_string()));
        }
    }
    Ok(inst_lines)
}

/// Validate the instruction line, check if the number
/// of operands is correct for the instruction
#[rustfmt::skip]
pub fn validate_instruction_line(inst_line: &InstructionLine) -> bool {
    match inst_line.instruction {
        Instruction::OUT => return inst_line.operands.len() == 0,
        
        Instruction::VAR 
        | Instruction::MOV 
        | Instruction::NEG 
        | Instruction::NOT => {
            return inst_line.operands.len() == 1;
        }

        Instruction::ADD
        | Instruction::SUB
        | Instruction::MUL
        | Instruction::DIV
        | Instruction::REM
        | Instruction::AND
        | Instruction::OR
        | Instruction::XOR
        | Instruction::SHR
        | Instruction::SHL => return inst_line.operands.len() == 2,
    }
}
