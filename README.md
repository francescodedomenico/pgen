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
                                       parameter is ignored. In case you would like to use
                                       whitespaces in your dictionary ' ' use double quotes: -d "aAb
                                       _1"
    -h, --help                         Print help information
    -j, --json                         Optional: passwords output as JSON
    -l, --length <LENGTH>              The desided length for the password
    -n, --number <NUMBER>              Number of passwords to be generated
    -o, --output-file <OUTPUT_FILE>    Optional: writes the generated password/passwords into the
                                       specified file in the current working directory
    -V, --version                      Print version information
```
By default the tool outputs on the standard output:
```shell
$ pgen  -l 16 -c 3 -n 10
```
```shell
### PASSWORD 1 ###
XgukQOfK9uOwVltM

### PASSWORD 2 ###
OwjkI0lP30pUAcqN

### PASSWORD 3 ###
25MldCmBJuATkeWw

### PASSWORD 4 ###
yqQo43jJwVOSmocU

### PASSWORD 5 ###
3gNAz3aOOj3W8xHo

### PASSWORD 6 ###
6lyLVJtTsyzUBIh4

### PASSWORD 7 ###
DBXgTECBfhkCe6zO

### PASSWORD 8 ###
35CTwUmLyIIbUESA

### PASSWORD 9 ###
ImAMvPP1hKJfEGS4

### PASSWORD 10 ###
hjQCTUSmRhr9yjrW
```
It is possible to make an easily parsable JSON adding the -j flag

```shell
$ pgen -l 16 -c 4 -n 10 -j
```
```json
[
  "zZ$3D?(|Q|C\\1W0P",
  "W6'QIY9WG='tmASX",
  "tZF$CXQg6j03$xB)",
  "Sg70kr3H\"p%75PZ5",
  "&2ZZjMpyy0CP6B=5",
  "8&iV8nTkC~7e/t4E",
  "/p7MbeZKJ5JfZh&U",
  "UUJAmn)=C^%IR0hg",
  "fqyRRijil$vtg\\P8",
  "??8Z\"w88&u5yVRaZ"
]
```
you may also flush the buffer into a file written in the current work directory:
```shell
$ pgen -l 16 -c 4 -n 10 -j -o passwords.json
```

