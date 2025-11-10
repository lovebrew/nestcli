
# nestcli

`nestcli` is a command-line tool for [LÖVE Potion](https://github.com/lovebrew/lovepotion) games, designed to streamline the development process by providing a command-line interface for managing and connecting to debug targets over a network.

#### Command Line

```
A simple CLI tool for LÖVE Potion games.

Usage: nestcli.exe <COMMAND>

Commands:
  config  Add, remove, or list configured target devices
  debug   Tools for debugging builds and resolving symbols
  bundle  Bundle utilization commands
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Installation

> [!IMPORTANT]
> The project is built using the rust programming language. Ensure that it is installed before trying to build the project.


**1. Clone the repository**

```bash
git clone https://github.com/TurtleP/nestdbg
cd nestdbg
```

**2. Install the binary locally**
```bash
cargo install --path .
```

Alternatively, install the tool through crates.io:
```bash
cargo install nestcli
```
