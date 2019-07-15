
# Local starter - A helper utilty that starts multiple nodes on your local computer

This directory contains the code for a helper utility for starting multiple nodes on your local computer. Make sure to change your current directory to the `local_starter` directory. If you type `cargo run -- --help` you will see the following:

```
Distributed-SWMR-registers: Local starter 0.1
Oskar Lundström
A helper utility that starts multiple nodes on your local computer.

USAGE:
    local_starter [FLAGS] [OPTIONS] --number-of-nodes <number-of-nodes>

FLAGS:
    -h, --help                       Prints help information
    -o, --optimize                   With this option, cargo will build/run in release mode. This uses optimizations and
                                     yields higher performance.
    -p, --print-client-operations    Print when a read/write operation starts/ends. If not included, the performance
                                     might be slightly higher.
    -e, --record-evaluation-info     Record information used for the evaluation, such as latency and number of messages
                                     sent. If not included, the performance might be slightly higher.
    -V, --version                    Prints version information

OPTIONS:
    -n, --number-of-nodes <number-of-nodes>        The number of local nodes to run.
    -r, --number-of-readers <number-of-readers>    The number of nodes that should read. [default: 0]
    -w, --number-of-writers <number-of-writers>    The number of nodes that should write. [default: 0]
    -l, --run-length <run-length>
            The number of seconds the program should run for. If 0 is given, the program will until aborted with Ctrl-C.
            [default: 0]
```

The idea is that you use this utility when testing the application locally. With this, you can easily start multiple nodes.

