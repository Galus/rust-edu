/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
#[derive(Debug, Copy, Clone)]
struct OpCode(u16);
impl OpCode {
    /// Fill registers v0 to vX inclusive.
    /// Sets I = I + X + 1
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    fn fx65(emu: &mut Emulator) {
        let num_registers = OpCode::get_x(&emu);
        for x in 0..=num_registers {
            let load_index = emu.index_register + (x as u16);
            emu.registers[x as usize] = emu.memory[load_index as usize]
        }
        emu.index_register += (num_registers + 1) as u16;
    }

    /// Store register vals v0 to vX inclusive in memory starting at address I.
    /// Sets I = I + X + 1
    /// Basically fx65 but instead of putting memory into registers, puts registers into memory.
    fn fx55(emu: &mut Emulator) {
        let num_registers = OpCode::get_x(&emu);
        for x in 0..=num_registers {
            let load_index = emu.index_register + (x as u16);
            emu.memory[load_index as usize] = emu.registers[x as usize];
        }
        emu.index_register += (num_registers + 1) as u16;
    }

    /// Store BCD of value in vX at addresses I, I+1, I+2
    fn fx33(emu: &mut Emulator) {
        let x = OpCode::get_x(&emu);
        let register = emu.registers[x as usize];
        let padded = format!("{:0>3}", register);
        let a: u8 = padded.chars().nth(0).unwrap() as u8 - 48; // ascii '0' starts at decimal 48
        let b: u8 = padded.chars().nth(1).unwrap() as u8 - 48;
        let c: u8 = padded.chars().nth(2).unwrap() as u8 - 48;
        let index = emu.index_register as usize;
        emu.memory[index] = a;
        emu.memory[index + 1] = b;
        emu.memory[index + 2] = c;
    }

    /// Set I to memory address of the sprite data corresponding to hex digit stored in register vX
    fn fx29(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let vx = &emu.registers[x as usize];
        emu.index_register = *vx as u16;
    }

    /// Add the value stored in register vX to register I
    /// Set I = I + Vx.
    /// The values of I and Vx are added, and the results are stored in I.
    fn fx1e(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let vx = &emu.registers[x as usize];
        let i = &emu.index_register;
        let new_i = (*vx) as u16 + i;
        emu.index_register = new_i;
    }

    /// Set the sound timer to value of register vX
    fn fx18(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let vx = emu.registers[x as usize];
        emu.sound_timer = vx;
    }

    /// Set the delay timer to the value of register vX
    fn fx15(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let vx = emu.registers[x as usize];
        emu.delay_timer = vx;
    }

    /// Wait for a keypress and store the result in register vX
    fn fx0a(emu: &mut Emulator) {
        todo!()
    }

    /// Store the current value of the delay timer in register vX
    fn fx07(emu: &mut Emulator) {
        let delay_timer = emu.delay_timer;
        let x = OpCode::get_x(emu);
        emu.registers[x as usize] = delay_timer;
    }

    /// Skip the following instruction if the key corresponding to the hex value currently stored
    /// in register vX is NOT pressed
    fn exa1(emu: &mut Emulator) {
        todo!()
    }

    /// Skip the following instruction if the key corresponding to the hex value currently stored
    /// in register vX is pressed
    fn ex9e(emu: &mut Emulator) {
        todo!()
    }

    /// Draw a sprite at position vX, vY with N bytes of sprite data starting at the address
    /// stored in I. Set vF to 01 if any set pixels are changed to unset, and 00 otherwise.
    fn dxyn(emu: &mut Emulator) {
        todo!()
    }

    /// Set vX to a random number with a mask of NN
    fn cxnn(emu: &mut Emulator) {
        let (_, x, n2, n3) = emu.current_opcode.into_tuple(); //opcodes are u16
        let rng = rand::random::<u8>();
        let masked_rng = (n2 << 4 | n3) & rng;
        emu.registers[x as usize] = masked_rng;
    }

    /// Jump to address NNN + v0
    fn bnnn(emu: &mut Emulator) {
        let (_, n1, n2, n3) = emu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        let added_address = emu.registers[0] as u16 + address;
        emu.index_register = added_address;
    }

    /// Store memory address NNN in register I
    fn annn(emu: &mut Emulator) {
        let (_, n1, n2, n3) = emu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        emu.index_register = address;
    }

    /// Skip the following instruction if the value of register vX is not equal to the value of
    /// register vY.
    fn _9xy0(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let y = OpCode::get_y(emu);
        let vx = emu.registers[x as usize];
        let vy = emu.registers[y as usize];
        if vx != vy {
            emu.program_counter += 2; // maybe +1 ?
        }
    }

