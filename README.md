# pgen
pgen is a very simple and very fast Rust based command-line password generator.

When installed in your path, becomes a useful tool in order to generate new passwords and secrets.

Currently pgen packs the following features:

*   Arbitrary password length
*   4 predefined password complexities
*   Possibility to output several secrets at once
*   Allows custom dictionaries

## Installation

If you have an installed Rust toolchain you may simply run 

```shell
cargo build -release
```

you'll fine the pgen executable in the /target/release folder. If you move it into your PATH you'll be able to run it as a command.

## Usage

```shell
$ pgen -h
pgen 0.1.0
Francesco De Domenico
A Very simple Password Generator!

USAGE:
    pgen.exe [OPTIONS] --length <LENGTH>

OPTIONS:
    -c, --complexity <COMPLEXITY>      The complexity of the password. Values allowed: 1 - Lowercase
                                       dictionary; 2 - Lowercase and uppercase dictionary; 3 -
                                       Lowercase, uppercase and digits dictionary; 4 - lowercase,
                                       uppercase, digits and symbols dictionary;
    -d, --dictionary <DICTIONARY>      A custom dictionary for the password, if set the complexity
                                       parameter is ignored
    -h, --help                         Print help information
    -j, --json                         Optional: passwords output as JSON
    -l, --length <LENGTH>              The desided length for the password
    -n, --number <NUMBER>              Number of passwords to be generated
    -o, --output-file <OUTPUT_FILE>    Optional: writes the generated password/passwords into the
                                       specified file in the current working directory
    -V, --version                      Print version information
```
