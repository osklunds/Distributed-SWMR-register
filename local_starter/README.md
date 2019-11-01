
# Local starter - A helper utilty that starts multiple nodes on your local computer

This directory contains the code for a helper utility for starting multiple nodes on your local computer. Make sure to change your current directory to the `local_starter` directory. If you type `cargo run -- --help` you will see the following:

```
Distributed-SWMR-register: Local starter 
A helper utility that starts multiple nodes on your local computer.

USAGE:
    local_starter [FLAGS] [OPTIONS] <number-of-nodes>

FLAGS:
    -h, --help                       Prints help information
    -o, --optimize                   With this option, cargo will build/run in release mode. This uses optimizations and
                                     yields higher performance.
    -p, --print-client-operations    Print when a read/write operation starts/ends. If not included, the performance
                                     might be slightly higher.
    -w, --write                      If the writer node should write.

OPTIONS:
    -r, --number-of-readers <number-of-readers>
            The number of nodes that should read. If the writer node is instructed to write, the number of readers must
            be at most one less than the total number of nodes. [default: 0]
    -l, --run-length <run-length>
            The number of seconds the program should run for. If 0 is given, the program will run until aborted with
            Ctrl-C. [default: 0]

ARGS:
    <number-of-nodes>    The number of local nodes to run.
```

The idea is that you use this utility when testing the application locally. With this, you can easily start multiple nodes.