    /// Store the value of register vY shifted left one bit in register vX
    /// Set register vF to the most significant bit prior to the shift
    /// vY is unchanged
    fn _8xye(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let y = OpCode::get_y(emu);
        let vy = emu.registers[y as usize];
        let msb_vy = (vy & 0b10000000) >> 7;
        emu.registers[0xF as usize] = msb_vy;
        let shifted_vy = vy << 1;
        emu.registers[x as usize] = shifted_vy
    }

    /// Set register VX to the value of VY minus VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    fn _8xy7(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let y = OpCode::get_y(emu);
        let vx = emu.registers[x as usize];
        let vy = emu.registers[y as usize];
        let (diff, borrow) = {
            let this = vy;
            let rhs = vx;
            let borrow = false;
            let (a, b) = this.overflowing_sub(rhs);
            let (c, d) = a.overflowing_sub(borrow as u8);
            (c, b || d)
        };
        emu.registers[x as usize] = diff;
        if borrow {
            emu.registers[0xF as usize] = 0x00;
        } else {
            emu.registers[0xF as usize] = 0x01;
        }
    }

    /// Store the value of register VY shifted right one bit in register VX¹
    /// Set register VF to the least significant bit prior to the shift
    /// VY is unchanged
    fn _8xy6(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let y = OpCode::get_y(emu);
        let vy = emu.registers[y as usize];
        let lsb_vy = vy & 0b00000001;
        emu.registers[0xF as usize] = lsb_vy;
        let shifted_vy = vy >> 1;
        emu.registers[x as usize] = shifted_vy
    }

    /// Subtract the value of register VY from register VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    fn _8xy5(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let y = OpCode::get_y(emu);
        let vx = emu.registers[x as usize];
        let vy = emu.registers[y as usize];
        let (diff, borrow) = {
            let this = vx;
            let rhs = vy;
            let borrow = false;
            let (a, b) = this.overflowing_sub(rhs);
            let (c, d) = a.overflowing_sub(borrow as u8);
            (c, b || d)
        };
        emu.registers[x as usize] = diff;
        if borrow {
            emu.registers[0xF as usize] = 0x00;
        } else {
            emu.registers[0xF as usize] = 0x01;
        }
    }

    /// Add the value of register VY to register VX
    /// Set VF to 01 if a carry occurs
    /// Set VF to 00 if a carry does not occur
    //#[feature(bigint_helper_methods)]
    fn _8xy4(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let y = OpCode::get_y(emu);
        let vx = emu.registers[x as usize];
        let vy = emu.registers[y as usize];
        let (sum, carry) = {
            let this = vx;
            let rhs = vy;
            let carry = false;
            let (a, b) = this.overflowing_add(rhs);
            let (c, d) = a.overflowing_add(carry as u8);
            (c, b || d)
        };
        emu.registers[x as usize] = sum;
        emu.registers[0xF as usize] = carry as u8;
    }

    /// 11 + 11 =>  3 + 3 = 6 = 110 , 111 + 111 = 7+7 = 14 = 1110 , overflow means lsb of larger
    ///    type

    /// Set vX to vX XOR vY
    fn _8xy3(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let y = OpCode::get_y(emu);
        let vx = emu.registers[x as usize];
        let vy = emu.registers[y as usize];
        emu.registers[x as usize] = vx ^ vy;
    }

    /// Set vX to vX AND vY
    fn _8xy2(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let y = OpCode::get_y(emu);
        let vx = emu.registers[x as usize];
        let vy = emu.registers[y as usize];
        emu.registers[x as usize] = vx & vy;
    }

    /// Set vX to vX OR vY
    fn _8xy1(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let y = OpCode::get_y(emu);
        let vx = emu.registers[x as usize];
        let vy = emu.registers[y as usize];
        emu.registers[x as usize] = vx | vy;
    }

    /// Store the value of register vY in register vX
    fn _8xy0(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let y = OpCode::get_y(emu);
        let vy = emu.registers[y as usize];
        emu.registers[x as usize] = vy;
    }

    /// Add the value NN to register vX
    fn _7xnn(emu: &mut Emulator) {
        let (_, x, n2, n3) = emu.current_opcode.into_tuple(); //opcodes are u16
        let value = (n2 as u8) << 4 | n3 as u8;
        let temp = emu.registers[x as usize] + value;
        emu.registers[x as usize] = temp;
    }

    /// Store the number NN in register vX
    fn _6xnn(emu: &mut Emulator) {
        let (_, x, n2, n3) = emu.current_opcode.into_tuple(); //opcodes are u16
        let value = (n2 as u8) << 4 | n3 as u8;
        emu.registers[x as usize] = value;
    }

    /// Skip the following instruction if the value of register vX is equal to the value of
    /// register vY.
    fn _5xy0(emu: &mut Emulator) {
        let x = OpCode::get_x(emu);
        let y = OpCode::get_y(emu);
        let vx = emu.registers[x as usize];
        let vy = emu.registers[y as usize];
        if vx == vy {
            emu.index_register += 1;
        }
    }

    /// Skip the following instruction if the value of register vX is NOT equal to NN
    fn _4xnn(emu: &mut Emulator) {
        let (_, x, n2, n3) = emu.current_opcode.into_tuple(); //opcodes are u16
        let value = (n2 as u8) << 4 | n3 as u8;
        let vx = emu.registers[x as usize];
        if vx != value {
            // Not sure if I should increment program counter by two or increment index_register ?
            // who knows, future galus
            emu.index_register += 1;
        }
    }

    /// Skip the following instruction if the value of register vX is equal to NN
    fn _3xnn(emu: &mut Emulator) {
        let (_, x, n2, n3) = emu.current_opcode.into_tuple(); //opcodes are u16
        let value = (n2 as u8) << 4 | n3 as u8;
        let vx = emu.registers[x as usize];
        if vx == value {
            // Not sure if I should increment program counter by two or increment index_register ?
            // who knows, future galus
            emu.index_register += 1;
        }
    }

    /// Execute subroutine starting at address NNN
    fn _2nnn(emu: &mut Emulator) {
        let (_, n1, n2, n3) = emu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        emu.program_counter = address;
    }

    /// Jump to address NNN
    fn _1nnn(emu: &mut Emulator) {
        let (_, n1, n2, n3) = emu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        emu.program_counter = address;
    }

    /// Execute machine language subroutine at address NNN
    fn _0nnn(emu: &mut Emulator) {
        let (_, n1, n2, n3) = emu.current_opcode.into_tuple(); //opcodes are u16
        let address = (n1 as u16) << 8 | (n2 as u16) << 4 | n3 as u16;
        println!("address d{:?}, x{:x?}", address, address);
        // Figure out if this NNN is BCD'd or if its the bits sequentially
        // where 0000 1111     0000 1011     0000 0111 implies -> 1111 1011 0111
        //              15            11             6         ->    E    B    6
        // otherwise BCD wouldnt allow for 1111, as 9 is the highest bcd.
    }

    /// Clear the screen
    fn _00e0(emu: &mut Emulator) {
        emu.screen = [false; 64 * 32];
        todo!()
    }

    /// Return from a subroutine
    fn _00ee(emu: &Emulator) {
        return;
    }

    /// Returns current opcodes 2nd nibble
    fn get_x(emu: &Emulator) -> u8 {
        //let op = emu.current_opcode;
        //let (_, x, _, _) = op.into_tuple();
        //x
        emu.current_opcode.into_tuple().1
    }

    /// Returns current opcodes 3rd nibble
    fn get_y(emu: &Emulator) -> u8 {
        emu.current_opcode.into_tuple().2
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
pub struct Emulator {
    pub current_opcode: OpCode,
    memory: [u8; 4096],

    registers: [u8; 16], // general purpose
    /// Address of the current instruction
    index_register: u16, // can only load 12-bit mem address due to range of mem accessible
    //                      1111 1111 1111 -> 0xFFF -> 4095 -> memsize
    program_counter: u16,

    pub screen: [bool; 64 * 32],

    pub stack: [u16; 16],
    stack_pointer: usize,

    delay_timer: u8,
    sound_timer: u8,

    pub keypad: [bool; 16],
    pub rom_buffer: Vec<u8>,
    pub running: bool,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            current_opcode: OpCode(0),
            memory: [0; 4096],
            registers: [0; 16],
            index_register: 0,
            program_counter: 0x200,
            screen: [false; 64 * 32],
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            rom_buffer: Vec::new(),
            running: false,
        }
    }

    pub fn print_memory(&self) {
        for (i, byte) in self.memory.iter().enumerate() {
            if i % 16 == 0 {
                println!("\n{:04X}: ", i);
            }
            print!("{:02X} ", byte);
        }
        println!();
    }
}

