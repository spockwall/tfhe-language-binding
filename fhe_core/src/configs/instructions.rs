#[derive(Debug)]
pub struct InstructionLine {
    pub instruction: Instruction,
    pub operands: Vec<String>,
    pub register: String,
}

impl InstructionLine {
    pub fn to_string(&self) -> String {
        let operands = self.operands.join(" ");
        format!(
            "{} {} {}",
            self.instruction.as_str(),
            operands,
            self.register
        )
    }
}

// define instruction
#[derive(Debug)]
pub enum Instruction {
    VAR,
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    REM,
    SHR,
    SHL,
    AND,
    OR,
    XOR,
    NOT,
    NEG,
    OUT,
}

impl Instruction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "VAR" => Some(Self::VAR),
            "MOV" => Some(Self::MOV),
            "ADD" => Some(Self::ADD),
            "SUB" => Some(Self::SUB),
            "MUL" => Some(Self::MUL),
            "DIV" => Some(Self::DIV),
            "REM" => Some(Self::REM),
            "SHR" => Some(Self::SHR),
            "SHL" => Some(Self::SHL),
            "AND" => Some(Self::AND),
            "OR" => Some(Self::OR),
            "XOR" => Some(Self::XOR),
            "NOT" => Some(Self::NOT),
            "NEG" => Some(Self::NEG),
            "OUT" => Some(Self::OUT),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::VAR => "VAR",
            Self::MOV => "MOV",
            Self::ADD => "ADD",
            Self::SUB => "SUB",
            Self::MUL => "MUL",
            Self::DIV => "DIV",
            Self::REM => "REM",
            Self::SHR => "SHR",
            Self::SHL => "SHL",
            Self::AND => "AND",
            Self::OR => "OR",
            Self::XOR => "XOR",
            Self::NOT => "NOT",
            Self::NEG => "NEG",
            Self::OUT => "OUT",
        }
    }
}
