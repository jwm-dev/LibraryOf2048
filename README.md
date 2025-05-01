# LibraryOf2048  
![Build Status](https://img.shields.io/badge/build-passing-brightgreen) ![License](https://img.shields.io/badge/license-MIT-blue)

## Overview

**LibraryOf2048** is a Rust-based application that catalogs and indexes all ~117 quintillion possible board states of the classic 2048 game. Inspired by Jonathan Basile’s *Library of Babel*, this project provides a deterministic, navigable archive of all valid 4×4 tile configurations using a two-part keying system that enables efficient enumeration and lookup. The project includes a GUI interface, and serves as a platform for further research into solving the stochastic dynamics of 2048, a PSPACE-complete challenge.

![Demo](https://i.imgur.com/9y3PaSN.gif)

## Core Concepts

The enumeration system is based on a dual-ID system and t-value, forming an efficient data triplet that distinguishes each individual board:

### 1. **T-Value (Tile Count)**
- Represents how many tiles are on the board.
- Valid `t` values range from 2 to 16.
- Boards with `t=0` or `t=1` are not used, as the game always starts with 2 tiles and never decreases tile count below that value.

### 2. **Global ID (Tile Placement)**
- Encodes the positions of tiles on the 4×4 board.
- For a given `t`, the global ID is a linear index across all  $\binom{16}{t} - 17$ permutations of tile placements (17 is the number of states t=0 and t=1 account for).
- Ranges from 1 to 65519, accounting for all valid tile-position combinations.

### 3. **Local ID (Tile Values)**
- Encodes the values of each tile in a board configuration.
- Uses base-11 notation to represent tile values \([2, 4, ..., 2048]\), mapped as digits representing powers of 2: \([1, 2, ..., A]\).
- The length of the local ID equals the `t` value; each digit corresponds to a tile's value, applied in left-to-right order on the board as dictated by the Global ID.

### Example:
If `t=2`, Global ID = `119`, and Local ID = `AA`:
- The board has two tiles.
- Their positions are derived from Global ID 119.
- Each tile is a 1024-tile (2^10 = 1024).

This systematic encoding allows precise generation, lookup, and traversal of the 2048 board space—over 1.17e20 (~117 quintillion) possible configurations.

## Features

- 📚 **Full Enumeration** of all 2048-valid board states
- 🔍 **Lookup by ID** — decode or explore by Global + Local IDs
- 🖼️ **GUI Interface** — built with egui, powered by Rust
- 🧠 **Research-Oriented** — supports investigation into PSPACE-level complexity of 2048

## Installation

### Prerequisites
- Rust toolchain (stable)

### Build
```bash
git clone https://github.com/jwm-dev/LibraryOf2048.git
cd LibraryOf2048
cargo build --release
```

## Technical Details

- Language: **Rust**  
- GUI: **egui**  
- Encoding: Base-11 local ID, combinatorial indexing of placements  
- Optimization: Indexed retrieval, fast computation, zero-copy board generation  
- Scope: ~1.17×10²⁰ valid board states

## Inspiration

This project is inspired by Jonathan Basile's *[Library of Babel](https://libraryofbabel.info/)*, which exhaustively indexes all possible combinations of characters in the Latin alphabet. *LibraryOf2048* follows a similar philosophical and technical arc — providing total determinism and structure to an otherwise intractable combinatorial space.

## Future Work

- Solve the stochastic, classic variant of 2048 through AI or formal methods
- Visual analytics for tile distributions and win conditions
- WebAssembly interface for browser-based board browsing

## License

MIT License. See `LICENSE` file.

---

*LibraryOf2048: not just a game, but a universe of structured possibility.*