/*
# Chip8 FONT encoding
16 chars are encoded from 00 - 4F
## Examples
0 = 0xF0 0x90 0x90 0x90 0xF0
0xF0 =   1 1 1 1 0 0 0 0     = 1 1 1 1
0x90 =   1 0 0 1 0 0 0 0     = 1     1
0x90 =   1 0 0 1 0 0 0 0     = 1     1
0x90 =   1 0 0 1 0 0 0 0     = 1     1
0xF0 =   1 1 1 1 0 0 0 0     = 1 1 1 1

1 = 0x20 0x60 0x20 0x20 0x70
0x20 =  0 0 1 0 0 0 0 0     =   1
0x60 =  0 1 1 0 0 0 0 0     =  11
0x20 =  0 0 1 0 0 0 0 0     =   1
0x20 =  0 0 1 0 0 0 0 0     =   1
0x70 =  0 1 1 1 0 0 0 0     =  111

2 = 0xF0 0x10 0xF0 0x80 0xF0
0xF0 =   1 1 1 1 0 0 0 0     = 1 1 1 1
0x10 =   0 0 0 1 0 0 0 0     =       1
0xF0 =   1 1 1 1 0 0 0 0     = 1 1 1 1
0x80 =   1 0 0 0 0 0 0 0     = 1
0xF0 =   1 1 1 1 0 0 0 0     = 1 1 1 1

I love Kodak Black~ SniperGangKodak
Start of memory */
pub const FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub fn load_font(emu: &mut Emulator) -> Result<bool, bool> {
    emu.memory[0..80].copy_from_slice(&FONTS);
    Ok(true)
}

