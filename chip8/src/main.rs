// Copyright (c) 2024 galus. All Rights Reserved.

use std::fs;

#[derive(Debug)]
struct OpCode(u16);
impl OpCode {
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

    /// Fill registers v0 to vX inclusive.
    /// Sets I = I + X + 1
    fn fx65(&self) {
        todo!()
    }

    /// Store register vals v0 to vX inclusive in memory starting at address I.
    /// Sets I = I + X + 1
    fn fx55(&self) {
        todo!()
    }

    /// Store BCD of value in vX at addresses I, I+1, I+2
    fn fx33(&self) {
        todo!()
    }

    /// Set I to memory address of the sprite data corresponding to hex digit stored in register vX
    fn fx29(&self) {
        todo!()
    }

    /// Add the value stored in register vX to register I
    fn fx1e(&self) {
        todo!()
    }

    /// Set the sound timer to value of register vX
    fn fx18(&self) {
        todo!()
    }

    /// Set the delay timer to the value of register vX
    fn fx15(&self) {
        todo!()
    }

    /// Wait for a keypress and store the result in register vX
    fn fx0a(&self) {
        todo!()
    }

    /// Store the current value of the delay timer in register vX
    fn fx07(&self) {
        todo!()
    }

    /// Skip the following instruction if the key corresponding to the hex value currently stored
    /// in register vX is NOT pressed
    fn exa1(&self) {
        todo!()
    }

    /// Skip the following instruction if the key corresponding to the hex value currently stored
    /// in register vX is pressed
    fn ex9e(&self) {
        todo!()
    }

    /// Draw a sprite at position vX, vY with N bytes of sprite data starting at the address
    /// stored in I. Set vF to 01 if any set pixels are changed to unset, and 00 otherwise.
    fn dxyn(&self) {
        todo!()
    }

    /// Set vX to a random number with a mask of NN
    fn cxnn(&self) {
        todo!()
    }

    /// Jump to address NNN + v0
    fn bnnn(&self) {
        todo!()
    }

    /// Store memory address NNN in register I
    fn annn(&self) {
        println!("hue hue hue");
        todo!()
    }

    /// Skip the following instruction if the value of register vX is not equal to the value of
    /// register vY.
    fn _9xy0(&self) {
        todo!()
    }

    /// Store the value of register vY shifted left one bit in register vX
    /// Set register vF to the most significant bit prior to the shift
    /// vY is unchanged
    fn _8xye(&self) {
        todo!()
    }

    /// Set register VX to the value of VY minus VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    fn _8xy7(&self) {
        todo!()
    }

    /// Store the value of register VY shifted right one bit in register VX¹
    /// Set register VF to the least significant bit prior to the shift
    /// VY is unchanged
    fn _8xy6(&self) {
        todo!()
    }

    /// Subtract the value of register VY from register VX
    /// Set VF to 00 if a borrow occurs
    /// Set VF to 01 if a borrow does not occur
    fn _8xy5(&self) {
        todo!()
    }

    /// Add the value of register VY to register VX
    /// Set VF to 01 if a carry occurs
    /// Set VF to 00 if a carry does not occur
    fn _8xy4(&self) {
        todo!()
    }

    /// Set vX to vX XOR vY
    fn _8xy3(&self) {
        todo!()
    }

    /// Set vX to vX AND vY
    fn _8xy2(&self) {
        todo!()
    }

    /// Set vX to vX OR vY
    fn _8xy1(&self) {
        todo!()
    }

    /// Store the value of register vY in register vX
    fn _8xy0(&self) {
        todo!()
    }

    /// Add the value NN to register vX
    fn _7xnn(&self) {
        todo!()
    }

    /// Store the number NN in register vX
    fn _6xnn(&self) {
        todo!()
    }

    /// Skip the following instruction if the value of register vX is equal to the value of
    /// register vY.
    fn _5xy0(&self) {
        todo!()
    }

    /// Skip the following instruction if the value of register vX is NOT equal to NN
    fn _4xnn(&self) {
        todo!()
    }

    /// Skip the following instruction if the value of register vX is equal to NN
    fn _3xnn(&self) {
        todo!()
    }

    /// Execute subroutine starting at address NNN
    fn _2nnn(&self) {
        todo!()
    }

    /// Jump to address NNN
    fn _1nnn(&self) {
        todo!()
    }

    /// Execute machine language subroutine at address NNN
    fn _0nnn(&self) {
        todo!()
    }

    /// Clear the screen
    fn _00e0(&self) {
        todo!()
    }

    /// Return from a subroutine
    fn _00ee(&self) {
        todo!()
    }
}

trait Nibbles {
    fn into_tuple(&self) -> (u8, u8, u8, u8);
    fn into_vec(&self) -> Vec<u8>;
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

    fn into_vec(&self) -> Vec<u8> {
        let nibbles: Vec<u8> = vec![
            ((0xF000 & self.0) >> 12) as u8,
            ((0x0F00 & self.0) >> 8) as u8,
            ((0x00F0 & self.0) >> 4) as u8,
            (0x000F & self.0) as u8,
        ];
        nibbles
    }
}

