# sargs

`sargs` is a command-line tool designed as an alternative to xargs. Unlike xargs, it allows for the execution of subsequent commands in a streaming fashion before the input-side program finishes. This enables real-time data processing, allowing for more efficient workflows.

## Features

- **Asynchronous Execution:** Executes subsequent commands as soon as data becomes available, without waiting for the input-side program to completely finish.
- **High Efficiency:** Processes large amounts of data quickly while keeping memory consumption low, thanks to streaming.
- **Flexibility:** Can be used in combination with various shell environments and command-line tools, catering to a wide range of applications.

## Installation

Currently, `sargs` can be installed using Cargo, the Rust package manager.

```sh
cargo install sargs-cmd
```

## Usage

The basic usage is similar to xargs, but `sargs` differs in that data is passed to the subsequent command as a stream. Below is an example of taking data from standard input and passing each line to the echo command.

```sh
cat example.txt | sargs echo
```

The command above reads each line from `example.txt` and passes it to the `echo` command as it is read, allowing for real-time processing of the file's contents.
The difference in behavior from xargs may not be immediately apparent in this example. 

For instance, if a command that reads one line and waits for one second is inserted in the middle of the cat command, xargs will read all lines and then execute the command for all lines, while sargs will execute the command immediately after reading each line.
```sh
cat example.txt | slow_pass_command | sargs echo
```

## Configuration and Options

`sargs` offers a wide range of customizable options. You can adjust its behavior through command-line options. All options and their descriptions can be accessed using the following command.

```sh
sargs --help

Usage: sargs [OPTIONS] [ARGS]...

Arguments:
  [ARGS]...  

Options:
  -I <INPUT_PLACEHOLDER>           
      --buffer-size <BUFFER_SIZE>  
  -h, --help                       Print help
  -V, --version                    Print version
```

### INPUT_PLACEHOLDER

You can specify a placeholder for the data passed to the subsequent command using the `-I` or `--input-placeholder` option.

```sh
cat example.txt | sargs -I __INPUT__ echo __INPUT__
```

In the command above, `__INPUT__` will be replaced with the contents of each line when passed to the `echo` command.

### BUFFER_SIZE

You can specify the number of buffers for the output command using the `--buffer-size` option. The default is `128`.

## Contributions

`sargs` is an open-source project and welcomes contributions from the community. Feel free to submit bug reports, feature suggestions, and pull requests through the GitHub repository.

## License

`sargs` is released under the [MIT License](https://opensource.org/licenses/MIT). 