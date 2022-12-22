# SturdyGB

An experimental GameBoy emulator written in Rust.

## Plans

The plan is to be a fully compatible GameBoy emulator that runs GB and GBC games.

Checklist of (not yet) implemented features:

- [x] CPU
- [ ] GPU
- [ ] Timer
- [x] Cartridge
    - [x] ROMONLY
    - [x] MBC1
    - [ ] MM01
    - [ ] MBC2
    - [ ] MBC3
    - [ ] MBC5
    - [ ] MBC7
- [x] MMU/Memory Bus
- [ ] Audio
- [ ] Joystick and input
- [ ] Load/Save
- [ ] Windowing and display
- [ ] Debugger
- [ ] Cheats and GameShark
- [ ] SaveStates
- [ ] Rewind
- [ ] Libretro core
- [ ] Android port
- [ ] Frontend

## License

    SturdyGB - Experimental GameBoy emulator written in Rust
    Copyright (C) 2022  Pedrenrique G. Guimarães

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.

The project is licensed under [GPLv3](LICENSE.md).