#[derive(Debug)]
#[allow(dead_code)] // REMOVE THIS WHEN DONE
struct Emulator {
    pub current_opcode: OpCode,
    memory: [u8; 4096],

    registers: [u8; 16], // general purpose
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

fn load_font(emu: &mut Emulator) -> Result<bool, bool> {
    emu.memory[0..80].copy_from_slice(&FONTS);
    Ok(true)
}

/// Puts the emu.rom_buffer into the emu.memory
fn load_rom(emu: &mut Emulator) -> Result<bool, bool> {
    let rom_length: usize = emu.rom_buffer.len();
    emu.memory[512..512 + rom_length].copy_from_slice(&emu.rom_buffer);
    emu.rom_buffer.clear();
    Ok(true)
}

/// Map the OpCode to an actual function.
fn process(op: OpCode) {
    // DECODE and Process
    match op.into_tuple() {
        (0, 0, 0xE, 0xE) => op._00ee(),
        (0, 0, 0xE, 0) => op._00e0(),
        (0, _, _, _) => op._0nnn(),
        (1, _, _, _) => op._1nnn(),
        (2, _, _, _) => op._2nnn(),
        (3, _, _, _) => op._3xnn(),
        (4, _, _, _) => op._4xnn(),
        (5, _, _, 0) => op._5xy0(),
        (6, _, _, _) => op._6xnn(),
        (7, _, _, _) => op._7xnn(),
        (8, _, _, 0) => op._8xy0(),
        (8, _, _, 1) => op._8xy1(),
        (8, _, _, 2) => op._8xy2(),
        (8, _, _, 3) => op._8xy3(),
        (8, _, _, 4) => op._8xy4(),
        (8, _, _, 5) => op._8xy5(),
        (8, _, _, 6) => op._8xy6(),
        (8, _, _, 7) => op._8xy7(),
        (8, _, _, 0xE) => op._8xye(),
        (9, _, _, 0) => op._9xy0(),
        (0xA, _, _, _) => op.annn(),
        (0xB, _, _, _) => op.bnnn(),
        (0xC, _, _, _) => op.cxnn(),
        (0xD, _, _, _) => op.dxyn(),
        (0xE, _, 9, 0xE) => op.ex9e(),
        (0xE, _, 0xA, 1) => op.exa1(),
        (0xF, _, 0, 7) => op.fx07(),
        (0xF, _, 0, 0xA) => op.fx0a(),
        (0xF, _, 1, 5) => op.fx15(),
        (0xF, _, 1, 8) => op.fx18(),
        (0xF, _, 1, 0xE) => op.fx1e(),
        (0xF, _, 2, 9) => op.fx29(),
        (0xF, _, 3, 3) => op.fx33(),
        (0xF, _, 5, 5) => op.fx55(),
        (0xF, _, 6, 5) => op.fx65(),
        (a, b, c, d) => println!("Not implemented {:x?}", (a, b, c, d)),
    }
}

fn main() {
    println!("🧨 Initializing emulator");
    let mut emu: Emulator = Emulator::new();
    //println!("emu.memory {:x?}", emu.memory);

    // inits
    println!("\t🖊️ Loading fonts into emulator...");
    let _ = load_font(&mut emu);

    let rom_path: &str = "maze.ch8";
    println!("\t👁️ Reading rom {}...", rom_path);
    let rom_data: Vec<u8> = fs::read(rom_path).unwrap();
    emu.rom_buffer = rom_data.clone();

    println!("\t🕹️ Loading rom into emulator...");
    //println!("BEFORE");
    //println!("\temu.rom_buffer {:x?}", emu.rom_buffer);
    //println!("\trom_data {:x?}", &rom_data);
    let _ = load_rom(&mut emu); // clears emu.rom_buffer
                                //println!("AFTER");
                                //println!("\temu.rom_buffer {:x?}", emu.rom_buffer);
                                //println!("\trom_data {:x?}", &rom_data);

    loop {
        // instruction cycle
        // - fetch
        let _ = fetch_opcode(&mut emu);
        //println!("current_opcode {:x?}", emu.current_opcode);
        //println!("co.into_vec {:x?}", emu.current_opcode.into_vec());
        //println!("co.into_tuple {:x?}", emu.current_opcode.into_tuple());

        // - decode and execute
        let _ = process(emu.current_opcode);

        // Trying to figure out how to have above return a fn ptr

        // display
        // input

        break;
    }

    println!("emu.memory {:x?}", emu.memory);
    println!("🍸 Exiting...");
}

fn fetch_opcode(emu: &mut Emulator) -> Result<bool, bool> {
    // PsuedoCode:
    //   OpCode = concat( memory[*pc], memory[*pc+1] ) = the opcode is at ProgramCounter and PC+1
    let opcode_high: u8 = emu.memory[emu.program_counter as usize];
    let opcode_low: u8 = emu.memory[emu.program_counter as usize + 1];
    let opcode: u16 = (opcode_high as u16) << 8 | opcode_low as u16;
    emu.current_opcode = OpCode(opcode);
    // println!("current_opcode {:x?}", opcode);
    Ok(true)
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
