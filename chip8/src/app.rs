// A nice wrapper so we dont have to pollute main with low level stuffs.
use color_eyre::Result;

use crate::cpu;
use crate::gpu;
use crate::tui;
use cpu::{fetch_opcode, load_font, load_rom, process, Emulator};

/// Creates a terminal and runs a loop that processes instructions, renders to ui, and handles
/// events
pub fn run() -> Result<()> {
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

    println!("\t🖥️ Initializing terminal...");
    let mut terminal = tui::init()?;

    println!("\t🏃 Running app...");

    //let mut counter = 0;
    loop {
        //counter += 1;
        //println!("Counter {:?}", counter);

        let _ = fetch_opcode(&mut emu);
        if let Err(err) = process(&mut emu) {
            eprintln!("failed to process.: {}", err);
            break;
        }
        let _app_result = gpu::App::default().run(&mut terminal);

        // Trying to figure out how to have above return a fn ptr
        // display
        // input

        break;
    }

    if let Err(err) = tui::restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }
    //emu.print_memory();
    //app_result
    Ok(())
}
