# CHIP-8 Emulator

This is a CHIP-8 emulator implemented in Rust. CHIP-8 is an interpreted programming language developed in the 1970s, primarily used on 8-bit microcomputers and for creating simple video games.

## Project Structure

The project is organized into the following modules:

- chip8
    - emu
        - cpu
            - gpu
            - memory

[joamag's boytacean gameboy emulator](https://github.com/joamag/boytacean) 
inspired my project layout to funnel all the things into the cpu.

## Features

- Accurate emulation of CHIP-8 instructions
- Display output
- Keyboard input
- Sound support
- Configurable clock speed

## Building and Running

To build and run the emulator, make sure you have Rust installed on your system. Then, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/galus/rust-edu.git
   cd rust-edu/chip8
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the emulator:
   ```
   cargo run --release -- path/to/rom.ch8
   ```

Replace `path/to/rom.ch8` with the path to a CHIP-8 ROM file you want to run.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
