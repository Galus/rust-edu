// Contains the CPUs Registers, OpCodes, and their impls.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
use color_eyre::Result;

use super::{gpu::Gpu, memory::Memory};

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            current_opcode: OpCode(0),
            // memory: [0; 4096],
            registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            // screen: [false; 64 * 32],
            // stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            rom_buffer: Vec::new(),
            running: false,
        }
    }

    /// Map the current OpCode to an actual function.
    pub fn process(&mut self, mem: &mut Memory, gpu: &mut Gpu) -> Result<()> {
        // DECODE and Process
        let op = &self.current_opcode;
        match op.into_tuple() {
            (0, 0, 0xE, 0xE) => OpCode::_00ee(self),
            (0, 0, 0xE, 0) => OpCode::_00e0(gpu),
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
            (0xF, _, 3, 3) => OpCode::fx33(self, mem),
            (0xF, _, 5, 5) => OpCode::fx55(self, mem),
            (0xF, _, 6, 5) => OpCode::fx65(self, mem),
            (a, b, c, d) => println!("Not implemented {:x?}", (a, b, c, d)),
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct OpCode(pub u16);
impl OpCode {
    /// Fill registers v0 to vX inclusive.
    /// Sets I = I + X + 1
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn fx65(cpu: &mut Cpu, mem: &mut Memory) {
        let num_registers = OpCode::get_x(&cpu);
        for x in 0..=num_registers {
            let load_index = cpu.index_register + (x as u16);
            cpu.registers[x as usize] = mem.data[load_index as usize]
        }
        cpu.index_register += (num_registers + 1) as u16;
    }

    /// Store register vals v0 to vX inclusive in memory starting at address I.
    /// Sets I = I + X + 1
    /// Basically fx65 but instead of putting memory into registers, puts registers into memory.
    fn fx55(cpu: &mut Cpu, mem: &mut Memory) {
        let num_registers = OpCode::get_x(&cpu);
        for x in 0..=num_registers {
            let load_index = cpu.index_register + (x as u16);
            mem.data[load_index as usize] = cpu.registers[x as usize];
        }
        cpu.index_register += (num_registers + 1) as u16;
    }

    /// Store BCD of value in vX at addresses I, I+1, I+2
    fn fx33(cpu: &mut Cpu, mem: &mut Memory) {
        let x = OpCode::get_x(&cpu);
        let register = cpu.registers[x as usize];
        let padded = format!("{:0>3}", register);
        let a: u8 = padded.chars().nth(0).unwrap() as u8 - 48; // ascii '0' starts at decimal 48
        let b: u8 = padded.chars().nth(1).unwrap() as u8 - 48;
        let c: u8 = padded.chars().nth(2).unwrap() as u8 - 48;
        let index = cpu.index_register as usize;
        mem.data[index] = a;
        mem.data[index + 1] = b;
        mem.data[index + 2] = c;
    }

    /// Set I to memory address of the sprite data corresponding to hex digit stored in register vX
    fn fx29(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let vx = &cpu.registers[x as usize];
        cpu.index_register = *vx as u16;
    }

    /// Add the value stored in register vX to register I
    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in I.
    fn fx1e(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let vx = &cpu.registers[x as usize];
        let i = &cpu.index_register;
        let new_i = (*vx) as u16 + i;
        cpu.index_register = new_i;
    }

    /// Set the sound timer to value of register vX
    fn fx18(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let vx = cpu.registers[x as usize];
        cpu.sound_timer = vx;
    }

    /// Set the delay timer to the value of register vX
    fn fx15(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let vx = cpu.registers[x as usize];
        cpu.delay_timer = vx;
    }

    /// Wait for a keypress and store the result in register vX
    fn fx0a(_cpu: &mut Cpu) {
        todo!()
    }

    /// Store the current value of the delay timer in register vX
    fn fx07(cpu: &mut Cpu) {
        let delay_timer = emu.delay_timer;
        let x = OpCode::get_x(cpu);
        cpu.registers[x as usize] = delay_timer;
    }

    /// Skip the following instruction if the key corresponding to the hex value currently stored
    /// in register vX is NOT pressed
    fn exa1(_cpu: &mut Cpu) {
        todo!()
    }

    /// Skip the following instruction if the key corresponding to the hex value currently stored
    /// in register vX is pressed
    fn ex9e(_cpu: &mut Cpu) {
        todo!()
    }

    /// Draw a sprite at position vX, vY with N bytes of sprite data starting at the address
    /// stored in I. Set vF to 01 if any set pixels are changed to unset, and 00 otherwise.
    fn dxyn(_cpu: &mut Cpu) {
        todo!()
    }

    /// Set vX to a random number with a mask of NN
    fn cxnn(cpu: &mut Cpu) {
        let (_, x, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let rng = rand::random::<u8>();
        let masked_rng = (n2 << 4 | n3) & rng;
        cpu.registers[x as usize] = masked_rng;
    }

    /// Jump to address NNN + v0
    fn bnnn(cpu: &mut Cpu) {
        let (_, n1, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        let added_address = cpu.registers[0] as u16 + address;
        cpu.index_register = added_address;
    }

    /// Store memory address NNN in register I
    fn annn(cpu: &mut Cpu) {
        let (_, n1, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        cpu.index_register = address;
    }

    /// Skip the following instruction if the value of register vX is not equal to the value of
    /// register vY.
    fn _9xy0(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(emu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        if vx != vy {
            emu.program_counter += 2; // maybe +1 ?
        }
    }

    /// Store the value of register vY shifted left one bit in register vX
    /// Set register vF to the most significant bit prior to the shift
    /// vY is unchanged
    fn _8xye(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(emu);
        let vy = cpu.registers[y as usize];
        let msb_vy = (vy & 0b10000000) >> 7;
        cpu.registers[0xF as usize] = msb_vy;
        let shifted_vy = vy << 1;
        cpu.registers[x as usize] = shifted_vy
    }

    /// Set register VX to the value of VY minus VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    fn _8xy7(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(emu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        let (diff, borrow) = {
            let this = vy;
            let rhs = vx;
            let borrow = false;
            let (a, b) = this.overflowing_sub(rhs);
            let (c, d) = a.overflowing_sub(borrow as u8);
            (c, b || d)
        };
        cpu.registers[x as usize] = diff;
        if borrow {
            cpu.registers[0xF as usize] = 0x00;
        } else {
            cpu.registers[0xF as usize] = 0x01;
        }
    }

    /// Store the value of register VY shifted right one bit in register VX¹
    /// Set register VF to the least significant bit prior to the shift
    /// VY is unchanged
    fn _8xy6(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(emu);
        let vy = cpu.registers[y as usize];
        let lsb_vy = vy & 0b00000001;
        cpu.registers[0xF as usize] = lsb_vy;
        let shifted_vy = vy >> 1;
        cpu.registers[x as usize] = shifted_vy
    }

    /// Subtract the value of register VY from register VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    fn _8xy5(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(emu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        let (diff, borrow) = {
            let this = vx;
            let rhs = vy;
            let borrow = false;
            let (a, b) = this.overflowing_sub(rhs);
            let (c, d) = a.overflowing_sub(borrow as u8);
            (c, b || d)
        };
        cpu.registers[x as usize] = diff;
        if borrow {
            cpu.registers[0xF as usize] = 0x00;
        } else {
            cpu.registers[0xF as usize] = 0x01;
        }
    }

    /// Add the value of register VY to register VX
    /// Set VF to 01 if a carry occurs
    /// Set VF to 00 if a carry does not occur
    //#[feature(bigint_helper_methods)]
    fn _8xy4(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(emu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        let (sum, carry) = {
            let this = vx;
            let rhs = vy;
            let carry = false;
            let (a, b) = this.overflowing_add(rhs);
            let (c, d) = a.overflowing_add(carry as u8);
            (c, b || d)
        };
        cpu.registers[x as usize] = sum;
        cpu.registers[0xF as usize] = carry as u8;
    }

    /// 11 + 11 =>  3 + 3 = 6 = 110 , 111 + 111 = 7+7 = 14 = 1110 , overflow means lsb of larger
    ///    type

    /// Set vX to vX XOR vY
    fn _8xy3(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(emu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        cpu.registers[x as usize] = vx ^ vy;
    }

    /// Set vX to vX AND vY
    fn _8xy2(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(emu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        cpu.registers[x as usize] = vx & vy;
    }

    /// Set vX to vX OR vY
    fn _8xy1(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(emu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        cpu.registers[x as usize] = vx | vy;
    }

    /// Store the value of register vY in register vX
    fn _8xy0(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(emu);
        let vy = cpu.registers[y as usize];
        cpu.registers[x as usize] = vy;
    }

    /// Add the value NN to register vX
    fn _7xnn(cpu: &mut Cpu) {
        let (_, x, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let value = (n2 as u8) << 4 | n3 as u8;
        let temp = cpu.registers[x as usize] + value;
        cpu.registers[x as usize] = temp;
    }

    /// Store the number NN in register vX
    fn _6xnn(cpu: &mut Cpu) {
        let (_, x, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let value = (n2 as u8) << 4 | n3 as u8;
        cpu.registers[x as usize] = value;
    }

    /// Skip the following instruction if the value of register vX is equal to the value of
    /// register vY.
    fn _5xy0(cpu: &mut Cpu) {
        let x = OpCode::get_x(cpu);
        let y = OpCode::get_y(emu);
        let vx = cpu.registers[x as usize];
        let vy = cpu.registers[y as usize];
        if vx == vy {
            cpu.index_register += 1;
        }
    }

    /// Skip the following instruction if the value of register vX is NOT equal to NN
    fn _4xnn(cpu: &mut Cpu) {
        let (_, x, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let value = (n2 as u8) << 4 | n3 as u8;
        let vx = cpu.registers[x as usize];
        if vx != value {
            // Not sure if I should increment program counter by two or increment index_register ?
            // who knows, future galus
            cpu.index_register += 1;
        }
    }

    /// Skip the following instruction if the value of register vX is equal to NN
    fn _3xnn(cpu: &mut Cpu) {
        let (_, x, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let value = (n2 as u8) << 4 | n3 as u8;
        let vx = cpu.registers[x as usize];
        if vx == value {
            // Not sure if I should increment program counter by two or increment index_register ?
            // who knows, future galus
            cpu.index_register += 1;
        }
    }

    /// Execute subroutine starting at address NNN
    fn _2nnn(cpu: &mut Cpu) {
        let (_, n1, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        emu.program_counter = address;
    }

    /// Jump to address NNN
    fn _1nnn(cpu: &mut Cpu) {
        let (_, n1, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        emu.program_counter = address;
    }

    /// Execute machine language subroutine at address NNN
    fn _0nnn(cpu: &mut Cpu) {
        let (_, n1, n2, n3) = cpu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        println!("address d{:?}, x{:x?}", address, address);
        // Figure out if this NNN is BCD'd or if its the bits sequentially
        // where 0000 1111     0000 1011     0000 0111 implies -> 1111 1011 0111
        //              15            11             6         ->    E    B    6
        // otherwise BCD wouldnt allow for 1111, as 9 is the highest bcd.
    }

    /// Clear the screen
    fn _00e0(gpu: &mut Gpu) {
        gpu.screen = [false; 64 * 32];
    }

    /// Return from a subroutine
    fn _00ee(_emu: &Cpu) {
        return;
    }

    /// Returns current opcodes 2nd nibble
    fn get_x(cpu: &Cpu) -> u8 {
        //let op = cpu.current_opcode;
        //let (_, x, _, _) = op.into_tuple();
        //x
        cpu.current_opcode.into_tuple().1
    }

    /// Returns current opcodes 3rd nibble
    fn get_y(emu: &Cpu) -> u8 {
        cpu.current_opcode.into_tuple().2
    }
}

trait Nibbles {
    fn into_tuple(&self) -> (u8, u8, u8, u8);
    // fn into_vec(&self) -> Vec<u8>;
}

impl Nibbles for OpCode {
    fn into_tuple(&self) -> (u8, u8, u8, u8) {
        (
            ((0xF000 & self.0) >> 12) as u8,
            ((0x0F00 & self.0) >> 8) as u8,
            ((0x00F0 & self.0) >> 4) as u8,
            (0x000F & self.0) as u8,
        )
    }

    //fn into_vec(&self) -> Vec<u8> {
    //    let nibbles: Vec<u8> = vec![
    //        ((0xF000 & self.0) >> 12) as u8,
    //        ((0x0F00 & self.0) >> 8) as u8,
    //        ((0x00F0 & self.0) >> 4) as u8,
    //        (0x000F & self.0) as u8,
    //    ];
    //    nibbles
    //}
}

#[derive(Debug)]
#[allow(dead_code)] // REMOVE THIS WHEN DONE
pub struct Cpu {
    pub current_opcode: OpCode,
    // memory: [u8; 4096],
    registers: [u8; 16], // general purpose
    /// Address of the current instruction
    index_register: u16, // can only load 12-bit mem address due to range of mem accessible
    //                      1111 1111 1111 -> 0xFFF -> 4095 -> memsize
    pub program_counter: u16,

    // pub screen: [bool; 64 * 32],
    // pub stack: [u16; 16],
    stack_pointer: usize,

    delay_timer: u8,
    sound_timer: u8,

    pub keypad: [bool; 16],
    pub rom_buffer: Vec<u8>,
    pub running: bool,
}

#[cfg(test)]
mod tests {
    use crate::emu::cpu::Cpu;
    use crate::emu::cpu::OpCode;
    use crate::emu::Memory;

    fn test_init_emu() -> (Cpu, Memory) {
        let mut cpu = Cpu::new();
        // random registers populated
        cpu.registers[0] = 105;
        cpu.registers[1] = 5;
        cpu.registers[2] = 14;
        cpu.registers[7] = 33;
        cpu.registers[12] = 0x11;
        cpu.index_register = 0x200;

        let mut mem = Memory::new();

        // create some fake memory
        mem.data[0x200] = 1;
        mem.data[0x201] = 2;
        mem.data[0x202] = 3;
        mem.data[0x203] = 4;
        mem.data[0x204] = 5;
        (cpu, mem)
    }

    #[test]
    fn test_fx55() {
        let (mut cpu, mut mem) = test_init_emu();
        cpu.current_opcode = OpCode(0xF555);

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
        assert_eq!(mem.data[0x200], cpu.registers[0]);
        assert_eq!(mem.data[0x201], cpu.registers[1]);
        assert_eq!(mem.data[0x202], cpu.registers[2]);
        assert_eq!(mem.data[0x203], cpu.registers[3]);
        assert_eq!(mem.data[0x204], cpu.registers[4]);
        assert_eq!(mem.data[0x205], cpu.registers[5]);

        // this next memory address shouldnt have been affected by 0xF555 b/c x=5
        assert_ne!(mem.data[0x206], cpu.registers[6]);
        assert_eq!(mem.data[0x206], 7);
    }

    #[test]
    fn test_fx65() {
        let mut emu = test_init_emu();
        cpu.current_opcode = OpCode(0xF565);

        // setting up data to check for out of bounds bugs
        (mem.data[0x206], cpu.registers[6]) = (0xDE, 0xAD);

        OpCode::fx65(&mut emu);
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

/* NOTES
DXYN	Display	draw(Vx, Vy, N)
Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
Each row of 8 pixels is read as bit-coded starting from memory location I; I value does not change after the execution of this instruction.
As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen.

*/

// COOL IDEA but im not ready for macros yet
//macro_rules! opcodes {
//    ($name: ident <$word: ty> { $($op: ident = $code: expr),+ $(,)? }) => {
//        #[derive(Debug, Clone, Copy)]
//        pub enum $name {
//            $($op = ( ($code & 0xF000) >> 12,$code & 0x000F)),+
//        }
//
//        impl $name {
//            pub fn to_inst(self) -> $word {
//                self.into()
//            }
//        }
//
//        impl Into<$word> for $name {
//            fn into(self) -> $word {
//                self as $word
//            }
//        }
//    };
//}
//opcodes!(OpCode<u16> {
//    EXEC    = 0x0FFF,
//    CLEAR   = 0x00E0,
//    //RETURN  = 0x00EE,
//    //JMP     = 0x1FFF,
//    //EXECSR  = 0x2FFF,
//    //SKIPEQ  = 0x3FFF,
//    //SKIPNE  = 0x4FFF,
//    //SKIPRE  = 0x5FF0,
//    //LOAD    = 0x6FFF,
//    //ADD     = 0x7FFF,
//    //LOADR   = 0x8FF0,
//    //OR      = 0x8FF1,
//    //AND     = 0x8FF2,
//    //XOR     = 0x8FF3,
//    //ADDC    = 0x8FF4,
//    //SUBC    = 0x8FF5,
//    //RSHIFT  = 0x8FF6, // Right shift
//    //SUB     = 0x8FF7,
//    //LSHIFT  = 0x8FFE,
//    //SKIPNE  = 0x9FF0,
//    //STORE   = 0xAFFF, // in register I
//    //JMP     = 0xBFFF,
//    //RAND    = 0xCFFF,
//    //DRAW    = 0xDFFF,
//    //SKIPKEY = 0xEF9E,
//    //SKIPNKEY = 0xEFA1,
//    //LDDELAY = 0xFF07,
//    //WAITKEY = 0xFF0A,
//    //DELAY   = 0xFF15,
//    //SOUND   = 0xFF18,
//    //ADDRI   = 0xFF1E,
//
//    // 1111 1011 0000 1010
//    // 1111 1111 0000 1010
//// xor 0000
//
//
//
//});

/* Chip8 Memory layout
0x000-0x04F - Chip 8 interpreter (contains font set in emu)       0 -   79
0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)    080 -  160
0x200-0xFFF - Program ROM and work RAM                          512 - 4096

0x200-0xE8F
"final 352 bytes of memory are reserved for “variables and display refresh"
https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#registers
*/
