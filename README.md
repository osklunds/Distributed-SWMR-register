
# Distributed-SWMR-registers

The ABD algorithm allows a set of distributed processors, that communicate with a message-passing system (i.e. the Internet), to emulate shared registers. The registers are so called *single-writer* *multi-reader* registers. This means that every processor can read every other processor's register, but only the owner of a register can write to it.

The goal of this project is for me to become a better Rust programmer. For another project I have used the excellent [SSPBFT framework](https://github.com/sspbft/BFTList), which serves as an inspiration for this project. I wanted to create something similar to SSPBFT but in Rust.

## How to run

1. [Install Rust](https://www.rust-lang.org/tools/install).
2. Clone this repository.
3. Uncomment the `printlnu` lines in `application/src/main.rs` to actually see something printed to the terminal.
4. Change directory to `local_starter` and type `cargo run n` where `n` is the number of local nodes you want to have. I suggest `n=5`.

## Repository overview

The `application` directory contains the code for an instance of an ABD node. On each computer you want to be part of this network, you run the code in this directory. More details are in `application/README.md`.

The `local_starter` directory contains the code for a helper tool. `local_starter` automatically starts the user-supplied number of ABD nodes on the local machine, to simplify testing of the code. Note that `local_starter` is purely for convenience. `application` works as a standalone program. More details are in `local_starter/README.md`.