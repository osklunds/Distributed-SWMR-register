
# Evaluator - A helper utilty that gathers evaluation results and aggregates them

This directory contains the code for a helper utility that gathers evaluation results from running the code on multiple remote machines and also aggregates the results. Make sure to change your current directory to the `evaluator` directory. If you type `cargo run -- --help` you will see the following:

```
Distributed SWMR registers: Evaluator 
A helper utilty that gathers evaluation results and aggregates them

USAGE:
    evaluator [SUBCOMMAND]

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    aggregate    Will aggregate multiple result-files to generate aggregated results, according to what you have
                 programatically defined.
    gather       Will run each scenario ones and gather the results in a file. The results-file will be built upon,
                 and if a scenario already exists there, it will not be run again.
    help         Prints this message or the help of the given subcommand(s)
    install      Will install Rust and the source code on the (remote) hosts.
```

We see that Evaluator has the three subcommands `aggregate`, `gather` and `install`.

`cargo run -- aggregate --help`:

```
evaluator-aggregate 
Will aggregate multiple result-files to generate aggregated results, according to what you have programatically defined.

USAGE:
    evaluator aggregate <result-files>

FLAGS:
    -h, --help    Prints help information

ARGS:
    <result-files>    The files with results. Each file should have the same scenarios as the other files.
```

`cargo run -- gather --help`:

```
evaluator-gather 
Will run each scenario ones and gather the results in a file. The results-file will be built upon, and if a scenario
already exists there, it will not be run again.

USAGE:
    evaluator gather [FLAGS] <hosts-file> <scenario-file> <result-file>

FLAGS:
    -h, --help                       Prints help information
    -o, --optimize                   With this option, cargo will run in release mode. This uses optimizations and
                                     yields higher performance.
    -p, --print-client-operations    Print when a read/write operation starts/ends. If not included, the performance
                                     might be slightly higher.

ARGS:
    <hosts-file>       The file with node ids, addresses, ports, ssh key paths and usernames.
    <scenario-file>    The file with scenarios to run.
    <result-file>      The file in which the results are stored.
```

`cargo run -- install --help`:

```
evaluator-install 
Will install Rust and the source code on the (remote) hosts.

USAGE:
    evaluator install [FLAGS] <hosts-file>

FLAGS:
    -h, --help        Prints help information
    -o, --optimize    With this option, cargo will build the application in release mode. This uses optimizations and
                      yields higher performance.

ARGS:
    <hosts-file>    The file with node ids, addresses, ports, ssh key paths and usernames.
```

The indented workflow is as follows:

1. Create the hosts file.
2. Use the `install` command to install Rust and the source code on the hosts.
3. Create the scenario file.
4. Use the `gather` command to run all scenarios.
5. Stash away the result file if you want each scenario to be run more than once.
6. Repeat step 4 and 5 until you have the number of results you want.
7. Define programtically (you have to change the source code) functions on the result data, for example, the average write latency, that you are intersted in.
8. Use the `aggregate` command to run your code on the result data.

Todo: have an example implementation for step 7 and 8.

## Note for usage

Check the notes for usage of `remote_starter`. Those notes apply here as well. In addition there are the notes below:

### Scenario file

An example of what the scenario file should look like is found in `scenarios_example.txt` and is also found below:

```
3,3,3
3,1,1
3,2,2
3,0,3
```

From left to right: number of nodes, number of readers, number of writers. Each line is a scenario.

### Result files

Evaluator will download files with names `node001.eval`, `node002.eval`, ... from the remote hosts. You don't need to save or care about these files.

The end result is stored in `results.eval`. It is a json-serialization of a `HashMap`. You shoul't modify it. Keep this file though.
