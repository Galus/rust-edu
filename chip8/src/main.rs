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

    println!("\t🖊️ Loading fonts into emulator...");
    let _ = load_font(&mut emu);

    let rom_path: &str = "maze.ch8";
    println!("\t👁️ Reading rom {}...", rom_path);
    let rom_data: Vec<u8> = std::fs::read(rom_path).unwrap();
    emu.rom_buffer = rom_data.clone();

    println!("\t🕹️ Loading rom into emulator...");
    let _ = load_rom(&mut emu); // clears emu.rom_buffer

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
