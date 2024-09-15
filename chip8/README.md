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

### Instructions Progress

- [ ] 00E0 - CLS
- [ ] 00EE - RET
- [ ] 0nnn - SYS addr
- [ ] 1nnn - JP addr
- [ ] 2nnn - CALL addr
- [ ] 3xkk - SE Vx, byte
- [ ] 4xkk - SNE Vx, byte
- [ ] 5xy0 - SE Vx, Vy
- [ ] 6xkk - LD Vx, byte
- [ ] 7xkk - ADD Vx, byte
- [ ] 8xy0 - LD Vx, Vy
- [ ] 8xy1 - OR Vx, Vy
- [ ] 8xy2 - AND Vx, Vy
- [ ] 8xy3 - XOR Vx, Vy
- [ ] 8xy4 - ADD Vx, Vy
- [ ] 8xy5 - SUB Vx, Vy
- [ ] 8xy6 - SHR Vx {, Vy}
- [ ] 8xy7 - SUBN Vx, Vy
- [ ] 8xyE - SHL Vx {, Vy}
- [ ] 9xy0 - SNE Vx, Vy
- [ ] Annn - LD I, addr
- [ ] Bnnn - JP V0, addr
- [ ] Cxkk - RND Vx, byte
- [ ] Dxyn - DRW Vx, Vy, nibble
- [ ] Ex9E - SKP Vx
- [ ] ExA1 - SKNP Vx
- [ ] Fx07 - LD Vx, DT
- [ ] Fx0A - LD Vx, K
- [ ] Fx15 - LD DT, Vx
- [ ] Fx18 - LD ST, Vx
- [ ] Fx1E - ADD I, Vx
- [ ] Fx29 - LD F, Vx
- [ ] Fx33 - LD B, Vx
- [ ] Fx55 - LD [I], Vx
- [ ] Fx65 - LD Vx, [I]

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
