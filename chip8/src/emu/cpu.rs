// Contains the CPUs Registers, OpCodes, and their impls.
use crate::emu::{
    iset::{Nibbles, OpCode},
    mem::Memory,
};

/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
use color_eyre::Result;

#[derive(Debug)]
#[allow(dead_code)] // REMOVE THIS WHEN DONE
pub struct Cpu {
    pub current_opcode: OpCode,
    // memory: [u8; 4096],
    pub registers: [u8; 16], // general purpose
    /// Address of the current instruction
    pub index_register: u16, // can only load 12-bit mem address due to range of mem accessible
    //                      1111 1111 1111 -> 0xFFF -> 4095 -> memsize
    pub program_counter: u16,

    // pub screen: [bool; 64 * 32],
    // pub stack: [u16; 16],
    pub stack_pointer: usize,

    pub delay_timer: u8,
    pub sound_timer: u8,

    pub keypad: [bool; 16],
    //pub rom_buffer: Vec<u8>, // moved to self.memory.rom
    pub running: bool,
    pub memory: Memory,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            current_opcode: OpCode(0),
            // memory: [0; 4096], // moved into 'memory' as 'ram'
            registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            // screen: [false; 64 * 32],
            // stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            //rom_buffer: Vec::new(), // Moved into 'memory' as 'rom'
            running: false,
            memory,
        }
    }

    //pub fn memory(&mut self) -> &mut Memory {
    //    &mut self.mem;
    //}

    /// Map the current OpCode to an actual function.
    pub fn process(&mut self) -> Result<()> {
        // DECODE and Process
        match &self.current_opcode.into_tuple() {
            (0, 0, 0xE, 0xE) => OpCode::_00ee(self),
            (0, 0, 0xE, 0) => OpCode::_00e0(&mut self.memory.gpu),
            (0, _, _, _) => OpCode::_0nnn(self),
            (1, _, _, _) => OpCode::_1nnn(self),
            (2, _, _, _) => OpCode::_2nnn(self),
            (3, _, _, _) => OpCode::_3xnn(self),
            (4, _, _, _) => OpCode::_4xnn(self),
            (5, _, _, 0) => OpCode::_5xy0(self),
            (6, _, _, _) => OpCode::_6xnn(self),
            (7, _, _, _) => OpCode::_7xnn(self),
            (8, _, _, 0) => OpCode::_8xy0(self),
            (8, _, _, 1) => OpCode::_8xy1(self),
            (8, _, _, 2) => OpCode::_8xy2(self),
            (8, _, _, 3) => OpCode::_8xy3(self),
            (8, _, _, 4) => OpCode::_8xy4(self),
            (8, _, _, 5) => OpCode::_8xy5(self),
            (8, _, _, 6) => OpCode::_8xy6(self),
            (8, _, _, 7) => OpCode::_8xy7(self),
            (8, _, _, 0xE) => OpCode::_8xye(self),
            (9, _, _, 0) => OpCode::_9xy0(self),
            (0xA, _, _, _) => OpCode::annn(self),
            (0xB, _, _, _) => OpCode::bnnn(self),
            (0xC, _, _, _) => OpCode::cxnn(self),
            (0xD, _, _, _) => OpCode::dxyn(self),
            (0xE, _, 9, 0xE) => OpCode::ex9e(self),
            (0xE, _, 0xA, 1) => OpCode::exa1(self),
            (0xF, _, 0, 7) => OpCode::fx07(self),
            (0xF, _, 0, 0xA) => OpCode::fx0a(self),
            (0xF, _, 1, 5) => OpCode::fx15(self),
            (0xF, _, 1, 8) => OpCode::fx18(self),
            (0xF, _, 1, 0xE) => OpCode::fx1e(self),
            (0xF, _, 2, 9) => OpCode::fx29(self),
            (0xF, _, 3, 3) => OpCode::fx33(self),
            (0xF, _, 5, 5) => OpCode::fx55(self),
            (0xF, _, 6, 5) => OpCode::fx65(self),
            (a, b, c, d) => println!("Not implemented {:x?}", (a, b, c, d)),
        }
        Ok(())
    }

    /// Retrieves opcode from memory into the cpu
    pub fn fetch_opcode(&mut self) -> Result<bool, bool> {
        let opcode_high: u8 = self.memory.ram[self.program_counter as usize];
        let opcode_low: u8 = self.memory.ram[self.program_counter as usize + 1];
        let opcode: u16 = (opcode_high as u16) << 8 | opcode_low as u16;
        self.current_opcode = OpCode(opcode);
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::emu::cpu::Cpu;
    use crate::emu::cpu::OpCode;
    use crate::emu::Memory;

    fn test_init_cpu() -> Cpu {
        let mut mem = Memory::new();

        // create some fake memory
        mem.ram[0x200] = 1;
        mem.ram[0x201] = 2;
        mem.ram[0x202] = 3;
        mem.ram[0x203] = 4;
        mem.ram[0x204] = 5;

        let mut cpu = Cpu::new(mem);

        // random registers populated
        cpu.registers[0] = 105;
        cpu.registers[1] = 5;
        cpu.registers[2] = 14;
        cpu.registers[7] = 33;
        cpu.registers[12] = 0x11;
        cpu.index_register = 0x200;

        cpu
    }

    #[test]
    fn test_fx55() {
        let mut cpu = test_init_cpu();
        cpu.current_opcode = OpCode(0xF555);
        let mem = cpu.mem();

        // memory should be 1-7 at 0x200-206
        mem.data[0x205] = 6;
        mem.data[0x206] = 7;

        assert_eq!(mem.data[0x200], 1);
        assert_eq!(mem.data[0x201], 2);
        assert_eq!(mem.data[0x202], 3);
        assert_eq!(mem.data[0x203], 4);
        assert_eq!(mem.data[0x204], 5);
        assert_eq!(mem.data[0x205], 6);
        assert_eq!(mem.data[0x206], 7);

        OpCode::fx55(&mut cpu, &mut mem);

        // our x was 5, v0..vx needs to get set with I..I+x
        assert_eq!(mem.ram[0x200], cpu.registers[0]);
        assert_eq!(mem.ram[0x201], cpu.registers[1]);
        assert_eq!(mem.ram[0x202], cpu.registers[2]);
        assert_eq!(mem.ram[0x203], cpu.registers[3]);
        assert_eq!(mem.ram[0x204], cpu.registers[4]);
        assert_eq!(mem.ram[0x205], cpu.registers[5]);

        // this next memory address shouldnt have been affected by 0xF555 b/c x=5
        assert_ne!(mem.ram[0x206], cpu.registers[6]);
        assert_eq!(mem.ram[0x206], 7);
    }

    #[test]
    fn test_fx65() {
        let (mut cpu, mut mem) = test_init_cpu();
        cpu.current_opcode = OpCode(0xF565);

        // setting up data to check for out of bounds bugs
        (mem.data[0x206], cpu.registers[6]) = (0xDE, 0xAD);

        OpCode::fx65(&mut cpu);
        // our x was 5, v0..vx needs to get set with I..I+x
        assert_eq!(mem.data[0x200], cpu.registers[0]);
        assert_eq!(mem.data[0x201], cpu.registers[1]);
        assert_eq!(mem.data[0x202], cpu.registers[2]);
        assert_eq!(mem.data[0x203], cpu.registers[3]);
        assert_eq!(mem.data[0x204], cpu.registers[4]);
        assert_eq!(mem.data[0x205], cpu.registers[5]);
        assert_ne!(mem.data[0x206], cpu.registers[6]);
    }

    #[test]
    fn test_fx33() {
        let mut emu = Cpu::new();
        cpu.current_opcode = OpCode(0xF533);
        cpu.registers[5] = 105;
        cpu.index_register = 0x200; // unnecessary but oh well...

        // Test init wierd mishaps
        assert_eq!(mem.data[cpu.index_register as usize], 0);
        assert_eq!(mem.data[(cpu.index_register + 1) as usize], 0);
        assert_eq!(mem.data[(cpu.index_register + 2) as usize], 0);
        println!("index_register: {:?}", cpu.index_register);
        let idxr: usize = cpu.index_register as usize;
        println!(
            "memory.data[ir..ir+3]: {:x?}",
            &mem.data[(idxr)..(idxr + 3)]
        );

        // Test fx33
        OpCode::fx33(&mut emu);
        println!(
            "memory.data[ir..ir+3]: {:x?}",
            &mem.data[(idxr)..(idxr + 3)]
        );

        assert_eq!(mem.data[cpu.index_register as usize], 1);
        assert_eq!(mem.data[(cpu.index_register + 1) as usize], 0);
        assert_eq!(mem.data[(cpu.index_register + 2) as usize], 5);

        //println!("{:x?}", &emu.memory);
        //assert_eq!(mem.data[(cpu.index_register + 2) as usize], 8);
    }

    #[test]
    fn test_fx1e() {
        // init
        let mut emu = Cpu::new();
        cpu.index_register = 0x200; // unnecessary but oh well...

        // save before
        cpu.current_opcode = OpCode(0xF51E);
        let old_i = cpu.index_register.clone();
        println!("old_i: {:?}", old_i);

        // test fx1e to see if vX = 0 works
        // b/c x=5 -> v[5] and b/c all registers are 0'd out now -> 0
        OpCode::fx1e(&mut cpu); // add 5 to i
        assert_eq!(cpu.index_register, old_i);

        cpu.registers[5] = 3;
        OpCode::fx1e(&mut cpu); // add 5 to i
        assert_eq!(cpu.index_register, old_i + 3);
    }
}

/* Chip8 Memory layout
0x000-0x04F - Chip 8 interpreter (contains font set in emu)       0 -   79
0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)    080 -  160
0x200-0xFFF - Program ROM and work RAM                          512 - 4096

0x200-0xE8F
"final 352 bytes of memory are reserved for “variables and display refresh"
https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#registers
*/