/// Puts the emu.rom_buffer into the emu.memory
pub fn load_rom(emu: &mut Emulator) -> Result<bool, bool> {
    let rom_length: usize = emu.rom_buffer.len();
    emu.memory[512..512 + rom_length].copy_from_slice(&emu.rom_buffer);
    emu.rom_buffer.clear();
    Ok(true)
}

pub fn fetch_opcode(emu: &mut Emulator) -> Result<bool, bool> {
    // PsuedoCode:
    //   OpCode = concat( memory[*pc], memory[*pc+1] ) = the opcode is at ProgramCounter and PC+1
    let opcode_high: u8 = emu.memory[emu.program_counter as usize];
    let opcode_low: u8 = emu.memory[emu.program_counter as usize + 1];
    let opcode: u16 = (opcode_high as u16) << 8 | opcode_low as u16;
    emu.current_opcode = OpCode(opcode);
    // println!("current_opcode {:x?}", opcode);
    Ok(true)
}

/// Map the current OpCode to an actual function.
pub fn process(emu: &mut Emulator) {
    // DECODE and Process
    let op = &emu.current_opcode;
    match op.into_tuple() {
        (0, 0, 0xE, 0xE) => OpCode::_00ee(emu),
        (0, 0, 0xE, 0) => OpCode::_00e0(emu),
        (0, _, _, _) => OpCode::_0nnn(emu),
        (1, _, _, _) => OpCode::_1nnn(emu),
        (2, _, _, _) => OpCode::_2nnn(emu),
        (3, _, _, _) => OpCode::_3xnn(emu),
        (4, _, _, _) => OpCode::_4xnn(emu),
        (5, _, _, 0) => OpCode::_5xy0(emu),
        (6, _, _, _) => OpCode::_6xnn(emu),
        (7, _, _, _) => OpCode::_7xnn(emu),
        (8, _, _, 0) => OpCode::_8xy0(emu),
        (8, _, _, 1) => OpCode::_8xy1(emu),
        (8, _, _, 2) => OpCode::_8xy2(emu),
        (8, _, _, 3) => OpCode::_8xy3(emu),
        (8, _, _, 4) => OpCode::_8xy4(emu),
        (8, _, _, 5) => OpCode::_8xy5(emu),
        (8, _, _, 6) => OpCode::_8xy6(emu),
        (8, _, _, 7) => OpCode::_8xy7(emu),
        (8, _, _, 0xE) => OpCode::_8xye(emu),
        (9, _, _, 0) => OpCode::_9xy0(emu),
        (0xA, _, _, _) => OpCode::annn(emu),
        (0xB, _, _, _) => OpCode::bnnn(emu),
        (0xC, _, _, _) => OpCode::cxnn(emu),
        (0xD, _, _, _) => OpCode::dxyn(emu),
        (0xE, _, 9, 0xE) => OpCode::ex9e(emu),
        (0xE, _, 0xA, 1) => OpCode::exa1(emu),
        (0xF, _, 0, 7) => OpCode::fx07(emu),
        (0xF, _, 0, 0xA) => OpCode::fx0a(emu),
        (0xF, _, 1, 5) => OpCode::fx15(emu),
        (0xF, _, 1, 8) => OpCode::fx18(emu),
        (0xF, _, 1, 0xE) => OpCode::fx1e(emu),
        (0xF, _, 2, 9) => OpCode::fx29(emu),
        (0xF, _, 3, 3) => OpCode::fx33(emu),
        (0xF, _, 5, 5) => OpCode::fx55(emu),
        (0xF, _, 6, 5) => OpCode::fx65(emu),
        (a, b, c, d) => println!("Not implemented {:x?}", (a, b, c, d)),
    }
}
