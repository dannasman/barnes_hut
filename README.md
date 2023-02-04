# barnes_hut
![ci](https://github.com/dannasman/barnes_hut/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

Barnes-Hut algorithm written in Rust.

## How to use
Use the algorithm on particles in file `foo.txt` that contains particle positions and charges in the form of `x y z q`. Run the algorithm by writing the following line to console:
```
cargo run -- foo.txt
```
