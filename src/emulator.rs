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
    imm:     i16, // 20 - 31
}

impl From<u32> for IType {
    fn from(ins: u32) -> Self {
        IType {
            rd:     ((ins >> 7)  & 0b11111) as u8,
            funct3: ((ins >> 12) & 0b111) as u8,
            rs1:    ((ins >> 15) & 0b11111) as u8,
            imm:    sign_extend((ins >> 20) as u64, 12) as i16,
        }
    }
}

#[derive(Debug)]
struct SType {  // bits:
    funct3: u8, // 12 - 14
    rs1:    u8, // 15 - 19
    rs2:    u8, // 20 - 24
    imm:   i16, //  7 - 11 + 25 - 31 
}

impl From<u32> for SType {
    fn from(ins: u32) -> Self {

        let imm40 = ((ins >> 7)  & 0b11111) as u8;
        let imm115 = (ins >> 25) as u8;
        let imm_final = ((imm115 as u16) << 5) | imm40 as u16;
        let imm_final = sign_extend(imm_final as u64, 12) as i16;

        SType {
            funct3: ((ins >> 12) & 0b111) as u8,
            rs1:    ((ins >> 15) & 0b11111) as u8,
            rs2:    ((ins >> 20) & 0b11111) as u8,
            imm:    imm_final,
        }
    }
}

#[derive(Debug)]
struct BType {    // bits
    imm:     i32, //  7 - 11
    funct3:   u8, // 12 - 14
    rs1:      u8, // 15 - 19
    rs2:      u8, // 20 - 24
}

impl From<u32> for BType {
    fn from(ins: u32) -> Self {

        let imm41 = ((ins >> 7)  & 0b1111);
        let imm105 = ((ins >> 25)  & 0b111111);
        let imm11 = ((ins >> 11)  & 0b1);
        let imm12 = (ins >> 31) & 0b1;
        let imm_final = (imm12 << 12) | (imm11 << 11) | (imm105 << 5) | (imm41 << 1);
        let imm_final = ((imm_final as i32) << 19) >> 19;

        BType {
            imm:    imm_final,
            funct3: ((ins >> 12) & 0b111) as u8,
            rs1:    ((ins >> 15) & 0b11111) as u8,
            rs2:    ((ins >> 20) & 0b11111) as u8,
        }
    }
}
 
#[derive(Debug)]       
struct UType { // bits
    rd:    u8, //  7 - 11
    imm:  i32, // 12 - 31
}

impl From<u32> for UType {
    fn from(ins: u32) -> Self {
        UType {
            rd: ((ins >> 7)  & 0b11111) as u8,
            imm: (ins & 0xFFFFF000) as i32,
        }
    }
}

#[derive(Debug)]
struct JType { // bits
    rd:   u8,  //  7 - 11
    imm: i32,  // 12 - 31
}

impl From<u32> for JType {
    fn from(ins: u32) -> Self {

        let imm20   = ((ins >> 31) & 0x1) << 20;
        let imm101 = ((ins >> 21) & 0x3FF) << 1;
        let imm11   = ((ins >> 20) & 0x1) << 11;
        let imm1912= ((ins >> 12) & 0xFF) << 12;
        let imm_final = sign_extend((imm20 | imm1912 | imm11 | imm101) as u64, 21) as i32;

        JType { 
            rd: ((ins >> 7)  & 0b11111) as u8,
            imm: imm_final,
        }
    }
}


/// sign extend an int to a i64
fn sign_extend(val: u64, len: u32) -> i64 {
    assert!(len <= 32 && len > 0);
    let shift_amount = 64 - len;
    ((val << shift_amount) as i64) >> shift_amount
}


#[derive(Debug)]
pub struct Interpreter {
    registers: Vec<u64>,
    pc: u64,
    mem: mmu::Memory,
}

impl Interpreter {
    /// Constructor
    pub fn new() -> Self {
        Interpreter {
            /// 32 x-registers
            registers: vec![0; 32],

            /// Program counter
            pc: 0,

            ///Virual memory space
            mem: mmu::Memory::new(4 * 1024 * 1024),
        }
    }

