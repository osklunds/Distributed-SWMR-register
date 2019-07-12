
# Application - The code for an ABD node

This directory contains the code for an instance of an ABD node. Make sure to change your current directory to the `application` directory. If you type `cargo run -- --help` you will see the following:

```
Distributed SWMR registers 0.1
Oskar Lundström
Todo

USAGE:
    distributed_swmr_registers <node-id> <hosts-file> [color]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <node-id>       The integer id of this node instance.
    <hosts-file>    The file with host ids, addresses and ports.
    <color>         Sets the color of the terminal output [default: Black]  
                    [possible values: Black, Red, Green, Yellow, Blue, Magenta, Cyan]
```

In other words, to start an instance, you supply the program the node id, a text file with the hosts and optinally a color that will be used for terminal output. The text file should contain the ids and socket addresses of all nodes used. An example is found in `hosts_example.txt`. The color option is mostly useful for `local_starter`.

## Code overview

The entry point of the program is `main()` in the `main.rs` file. `main()` creates an instance of `Mediator` and spawns two threads for read and write operations respectively.

`Mediator` is the core of the program and wires together an `AbdNode` and a `Communicator`. `Communicator` has a UDP socket that it receives from on a background thread. It also allows other structs to send UDP messages with it. `AbdNode` is the implementation of the ABD algorithm. `AbdNode` and `Communicator` don't interact with each other directly. All interactions happens through the `Mediator`.



