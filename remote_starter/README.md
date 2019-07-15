
# Remote starter - A helper utilty that starts multiple nodes on your local computer

This directory contains the code for a helper utility for starting multiple nodes on remote computers via SSH. Make sure to change your current directory to the `remote_starter` directory. If you type `cargo run -- --help` you will see the following:

```
Distributed SWMR registers: Remote starter 0.1
Oskar Lundström
A helper utility that starts multiple nodes on remote machines via SSH.

USAGE:
    remote_starter [FLAGS] [OPTIONS] <hosts-file>

FLAGS:
    -h, --help                       Prints help information
    -i, --install                    With this option, Rust will be installed, the source code and configuration files
                                     will be uploaded and the application will be built. Without this option, the
                                     application will be launched.
    -o, --optimize                   With this option, cargo will build/run in release mode. This uses optimizations and
                                     yields higher performance.
    -p, --print-client-operations    Print when a read/write operation starts/ends. If not included, the performance
                                     might be slightly higher.
    -e, --record-evaluation-info     Record information used for the evaluation, such as latency and number of messages
                                     sent. If not done, the performance might be slightly higher.
    -V, --version                    Prints version information

OPTIONS:
    -r, --number-of-readers <number-of-readers>    The number of nodes that should read. [default: 0]
    -w, --number-of-writers <number-of-writers>    The number of nodes that should write. [default: 0]
    -l, --run-length <run-length>
            The number of seconds the program should run for. If 0 is given, the program will until aborted with Ctrl-C.
            [default: 0]

ARGS:
    <hosts-file>    The file with node ids, addresses, ports, ssh key paths and usernames
```

The idea is that you use this utility when you want to run the application on multiple networked machines.

## Usage

The `--install` option will install Rust, upload the source code and hosts file and build the source code. You need to use `--install` every time you change the source code and what it updated on the remote computers. If you want to run the code, make sure to not specify `--install`. You can run the code multiple times, with different command line arguments, without reinstalling every time.