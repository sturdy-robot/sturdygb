<!--
SPDX-FileCopyrightText: 2026 Pedrenrique G. Guimarães <pedrenriquegg@hotmail.com>

SPDX-License-Identifier: MIT
-->

<div align="center">

![Sturdy GB logo](images/sturdygb_logo.svg)

[![Rust](https://img.shields.io/badge/Rust-CC342D?style=flat&logo=rust&logoColor=white)](https://rust-lang.org/)
[![GitHub release](https://img.shields.io/github/v/release/sturdy-robot/sturdygb)](https://github.com/sturdy-robot/sturdygb/releases)
[![GitHub Actions build status](https://img.shields.io/github/actions/workflow/status/sturdy-robot/sturdygb/release.yml)](https://github.com/sturdy-robot/sturdygb/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/license/mit)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://GitHub.com/sturdy-robot/sturdygb/graphs/commit-activity)
[![Last commit](https://img.shields.io/github/last-commit/sturdy-robot/sturdygb)](https://github.com/sturdy-robot/sturdygb/commits/main)
[![GitHub contributors](https://img.shields.io/github/contributors/sturdy-robot/sturdygb)](https://GitHub.com/sturdy-robot/sturdygb/graphs/contributors/)

---

<table>
<tr>
<td>
<img width="300" src="https://github.com/user-attachments/assets/e96bb414-5d8e-49df-9d4e-22e15f0cbe34" />
</td>
<td>
<img width="300"src="https://github.com/user-attachments/assets/07143e0d-e74b-461d-8976-58e5b7cc4479" />
</td>
</tr>
<tr>
<td>
<img width="300" src="https://github.com/user-attachments/assets/b63ebce0-78d8-4627-8703-b4b465681274" />
<td>
<img width="300" src="https://github.com/user-attachments/assets/6a5766bc-d829-418a-a81f-362012a75168" />
</tr>
</table>

</div>

**SturdyGB** is an experimental **Game Boy emulator written in Rust**, focused on correctness, clean architecture, and long-term accuracy.

This project is still in **early development**. It can already run many commercial **DMG (original Game Boy)** titles.

## Current Features

### Core Emulation

- LR35902 CPU
  - Passes **blargg’s CPU instruction tests**
- Timer
- Interrupt controller
- Memory bus
- Serial I/O

### Video (PPU)

- Functional DMG PPU
- Background, window, and sprite rendering
- OAM DMA
- Scanline-based renderer (not cycle-accurate yet)

> [!WARNING]
> Some games may show graphical issues due to the inaccurate PPU.

### Input

- Joypad emulation
- Keyboard input in frontend

### Cartridge Support

- ROM-only
- MBC1
- MBC2
- MBC3
- MBC5
- MBC7
- MM01 (not implemented yet)
- Save games supported.

### Audio (APU)

- Sound channels
- Wave pattern generation
- Volume envelope
- Sound output

## Frontend

- **Pure Rust frontend**
- Uses **[egui (eframe)](https://github.com/emilk/egui/tree/main/crates/eframe)** for rendering and input
- Pixel-perfect nearest-neighbor scaling
- No native dependencies
- Simple frontend with `egui`

## Missing / Incomplete Features

- Inaccurate PPU
- Game Boy Color (CGB) support
- Super Game Boy (SGB) features
- Save states
- Debugger UI
- Cheats/GameShark
- Rewind
- Customizable keys

## Building and running from source

You can find release builds [here](https://github.com/sturdy-robot/sturdygb/releases).

Officially supported architectures:

- aarch64-apple-darwin
- aarch64-pc-windows-msvc
- aarch64-unknown-linux-gnu
- x86_64-apple-darwin
- x86_64-pc-windows-msvc
- x86_64-unknown-linux-gnu

### Prerequisites

To build SturdyGB, you need the Rust toolchain installed. You can install it via [rustup](https://rustup.rs/).

### Desktop Build

To build the native desktop application from source, run:

```bash
cargo build --release
```

You can run the emulator directly using:

```bash
# Run without a ROM
cargo run --release --bin sturdygb_bin

# Run with a specific ROM
cargo run --release --bin sturdygb_bin <rom-name.gb>
```

### WebAssembly (WASM) Build

SturdyGB can also be compiled to run in a web browser using WebAssembly.

1. **Install the `wasm32-unknown-unknown` target:**

   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **Install `wasm-bindgen-cli`:**
   Make sure the version matches the one in `Cargo.toml`.

   ```bash
   cargo install wasm-bindgen-cli
   ```

3. **Build the WebAssembly target:**

   ```bash
   cd crates/frontend
   cargo build --target wasm32-unknown-unknown --release
   ```

4. **Generate the WASM bindings:**

   ```bash
   wasm-bindgen --out-dir public/pkg --target web ../../target/wasm32-unknown-unknown/release/sturdygb.wasm
   ```

5. **Serve the application:**
   You will need a local web server to serve the files in the `crates/frontend/public` directory. For example, using Python:
   ```bash
   cd public
   python -m http.server 8080
   ```
   Then navigate to `http://localhost:8080` in your web browser.

## Keys

The default keys are:

| Key          | Action |
| ------------ | ------ |
| Arrow Up     | Up     |
| Arrow Down   | Down   |
| Arrow Left   | Left   |
| Arrow Right  | Right  |
| Z            | A      |
| X            | B      |
| Return/Enter | Start  |
| Space        | Select |

You can customize them in the UI (no joypad support yet).

## Roadmap

Planned future work includes:

- Pixel-FIFO-based, cycle-accurate PPU
- Correct STAT interrupt edge behavior
- Game Boy Color (CGB) mode
- Debugging tools (PPU viewer, memory viewer, breakpoints)
- Save states
- Libretro core
- Android port (?)
- Customizable keys
- Rewind

## Reference Material

This project is based on research and documentation from the Game Boy reverse-engineering community.

Special thanks to the authors and maintainers of:

- **Game Boy opcode tables**  
  https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html  
  https://gbdev.io/gb-opcodes/optables/

- **Game Boy CPU Manual**  
  http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf

- **The Cycle-Accurate Game Boy Docs**  
  https://github.com/geaz/emu-gameboy/blob/master/docs/The%20Cycle-Accurate%20Game%20Boy%20Docs.pdf

- **PyBoy documentation**  
  https://github.com/Baekalfen/PyBoy/blob/master/PyBoy.pdf

- **SameBoy**  
  https://github.com/LIJI32/SameBoy

- **Gambatte**  
  https://github.com/pokemon-speedrunning/gambatte-core

- **Rboy**  
  https://github.com/mvdnes/rboy

- **mGBA**  
  https://github.com/mgba-emu/mgba

- **BGB**  
  https://bgb.bircd.org/

- **GameBoy-Online**  
  https://github.com/taisel/GameBoy-Online

- **Low Level Devel – Game Boy series**  
  https://www.youtube.com/watch?v=e87qKixKFME

- **GameBoy Doctor**  
  https://github.com/robert/gameboy-doctor

## Disclaimer

SturdyGB is an **early technical project**.

Accuracy, structure, and experimentation are prioritized over performance, UX, or completeness. Expect bugs, missing features, and breaking changes.

## License

    Copyright © 2022-2026 Pedrenrique G. Guimarães

    Permission is hereby granted, free of charge, to any person
    obtaining a copy of this software and associated documentation
    files (the “Software”), to deal in the Software without
    restriction, including without limitation the rights to use,
    copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following
    conditions:

    The above copyright notice and this permission notice shall be
    included in all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
    OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
    NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
    HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
    WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
    OTHER DEALINGS IN THE SOFTWARE.

The project is licensed under [MIT](LICENSE.md) license.
