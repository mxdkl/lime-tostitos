use crate::mmu;


#[derive(Debug)]
struct RType {
    rd:     u8,
    funct3: u8,
    rs1:    u8,
    rs2:    u8,
    funct7: u8,
}

impl From<u32> for Rtype {
    fn from(ins: u32) -> Self {
        
    }
}

#[derive(Debug)]
struct IType {
    rd:     u8,
    funct3: u8,
    rs1:    u8,
    imm110  u16,
}

impl From<u32> for Itype {
}

#[derive(Debug)]
struct SType {
    imm40:  u8,
    funct3: u8,
    rs1:    u8,
    rs2:    u8,
    imm115: u8,

}

impl From<u32> for Stype {
}

#[derive(Debug)]
struct BType {
    rd:     u8,
    funct3: u8,
    rs1:    u8,
    rs2:    u8,
    funct7: u8,

}

impl From<u32> for Btype {
}
 
#[derive(Debug)]       
struct UType {
    rd:     u8,
    funct3: u8,
    rs1:    u8,
    rs2:    u8,
    funct7: u8,

}

impl From<u32> for Utype {
}

#[derive(Debug)]
struct JType {
    rd:     u8,
    funct3: u8,
    rs1:    u8,
    rs2:    u8,
    funct7: u8,

}

impl From<u32> for Jtype {
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

        // LUI
        if opcode == 0b0110111 {
            self.lui(ins)
        }
        

    }

}





