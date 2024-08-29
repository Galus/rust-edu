// Copyright (c) 2024 galus. All Rights Reserved.
//    __                                                        __
//   / /_/\____/\____/\____/\____/\____/\____/\____/\____/\__  / /
//  / /\    /\    /\    /\    /\    /\    /\    /\    /\    / / /
// / / /_  _\/_  _\/_  _\/_  _\/_  _\/_  _\/_  _\/_  _\/_  _\/ /
///_/    \/    \/    \/    \/    \/    \/    \/    \/    \/ /_/
//
//    __                        _                                __
//   / /_/\__        __ _  __ _| |_   _ ___             __/\__  / /
//  / /\    /       / _` |/ _` | | | | / __|            \    / / /
// / / /_  _\      | (_| | (_| | | |_| \__ \            /_  _\/ /
///_/    \/         \__, |\__,_|_|\__,_|___/              \/ /_/
//                  |___/
//    __                                                        __
//   / /_/\____/\____/\____/\____/\____/\____/\____/\____/\__  / /
//  / /\    /\    /\    /\    /\    /\    /\    /\    /\    / / /
// / / /_  _\/_  _\/_  _\/_  _\/_  _\/_  _\/_  _\/_  _\/_  _\/ /
///_/    \/    \/    \/    \/    \/    \/    \/    \/    \/ /_/
mod cpu;
use cpu::{fetch_opcode, load_font, load_rom, process, Emulator};

fn main() {
    println!("🧨 Initializing emulator");
    let mut emu: Emulator = Emulator::new();
    //println!("emu.memory {:x?}", emu.memory);

    // inits
    println!("\t🖊️ Loading fonts into emulator...");
    let _ = load_font(&mut emu);

    let rom_path: &str = "maze.ch8";
    println!("\t👁️ Reading rom {}...", rom_path);
    let rom_data: Vec<u8> = std::fs::read(rom_path).unwrap();
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
        let _ = fetch_opcode(&mut emu);
        let _ = process(&mut emu);

        // Trying to figure out how to have above return a fn ptr
        // display
        // input

        break;
    }

    //emu.print_memory();
    println!("🍸 Exiting...");
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

#[cfg(test)]
mod tests {
    use crate::Emulator;
    use crate::OpCode;

    fn test_init_emu() -> Emulator {
        let mut emu = Emulator::new();
        // random registers populated
        emu.registers[0] = 105;
        emu.registers[1] = 5;
        emu.registers[2] = 14;
        emu.registers[7] = 33;
        emu.registers[12] = 0x11;
        emu.index_register = 0x200;

        // create some fake memory
        emu.memory[0x200] = 1;
        emu.memory[0x201] = 2;
        emu.memory[0x202] = 3;
        emu.memory[0x203] = 4;
        emu.memory[0x204] = 5;
        emu
    }

    #[test]
    fn test_fx55() {
        let mut emu = test_init_emu();
        emu.current_opcode = OpCode(0xF555);

        // memory should be 1-7 at 0x200-206
        emu.memory[0x205] = 6;
        emu.memory[0x206] = 7;

        assert_eq!(emu.memory[0x200], 1);
        assert_eq!(emu.memory[0x201], 2);
        assert_eq!(emu.memory[0x202], 3);
        assert_eq!(emu.memory[0x203], 4);
        assert_eq!(emu.memory[0x204], 5);
        assert_eq!(emu.memory[0x205], 6);
        assert_eq!(emu.memory[0x206], 7);

        OpCode::fx55(&mut emu);

        // our x was 5, v0..vx needs to get set with I..I+x
        assert_eq!(emu.memory[0x200], emu.registers[0]);
        assert_eq!(emu.memory[0x201], emu.registers[1]);
        assert_eq!(emu.memory[0x202], emu.registers[2]);
        assert_eq!(emu.memory[0x203], emu.registers[3]);
        assert_eq!(emu.memory[0x204], emu.registers[4]);
        assert_eq!(emu.memory[0x205], emu.registers[5]);

        // this next memory address shouldnt have been affected by 0xF555 b/c x=5
        assert_ne!(emu.memory[0x206], emu.registers[6]);
        assert_eq!(emu.memory[0x206], 7);
    }

    #[test]
    fn test_fx65() {
        let mut emu = test_init_emu();
        emu.current_opcode = OpCode(0xF565);

        // setting up data to check for out of bounds bugs
        (emu.memory[0x206], emu.registers[6]) = (0xDE, 0xAD);

        OpCode::fx65(&mut emu);
        // our x was 5, v0..vx needs to get set with I..I+x
        assert_eq!(emu.memory[0x200], emu.registers[0]);
        assert_eq!(emu.memory[0x201], emu.registers[1]);
        assert_eq!(emu.memory[0x202], emu.registers[2]);
        assert_eq!(emu.memory[0x203], emu.registers[3]);
        assert_eq!(emu.memory[0x204], emu.registers[4]);
        assert_eq!(emu.memory[0x205], emu.registers[5]);
        assert_ne!(emu.memory[0x206], emu.registers[6]);
    }

    #[test]
    fn test_fx33() {
        let mut emu = Emulator::new();
        emu.current_opcode = OpCode(0xF533);
        emu.registers[5] = 105;
        emu.index_register = 0x200; // unnecessary but oh well...

        // Test init wierd mishaps
        assert_eq!(emu.memory[emu.index_register as usize], 0);
        assert_eq!(emu.memory[(emu.index_register + 1) as usize], 0);
        assert_eq!(emu.memory[(emu.index_register + 2) as usize], 0);
        println!("index_register: {:?}", emu.index_register);
        let idxr: usize = emu.index_register as usize;
        println!("memory[ir..ir+3]: {:x?}", &emu.memory[(idxr)..(idxr + 3)]);

        // Test fx33
        OpCode::fx33(&mut emu);
        println!("memory[ir..ir+3]: {:x?}", &emu.memory[(idxr)..(idxr + 3)]);

        assert_eq!(emu.memory[emu.index_register as usize], 1);
        assert_eq!(emu.memory[(emu.index_register + 1) as usize], 0);
        assert_eq!(emu.memory[(emu.index_register + 2) as usize], 5);

        //println!("{:x?}", &emu.memory);
        //assert_eq!(emu.memory[(emu.index_register + 2) as usize], 8);
    }

    #[test]
    fn test_fx1e() {
        // init
        let mut emu = Emulator::new();
        emu.index_register = 0x200; // unnecessary but oh well...

        // save before
        emu.current_opcode = OpCode(0xF51E);
        let old_i = emu.index_register.clone();
        println!("old_i: {:?}", old_i);

        // test fx1e to see if vX = 0 works
        // b/c x=5 -> v[5] and b/c all registers are 0'd out now -> 0
        OpCode::fx1e(&mut emu); // add 5 to i
        assert_eq!(emu.index_register, old_i);

        emu.registers[5] = 3;
        OpCode::fx1e(&mut emu); // add 5 to i
        assert_eq!(emu.index_register, old_i + 3);
    }
}
