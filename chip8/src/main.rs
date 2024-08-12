// Copyright (c) 2024 galus. All Rights Reserved.

use std::fs;

#[derive(Debug)]
struct OpCode {
    l: u8,
    r: u8,
}

#[derive(Debug)]
struct Emulator {
    pub current_opcode: OpCode,
    /* Chip8 Memory layout
    0x000-0x04F - Chip 8 interpreter (contains font set in emu)       0 -   79
    0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)    080 -  160
    0x200-0xFFF - Program ROM and work RAM                          512 - 4096 */
    memory: [u8; 4096],

    registers: [u8; 16],
    index_register: u16,
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
            current_opcode: OpCode { l: 0, r: 0 },
            memory: [0; 4096],
            registers: [0; 16],
            index_register: 0,
            program_counter: 0,
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

fn main() {
    println!("Init emulator");
    let mut emu: Emulator = Emulator::new();
    println!("emu.memory {:x?}", emu.memory);

    // inits
    let _ = load_font(&mut emu);
    let rom_path: &str = "maze.ch8";
    let rom_data: Vec<u8> = fs::read(rom_path).unwrap();
    emu.rom_buffer = rom_data.clone();
    println!("BEFORE");
    println!("\temu.rom_buffer {:x?}", emu.rom_buffer);
    println!("\trom_data {:x?}", &rom_data);
    let _ = load_rom(&mut emu); // clears emu.rom_buffer
    println!("AFTER");
    println!("\temu.rom_buffer {:x?}", emu.rom_buffer);
    println!("\trom_data {:x?}", &rom_data);

    println!("emu.memory {:x?}", emu.memory);
}

/* NOTES
DXYN	Display	draw(Vx, Vy, N)
Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
Each row of 8 pixels is read as bit-coded starting from memory location I; I value does not change after the execution of this instruction.
As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen.

*/
