# cr6502emu


## License

Before doing anything with the source code, make sure you understand the project license.

cr6502emu is under the GNU AGPLv3, not to be confused with GPLv3.

You can read all the terms at [https://choosealicense.com/licenses/agpl-3.0/](https://choosealicense.com/licenses/agpl-3.0/), but the most important aspect is that **ANYONE** that uses **any** part of this project, **EVEN if it's over the network**, like on a web page, **MUST** have access to the source code, **including** any changes that you have made.


## Project Structure

This project is primarily composed of 3 modules:

1. **assembler**: Assembles and disassembles code. (Rust)
2. **system**: Which does all the actual processing/emulation. (Rust)
3. **html_gui**: That provides a user interface and joins all the other components. (HTML, CSS, JS)

## Getting Started

### Prerequisites

- [Rust Lang + Cargo (Installer)](https://www.rust-lang.org/tools/install)
- [Wasm Bindgen + Wasm-Pack (Installer)](https://rustwasm.github.io/wasm-pack/installer/)

### Compiling

#### Using project scripts (UNIX-like OS only)
    $ cd system
    $ ../scripts/build_debug.sh
    
#### Using wasm-pack
    $ wasm-pack build system

## Built With

- [UIKit CSS](https://github.com/uikit/uikit)
- [Vue](https://github.com/vuejs/vue)


- [Rust Lang](https://github.com/rust-lang/rust)
- [Rust Wasm-Bindgen](https://github.com/rustwasm/wasm-bindgen)
- [WebAssembly (Wasm)](https://webassembly.org/)

