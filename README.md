# pgen
pgen is a very simple and very fast Rust based command-line password generator.

When installed in your path, becomes a useful tool in order to generate new passwords and secrets.

Currently pgen packs the following features:

*   Arbitrary password length
*   4 predefined password complexities
*   Possibility to output several secrets at once
*   Allows custom dictionaries
*   Outputs passes as JSON Array

## Installation

Download the archive for your system from the [releases page](https://github.com/francescodedomenico/pgen/releases) (macOS arm64 and x86_64, Linux x86_64 and aarch64 as static binaries, Windows x86_64), check it against the `.sha256` file beside it, and put the `pgen` binary in your `PATH`. Every release also carries a CycloneDX SBOM of the crates it was built from.

If you have an installed Rust toolchain you may instead run

```shell
$ cargo build --release
```

you'll find the pgen executable in the /target/release folder. If you move it into your PATH you'll be able to run it as a command.

## Usage

```shell
$ pgen -h
A Very simple Password Generator!

Usage: pgen [OPTIONS] --length <LENGTH>

Options:
  -l, --length <LENGTH>            The desired length for the password
  -c, --complexity <COMPLEXITY>    The complexity of the password. Values allowed: 1 - Lowercase dictionary; 2 - Lowercase and uppercase dictionary; 3 - Lowercase, uppercase and digits dictionary; 4 - lowercase, uppercase, digits and symbols dictionary;
  -d, --dictionary <DICTIONARY>    A custom dictionary for the password, if set the complexity parameter is ignored. In case you would like to use whitespaces in your dictionary ' ' use double quotes: -d "aAb _1"
  -n, --number <NUMBER>            Number of passwords to be generated
  -o, --output-file <OUTPUT_FILE>  Optional: writes the generated password/passwords into the specified file (a relative path is resolved from the current working directory)
  -j, --json                       Optional: passwords output as JSON
  -h, --help                       Print help
  -V, --version                    Print version
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

## Development

Branches follow Gitflow (`main` holds tagged releases, `develop` integrates, `feature/*` off `develop`) and commits follow [Conventional Commits](https://www.conventionalcommits.org/). Versions are derived from the commits by [release-plz](https://release-plz.dev/), which opens a release PR against `main`; merging it tags the release and the tag builds and publishes the binaries. See [CHANGELOG.md](CHANGELOG.md) for what changed.

Before a PR, run the same gate CI does:

```shell
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo deny check
bash dev/sast.sh        # semgrep, cargo-deny, grype, gitleaks, SBOM (needs Docker)
```
