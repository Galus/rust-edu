pub(crate) mod cpu;
mod gpu;
mod input;
mod memory;
mod timer;

use cpu::{Cpu, OpCode};
use gpu::Gpu;
use input::Keypad;
use memory::{Memory, FONTS};
use timer::Timer;

#[derive(Debug)]
pub struct Emulator {
    pub cpu: Cpu,
    pub gpu: Gpu,
    pub memory: Memory,
    keypad: Keypad,
    delay_timer: Timer,
    sound_timer: Timer,
    pub rom_buffer: Vec<u8>,
    pub running: bool,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            memory: Memory::new(),
            keypad: Keypad::new(),
            delay_timer: Timer::new(),
            sound_timer: Timer::new(),
            rom_buffer: Vec::new(),
            running: false,
        }
    }

    pub fn fetch_opcode(&mut self) -> Result<bool, bool> {
        let opcode_high: u8 = self.memory.data[self.cpu.program_counter as usize];
        let opcode_low: u8 = self.memory.data[self.cpu.program_counter as usize + 1];
        let opcode: u16 = (opcode_high as u16) << 8 | opcode_low as u16;
        self.cpu.current_opcode = OpCode(opcode);
        Ok(true)
    }

    /// The print_memory function has been moved to the Memory module
    pub fn print_memory(&self) {
        for (i, byte) in self.memory.data.iter().enumerate() {
            if i % 16 == 0 {
                println!("\n{:04X}: ", i);
            }
            print!("{:02X} ", byte);
        }
        println!();
    }

    pub fn load_font(&mut self) -> Result<bool, bool> {
        self.memory.data[0..80].copy_from_slice(&FONTS);
        Ok(true)
    }

    /// Puts the emu.rom_buffer into the emu.memory
    pub fn load_rom(&mut self) -> Result<bool, bool> {
        let rom_length: usize = self.rom_buffer.len();
        self.memory.data[512..512 + rom_length].copy_from_slice(&self.rom_buffer);
        self.rom_buffer.clear();
        Ok(true)
    }
}
