# SturdyGB

An experimental GameBoy emulator written in Rust.

## Plans

The plan is to be a fully compatible GameBoy emulator that runs GB and GBC games.

Checklist of (not yet) implemented features:

- [x] CPU
- [ ] GPU
- [ ] Timer
- [ ] Emulate all versions of the GameBoy (DMG, SGB, MGB, GBC)
- [x] Cartridge
    - [x] ROMONLY
    - [x] MBC1
    - [ ] MM01
    - [ ] MBC2
    - [ ] MBC3
    - [ ] MBC5
    - [ ] MBC7
- [x] Memory Bus
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

## Reference material

This project is based on the research and documentation made by giants on the GameBoy reverse-engineering scene.

Thanks to all the following documentation and repos, I was able to test and debug my emulator properly:

- **GameBoy Opcodes tables**: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html and https://gbdev.io/gb-opcodes/optables/
- [**GameBoy CPU Manual**](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf)
- [**The Cycle-Accurate GameBoy Docs**](https://github.com/geaz/emu-gameboy/blob/master/docs/The%20Cycle-Accurate%20Game%20Boy%20Docs.pdf)
- [**Emulation of the Nintendo GameBoy (DMG-01) (PyBoy)**](https://github.com/Baekalfen/PyBoy/blob/master/PyBoy.pdf)
- [**SameBoy**](https://github.com/LIJI32/SameBoy): the SameBoy emulator is incredibly accurate and was a greate resource for learning and testing my own emulator and test my implementation. Much of the implementation details in the opcodes were based on SameBoy's implementation.
- [**Rboy**](https://github.com/mvdnes/rboy): this project helped me out early on to understand how to implement a GameBoy emulator. I started this project with this implementation in mind, but later I decided to rewrite everything from scratch, taking what I learned about Rust and the GameBoy from this repo.
- [**Gambatte**](https://github.com/pokemon-speedrunning/gambatte-core): seeing how others implemented the same thing was a great source of inspiration. Gambatte is also pretty accurate, and helped me understand a few things that I could not grasp from SameBoy.
- [**mGBA**](https://github.com/mgba-emu/mgba): mGBA is not only a GBA emulator, but it also contains a GameBoy core inside it, and it was pretty helpful overall.
- [**BGB**](https://bgb.bircd.org/): the BGB emulator, although not open source, has an awesome debugger. Not only it can emulate several GameBoy versions, but the debugger allows me to see every register and values in real-time. It was an awesome resource for comparing implementations and testing my emulator's accuracy.
- [**GameBoy-Online**](https://github.com/taisel/GameBoy-Online): I discovered this emulator when looking at the [GB-Studio](https://github.com/chrismaltby/gb-studio) project. This is the emulator they used for GB-Studio, and when I was implementing some opcodes originally, I was looking at this implementation to understand what the hell I was doing.


## License

    Copyright © 2022 Pedrenrique G. Guimarães

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

The project is licensed under [MIT](LICENSE.md).
