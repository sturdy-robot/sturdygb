<!--
SPDX-FileCopyrightText: 2024 Pedrenrique G. Guimarães <pedrenriquegg@hotmail.com>

SPDX-License-Identifier: MIT
-->

<div align="center">

![Sturdy GB logo](images/sturdygb_logo.svg)

[![Rust](https://img.shields.io/badge/Rust-CC342D?style=flat&logo=rust&logoColor=white)](https://rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/license/mit)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://GitHub.com/sturdy-robot/sturdygb/graphs/commit-activity)
[![Last commit](https://img.shields.io/github/last-commit/sturdy-robot/sturdygb)](https://github.com/sturdy-robot/sturdygb/commits/main)
[![GitHub contributors](https://img.shields.io/github/contributors/sturdy-robot/sturdygb)](https://GitHub.com/sturdy-robot/sturdygb/graphs/contributors/)


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

This project is still in **early development**. It can already run many commercial **DMG (original Game Boy)** titles, but several subsystems are incomplete or approximate.

SturdyGB is primarily intended as:
- a learning and research project,
- a testbed for emulator accuracy,
- and a clean Rust codebase for exploring Game Boy internals.

It is **not yet** a drop-in replacement for mature emulators.

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

> Some games that rely on mid-scanline PPU tricks or precise STAT timing may show graphical issues.

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
- Uses **[Notan](https://github.com/Nazariglez/notan)** for rendering and input
- Pixel-perfect nearest-neighbor scaling
- No native dependencies (SDL removed)

## Missing / Incomplete Features

- Inaccurate PPU
- Game Boy Color (CGB) support
- Super Game Boy (SGB) features
- Save states
- Debugger UI
- Cheats/GameShark
- Rewind
- Customizable keys

## Building and running from GitHub

You can find release builds [here](https://github.com/sturdy-robot/sturdygb/releases).

To build from source, just use:

```bash
cargo build
```

And you can run it simply with:

```bash
cargo run <rom-name.gb>
```

## How to run

Since we don't have a fully fledged frontend yet, you have to run it from the command line:

```bash
sturdygb <rom-name.gb>
```

## Keys

Keys are not yet customizable, and they are hardcoded as follows:

| Key | Action |
| --- | --- |
| Arrow Up | Up |
| Arrow Down | Down |
| Arrow Left | Left |
| Arrow Right | Right |
| Z | A |
| X | B |
| Return/Enter | Start |
| Right Shift | Select |

## Roadmap

Planned future work includes:

- Pixel-FIFO-based, cycle-accurate PPU
- Correct STAT interrupt edge behavior
- Full APU implementation
- Game Boy Color (CGB) mode
- Debugging tools (PPU viewer, memory viewer, breakpoints)
- Save states and battery-backed RAM (to save your games)
- Libretro core
- Android port (?)
- Fully functional standalone frontend
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

## 📜 License

    Copyright © 2022-2024 Pedrenrique G. Guimarães

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