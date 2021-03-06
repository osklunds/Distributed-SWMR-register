
# Remote starter - A helper utilty that starts multiple nodes on remote computers

This directory contains the code for a helper utility for starting multiple nodes on remote computers via SSH. Make sure to change your current directory to the `remote_starter` directory. If you type `cargo run -- --help` you will see the following:

```
Distributed SWMR register: Remote starter 
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
    -w, --write                      If the writer node should write.

OPTIONS:
    -r, --number-of-readers <number-of-readers>
            The number of nodes that should read. If the writer node is instructed to write, the number of readers must
            be at most one less than the total number of nodes. [default: 0]
    -l, --run-length <run-length>
            The number of seconds the program should run for. If 0 is given, the program will run until aborted with
            Ctrl-C. [default: 0]

ARGS:
    <hosts-file>    The file with node ids, addresses, ports, ssh key paths and usernames.

```

The idea is that you use this utility when you want to run the application on multiple networked machines.


**IMPORTANT: READ THE NOTES BELOW, ESPECIALLY THE LAST ONES.**

## Notes for usage

### Installing

The `--install` option will install Rust, upload the source code and hosts file and build the source code. You need to use `--install` every time you change the source code and what it updated on the remote computers. If you want to run the code, make sure to not specify `--install`. You can run the code multiple times, with different command line arguments, without reinstalling every time. You must provide the `--variant` argument even for install, but it doesn't matter which value you use.


### Hosts file

An example of what the hosts file should look like is found in `hosts_example.txt` and is also found below:

```
1,192.168.1.17:62001,~/.ssh/id_rsa,my_cool_user_name,scripts/rust_only.sh
2,192.168.1.17:62002,~/.ssh/id_rsa,my_cool_user_name,scripts/rust_only.sh
3,192.168.1.18:62001,~/.ssh/id_rsa,my_cool_user_name,scripts/rust_only.sh
```

From left to right: Node id, ip address:port number, path to ssh key file, username on the remote computer, path to the install script.


### Install script

The install script is a bash script (no Rust unfortunately) that contains what is required to install Rust on the corresponding computer. For most normal machines, running `rustup.sh` is enough, which is precisely what `rust_only.sh` will do. But for PlanetLab, build tools must also be installed. This is what `planet_lab.sh` contains.


### Node ids

The application itself supports arbitrary numbers as node ids. But for the remote starter, you should use the number 1,2,3,...,n if you have n nodes. The reason is that the remote starter uses the node ids to determine which nodes should write and read, and assumes they follow the previosuly mentioned pattern.


### Security

To make evaluation on PlanetLab convenient, `remote_starter` does not ask you to verify the fingerprint when connecting to a new ssh server. However, if the fingerprint has changed, then it will warn you. So if you care about security you need to first manually connect to all used ssh servers so that their fingerprints are recorded.


### Processes and users

When `remote_starter` is exited, it will run `pkill -u <username>` on all remote ssh servers, where `username` comes from the hosts file. This means all processes of that user are killed. As you can imagine, if not done in a controlled way, this could be bad. Therefore I recommened to have a dedicated user account just for running this code. This is no problem on PlanetLab.