    /// executes a single instruction 
    pub fn execute_instruction(&mut self, ins: u32) -> u8 {
        let opcode = ins & 0b1111111;

        match opcode {

            // LUI - UType
            0b0110111 => {
                let ins = UType::from(ins);
                println!("{:?}", ins);
                self.registers[ins.rd as usize] = ins.imm as u64;
            },

            // AUIPC - UType
            0b0010111 => {
                let ins = UType::from(ins);
                println!("{:?}", ins);
                self.registers[ins.rd as usize] = self.pc.wrapping_add(
                    (ins.imm << 12) as u64
                );
            },
            
            // JAL - JType
            0b1101111 => {
                let ins = JType::from(ins);
                println!("{:?}", ins);
                self.registers[ins.rd as usize] = self.pc.wrapping_add(4);
                self.pc = self.pc.wrapping_add(ins.imm as u64);
            },

            // JALR - IType
            0b1100111 => {
                let ins = IType::from(ins);
                println!("{:?}", ins);
                self.registers[ins.rd as usize] = self.pc.wrapping_add(4);
                self.pc = self.registers[ins.rs1 as usize].wrapping_add(
                    ins.imm as u64
                ) & !1;
            },

            // BEQ/BNE/BLT/BGE/BLTU/BGEU - BType
            0b1100011 => {
                let ins = BType::from(ins);
                println!("{:?}", ins);

                match ins.funct3 {

                    0b000 => { // BEQ
                        if self.registers[ins.rs1 as usize] == self.registers[ins.rs2 as usize] {
                            self.pc = self.pc.wrapping_add(ins.imm as u64);
                        }
                    },

                    0b001 => { // BNE
                        if self.registers[ins.rs1 as usize] != self.registers[ins.rs2 as usize] {
                            self.pc = self.pc.wrapping_add(ins.imm as u64);
                        }
                    },

                    0b100 => { // BLT
                        if (self.registers[ins.rs1 as usize] as i64) < (self.registers[ins.rs2 as usize] as i64) {
                            self.pc = self.pc.wrapping_add(ins.imm as u64);
                        }
                    },

                    0b101 => { // BGE
                        if (self.registers[ins.rs1 as usize] as i64) >= (self.registers[ins.rs2 as usize] as i64) {
                            self.pc = self.pc.wrapping_add(ins.imm as u64);
                        }
                    },

                    0b110 => { // BLTU
                        if self.registers[ins.rs1 as usize] < self.registers[ins.rs2 as usize] {
                            self.pc = self.pc.wrapping_add(ins.imm as u64);
                        }
                    },

                    0b111 => { // BGEU
                        if self.registers[ins.rs1 as usize] >= self.registers[ins.rs2 as usize] {
                            self.pc = self.pc.wrapping_add(ins.imm as u64);
                        }
                    },

                    _ => panic!("at the disco"),
                }
            },

            // LB/LH/LW/LBU/LHU - IType
            0b0000011 => {
                let ins = IType::from(ins);
                println!("{:?}", ins);
            },

            // SB/SH/SW - SType
            0b0100011 => {
                let ins = SType::from(ins);
                println!("{:?}", ins);
            },

            // ADDI/SLTI/SLTIU/XORI/ORI/ANDI/SLLI/SRLI/SRAI- IType
            0b0010011 => {
                let ins = IType::from(ins);
                println!("{:?}", ins);
            },
        
            // ADD/SUB/SLL/SLT/SLTU/XOR/SRL/SRA/OR/AND - RType
            0b0110011 => {
                let ins = RType::from(ins);
                println!("{:?}", ins);
            },

            // FENCE/FENCE.TSO/PAUSE - IType
            0b0001111 => {
                let ins = IType::from(ins);
                println!("{:?}", ins);
            },

            // ECALL/EBREAK - IType
            0b1110011 => {
                let ins = IType::from(ins);
                println!("{:?}", ins);
            },
            
            // Catch all
            _ => panic!("**SEGFAULT** Bad instruction"),

        }

        0
    }

}




