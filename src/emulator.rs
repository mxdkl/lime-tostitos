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
        rd:     ((ins >> 7)  & 0b11111) as u8,
        funct3: ((ins >> 12) & 0b111) as u8,
        rs1:    ((ins >> 15) & 0b11111) as u8,
        rs2:    ((ins >> 20) & 0b11111) as u8,
        funct7:  (ins >> 25) as u8,
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
        rd:     ((ins >> 7)  & 0b11111) as u8,
        funct3: ((ins >> 12) & 0b111) as u8,
        rs1:    ((ins >> 15) & 0b11111) as u8,
        imm110:  (ins >> 20) as u16,
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
        imm40:  ((ins >> 7)  & 0b11111) as u8,
        funct3: ((ins >> 12) & 0b111) as u8,
        rs1:    ((ins >> 15) & 0b11111) as u8,
        rs2:    ((ins >> 20) & 0b11111) as u8,
        imm115:  (ins >> 25) as u8,
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
        imm4111:   ((ins >> 7)  & 0b11111) as u8,
        funct3:    ((ins >> 12) & 0b111) as u8,
        rs1:       ((ins >> 15) & 0b11111) as u8,
        rs2:       ((ins >> 20) & 0b11111) as u8,
        imm12105:   (ins >> 25) as u8,
    }
}
 
#[derive(Debug)]       
struct UType {    // bits
    rd:      u8,  //  7 - 11
    imm3112: u32, // 12 - 31
}

impl From<u32> for UType {
    fn from(ins: 32) -> Self {
        rd:      ((ins >> 7) & 0b11111) as u8,
        imm3112:  (ins >> 12) as u32,
    }
}

#[derive(Debug)]
struct JType {          // bits
    rd:             u8, //  7 - 11
    imm20101111912: u8, // 12 - 31
}

impl From<u32> for JType {
    fn from(ins: 32) -> Self {
        rd:             ((ins >> 7) & 0b11111) as u8,
        imm20101111912:  (ins >> 12) as u32,
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
            /// 32 registers, 1 program counter
            registers: vec![0; 33],

            ///Virual memory space
            mem: mmu::Memory::new(4 * 1024 * 1024),
        }
    }

    
    // Private functions
    
    fn instructionDecode(self, ins: u32) -> Some() {
        let opcode = ins & 0b111111;
        
    }

}





