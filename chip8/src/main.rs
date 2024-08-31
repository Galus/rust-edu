// Copyright (c) 2024 galus. All Rights Reserved.
//    __                        _                                __
//   / /_/\__        __ _  __ _| |_   _ ___             __/\__  / /
//  / /\    /       / _` |/ _` | | | | / __|            \    / / /
// / / /_  _\      | (_| | (_| | | |_| \__ \            /_  _\/ /
///_/    \/         \__, |\__,_|_|\__,_|___/              \/ /_/
//                  |___/
use color_eyre::Result;

mod cpu;
mod gpu;
mod tui;

use cpu::{fetch_opcode, load_font, load_rom, process, Emulator};

fn main() -> Result<()> {
    color_eyre::install()?; // error hooks
    println!("🧨 Initializing emulator");
    let mut emu: Emulator = Emulator::new();

    println!("\t🖊️ Loading fonts into emulator...");
    let _ = load_font(&mut emu);

    let rom_path: &str = "../roms/maze.ch8";
    println!("\t👁️ Reading rom {}...", rom_path);
    let rom_data = std::fs::read(rom_path)?;
    emu.rom_buffer = rom_data;

    println!("\t🕹️ Loading rom into emulator...");
    let _ = load_rom(&mut emu); // clears emu.rom_buffer

    println!("\t🖥️ Initializing terminal...");
    let mut terminal = tui::init()?;

    println!("\t🏃 Running app...");

    loop {
        let _ = fetch_opcode(&mut emu);
        if let Err(err) = process(&mut emu) {
            eprintln!("failed to process.: {}", err);
            break;
        }
        gpu::App::default().run(&mut terminal)?;

        // Trying to figure out how to have above return a fn ptr
        // display
        // input

        // add a conditioal break
        break;
    }

    if let Err(err) = tui::restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }

    println!("🍸 Exiting...");
    Ok(())
}
