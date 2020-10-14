# cr6502emu


## License

cr6502emu is under GNU AGPLv3, not to be confused with GPLv3.

The most important aspect of this license is that **anyone** that uses this project, **must** have access to the source code, including any changes that you have made.


## Project Structure

This project is primarily composed of 3 modules:

1. **assembler**: Assembles and disassembles code. **(This module is really messy and should be rewritten)** (Rust)
2. **system**: Which does all the actual processing/emulation. (Rust)
3. **html_gui**: That provides a user interface and joins all the other components. (Vue, HTML, CSS, JS)

## Getting Started

### Prerequisites

- [Rust Lang + Cargo](https://www.rust-lang.org/tools/install)
- [Wasm Bindgen + Wasm-Pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Vue JS](https://vuejs.org/v2/guide/installation.html)

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

