use crate::mmu;


#[derive(Debug)]
struct RType {  // bits:
    rd:     u8, //  7 - 11
    funct3: u8, // 12 - 14
    rs1:    u8, // 15 - 19
    rs2:    u8, // 20 - 24
    funct7: u8, // 25 - 31
}

impl From<u32> for RType {
    fn from(ins: u32) -> Self {
        RType {
            rd:     ((ins >> 7)  & 0b11111) as u8,
            funct3: ((ins >> 12) & 0b111) as u8,
            rs1:    ((ins >> 15) & 0b11111) as u8,
            rs2:    ((ins >> 20) & 0b11111) as u8,
            funct7:  (ins >> 25) as u8,
        }
    }
}

#[derive(Debug)]
struct IType {    // bits: 
    rd:       u8, //  7 - 11 
    funct3:   u8, // 12 - 14
    rs1:      u8, // 15 - 19
    imm110:  u16, // 20 - 31
}

impl From<u32> for IType {
    fn from(ins: u32) -> Self {
        IType {
            rd:     ((ins >> 7)  & 0b11111) as u8,
            funct3: ((ins >> 12) & 0b111) as u8,
            rs1:    ((ins >> 15) & 0b11111) as u8,
            imm110:  (ins >> 20) as u16,
        }
    }
}

#[derive(Debug)]
struct SType {  // bits:
    imm40:  u8, //  7 - 11
    funct3: u8, // 12 - 14
    rs1:    u8, // 15 - 19
    rs2:    u8, // 20 - 24
    imm115: u8, // 25 - 31
}

impl From<u32> for SType {
    fn from(ins: u32) -> Self {
        SType {
            imm40:  ((ins >> 7)  & 0b11111) as u8,
            funct3: ((ins >> 12) & 0b111) as u8,
            rs1:    ((ins >> 15) & 0b11111) as u8,
            rs2:    ((ins >> 20) & 0b11111) as u8,
            imm115:  (ins >> 25) as u8,
        }
    }
}

#[derive(Debug)]
struct BType {    // bits
    imm4111:  u8, //  7 - 11
    funct3:   u8, // 12 - 14
    rs1:      u8, // 15 - 19
    rs2:      u8, // 20 - 24
    imm12105: u8, // 25 - 31
}

impl From<u32> for BType {
    fn from(ins: u32) -> Self {
        BType {
            imm4111:   ((ins >> 7)  & 0b11111) as u8,
            funct3:    ((ins >> 12) & 0b111) as u8,
            rs1:       ((ins >> 15) & 0b11111) as u8,
            rs2:       ((ins >> 20) & 0b11111) as u8,
            imm12105:   (ins >> 25) as u8,
        }
    }
}
 
#[derive(Debug)]       
struct UType {    // bits
    rd:      u8,  //  7 - 11
    imm3112: u32, // 12 - 31
}

impl From<u32> for UType {
    fn from(ins: u32) -> Self {
        UType {
            rd:      ((ins >> 7)  & 0b11111) as u8,
            imm3112:  (ins >> 12) as u32,
        }
    }
}

#[derive(Debug)]
struct JType {           // bits
    rd:             u8,  //  7 - 11
    imm20101111912: u32, // 12 - 31
}

impl From<u32> for JType {
    fn from(ins: u32) -> Self {
        JType { 
            rd:             ((ins >> 7)  & 0b11111) as u8,
            imm20101111912:  (ins >> 12) as u32,
        }
    }
}


#[derive(Debug)]
pub struct Interpreter {
    registers: Vec<u64>,
    mem: mmu::Memory,
}

impl Interpreter {
    // Public functions
    
    /// Constructor
    pub fn new() -> Self {
        Interpreter {
            /// 32 x-registers, 1 program counter
            registers: vec![0; 33],

            ///Virual memory space
            mem: mmu::Memory::new(4 * 1024 * 1024),
        }
    }

    
    // Private functions
    
    pub fn execute_instruction(mut self, ins: u32) -> u8 {
        let opcode = ins & 0b111111;

        match opcode {

            // LUI - UType
            0b0110111 => {
                let ins = UType::from(ins);
                self.registers[ins.rd as usize] = ins.imm3112 as u64;
            },

            // AUIPC - UType
            0b0010111 => {
                let ins = UType::from(ins);
            },
            
            // JAL - JType
            0b1101111 => {
                let ins = JType::from(ins);
            },

            // JALR - IType
            0b1100111 => {
                let ins = IType::from(ins);
            },

            // BEQ/BNE/BLT/BGE/BLTU/BGEU - BType
            0b1100011 => {
                let ins = BType::from(ins);
            },

            // LB/LH/LW/LBU/LHU - IType
            0b0000011 => {
                let ins = IType::from(ins);
            },

            // SB/SH/SW - SType
            0b0100011 => {
                let ins = SType::from(ins);
            },

            // ADDI/SLTI/SLTIU/XORI/ORI/ANDI/SLLI/SRLI/SRAI- IType
            0b0010011 => {
                let ins = IType::from(ins);
            },
        
            // ADD/SUB/SLL/STL/STLU/XOR/SRL/SRA/OR/AND - IType
            0b0110011 => {
                let ins = IType::from(ins);
            },

            // FENCE/FENCE.TSO/PAUSE - IType
            0b0001111 => {
                let ins = IType::from(ins);
            },

            // ECALL/EBREAK - IType
            0b11100111 => {
                let ins = IType::from(ins);
            },

            _ => panic!(),

        }

        0
    }

}




