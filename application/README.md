
# Application - The code for an ABD node

This directory contains the code for an instance of an ABD node. Make sure to change your current directory to the `application` directory. If you type `cargo run -- --help` you will see the following:

```
Distributed-SWMR-registers: Application 0.1
Oskar Lundström
The application code, that is an instance of an ABD node.

USAGE:
    distributed_swmr_registers [FLAGS] <node-id> <hosts-file> <run-length> [color]

FLAGS:
    -h, --help                       Prints help information
    -p, --print-client-operations    Print when a read/write operation starts/ends. If not included, the performance
                                     might be slightly higher.
    -r, --read                       Makes this node perform read operations.
    -e, --record-evaluation-info     Record information used for the evaluation, such as latency and number of messages
                                     sent. If not included, the performance might be slightly higher.
    -V, --version                    Prints version information
    -w, --write                      Makes this node perform write operations.

ARGS:
    <node-id>       The integer id of this node instance.
    <hosts-file>    The file with host ids, addresses and ports.
    <run-length>    The number of seconds the program should run for. If 0 is given, the program will run until
                    aborted with Ctrl+C.
    <color>         The color of the terminal output [default: Black]  [possible values: Black, Red, Green, Yellow,
                    Blue, Magenta, Cyan]
```

The idea is that you create a hosts file with all the hosts you want to be part of the system. Then you copy this source code to all the hosts and specify the the above arguments to your liking. Doing it like this manually for each node is certainly possible, but it's not very convenient. Therefore I have the tools `local_starter` (for running multiple nodes on your own computer) and `remote_starter` (for running multiple nodes on different computers). Check out the readmes of those two for more information about them.

## Code overview

The entry point of the program is `main()` in the `main.rs` file. `main()` creates an instance of `Mediator` and spawns two threads for read and write operations respectively.

`Mediator` is the core of the program and wires together an `AbdNode` and a `Communicator`. `Communicator` has a UDP socket that it receives from on a background thread. It also allows other structs to send UDP messages with it. `AbdNode` is the implementation of the ABD algorithm. `AbdNode` and `Communicator` don't interact with each other directly. All interactions happen through the `Mediator`.

## Real-world usage of the code

The application code, as of now, just writes and reads to the shared registers. Not very useful. But it's just for demonstration. To use the code for your own application, you create an instance of `Mediator` by supplying it the hosts of your system. Then you can call `write()` and `read()` on it, to let your application operatate on the shared registers. The shared registers is like a lower layer that your application runs on top of. `main.rs` can be seen as the current application, an application that just writes and reads in order to measure the performance.
