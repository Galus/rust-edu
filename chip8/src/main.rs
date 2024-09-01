// Copyright (c) 2024 galus. All Rights Reserved.
//    __                        _                                __
//   / /_/\__        __ _  __ _| |_   _ ___             __/\__  / /
//  / /\    /       / _` |/ _` | | | | / __|            \    / / /
// / / /_  _\      | (_| | (_| | | |_| \__ \            /_  _\/ /
///_/    \/         \__, |\__,_|_|\__,_|___/              \/ /_/
//                  |___/
use color_eyre::Result;

mod emojis;
mod emu;

use emojis::EMOJIS; // Avoid Emoji Nightmares
use emu::Emulator;

fn main() -> Result<()> {
    color_eyre::install()?; // error hooks
    println!("{} Initializing emulator", EMOJIS[0]);
    let mut emu: Emulator = Emulator::new();

    println!("\t{} Loading fonts into emulator...", EMOJIS[1]);
    emu.load_font();

    let rom_path: &str = "./roms/maze.ch8";
    println!("\t{} Reading rom {}...", EMOJIS[2], rom_path);
    let rom_data = std::fs::read(rom_path)?;
    emu.cpu.memory.rom = rom_data;

    println!("\t{} Loading rom into emulator...", EMOJIS[3]);
    emu.load_rom(); // clears emu.rom_buffer

    println!("\t{} Initializing terminal...", EMOJIS[4]);
    let mut terminal = emu.cpu.memory.gpu.init()?;

    println!("\t{} Running app...", EMOJIS[5]);

    loop {
        let _ = emu.cpu.fetch_opcode();
        if let Err(err) = emu.cpu.process() {
            eprintln!("failed to process.: {}", err);
            break;
        }

        emu.cpu.memory.gpu.run(&mut terminal)?;

        // Trying to figure out how to have above return a fn ptr
        // display
        // input

        // add a conditioal break
        break;
    }

    if let Err(err) = emu.cpu.memory.gpu.restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }

    println!("{} Exiting...", EMOJIS[6]);
    Ok(())
}
