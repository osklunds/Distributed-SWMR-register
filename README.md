
# Distributed-SWMR-registers

The ABD algorithm allows a set of distributed processors, that communicate with messages (i.e. the Internet) to emulate shared registers. The registers are so called *single-writer* *multi-reader* registers. This means that every processor can read every other processor's register, but only the owning processor of a register can write to it.

The goal of this project is for me to become a better Rust programmer. For another project I have used the excellent [SSPBFT framework](https://github.com/sspbft/BFTList), which served as an inspiration for this project. I wanted to create something similar to SSPBFT but in Rust.

TODO: Add a longer description.
