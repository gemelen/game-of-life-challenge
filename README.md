# game-of-life-challenge

Combination of ready-to-go implementation for a B2S23 variation of a Conway's Game of Life and a bit of terminal-based UI.

Game is configured to be run on a 25x25 grid, with a seed of a Glider patter placed in a center of the grid.

Executables are provided for Linux x86-64 and MacOS ARM64. For any other platform you'd need to build it by yourself. To do so you need to have a [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) package manager and run `cargo build --release` in a top level directory with the sources of this project. This will create a binary for you platform in a `target/release` directory (if nothing goes wrong).
