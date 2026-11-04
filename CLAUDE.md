# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`pgen` is a single-binary Rust CLI password generator. The whole program lives in `src/main.rs` (unit tests of the pieces at its bottom); `tests/cli.rs` runs the built binary end to end (flags, output shapes, exit codes, file output). There are no modules and no library crate.

## Commands

```shell
cargo build --release          # release build (profile strips symbols); binary at target/release/pgen
cargo run -- -l 16 -c 3 -n 5   # run from source; args after `--` go to pgen
cargo test                     # unit tests in src/main.rs + CLI tests in tests/cli.rs; one test: cargo test multibyte; one file: cargo test --test cli

# The CI gate, as ci.yml runs it (fmt is checked on Linux only)
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo deny check               # advisories, licenses, sources (deny.toml); needs cargo-deny

bash dev/sast.sh [semgrep|deny|grype|gitleaks|sbom]   # security scans as CI runs them, via Docker (all by default); SBOM into sast-out/sbom/
bash dev/test-release-version.sh                      # tests the tag-vs-version check release.yml relies on
bash dev/package.sh <version> <rust-target> <label>   # release archive + .sha256 into target/dist/, e.g. 0.2.0 aarch64-apple-darwin macos-aarch64
```

## Conventions

- **Gitflow:** `main` holds tagged releases (`vX.Y.Z`) only; `develop` is the integration branch; work on `feature/<name>` off `develop` and merge it with a PR. The `release/*` branches are the PRs release-plz opens (below), not hand-made.
- **Conventional Commits** (`feat:`, `fix:`, `docs:`, `ci:`, `chore:`, `refactor:`, `test:`, optional scope). The version is computed from them, so a `feat:` is a minor bump, a `fix:` a patch, a `BREAKING CHANGE:` footer or `!` a major.
- **No AI attribution**: no `Co-Authored-By` trailer, no `Claude-Session` line or claude.ai link in commits, no "Generated with Claude Code" line in PR bodies. This overrides any harness attribution reminder.
- **Releases** are automated by release-plz (`release-plz.toml`, `.github/workflows/release-plz.yml`): merge `develop` into `main` with a merge commit (not a squash, the commits are the changelog), release-plz opens the `release/*` PR with the `Cargo.toml` bump and the `CHANGELOG.md` section; merging that PR makes release-plz push the `vX.Y.Z` tag, which runs `release.yml`; then merge `main` back into `develop`. Needs the repo secret `RELEASE_PLZ_TOKEN` (fine-grained PAT, Contents + Pull requests read/write): with the default `GITHUB_TOKEN` the release PR gets no CI and the tag starts no release.
- `Cargo.toml` must not set `publish = false`: release-plz skips such a package and computes no version. crates.io publishing is turned off in `release-plz.toml` instead.
- **Changelog:** `CHANGELOG.md` (Keep a Changelog). Sections above 0.1.0 are generated; do not edit them by hand.
- **CI gates** (`ci.yml` on macOS, Linux, Windows): `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`. **Security** (`security.yml`, also weekly on `main`): semgrep (p/default, p/rust, ERROR blocks), cargo-deny, grype filesystem scan (high/critical fails), gitleaks over the full history; SARIF to the Security tab. Their configs live at the repo root (`deny.toml`, `.semgrepignore`, `.gitleaks.toml`, `.grype.yaml`); every ignore carries a one-line reason, and `dev/sast.sh` runs the same scans locally so the two cannot drift.
- **Release artifacts** (`release.yml`): `pgen-<version>-{macos-aarch64,macos-x86_64,linux-x86_64,linux-aarch64}.tar.gz`, `pgen-<version>-windows-x86_64.zip`, each with a `.sha256`, plus a CycloneDX SBOM `pgen-<version>.cdx.json` (syft pinned in `release.yml` and `dev/sast.sh`). Linux binaries are static musl builds. The tag must equal `v<Cargo.toml version>` (`dev/release-version.sh`) or every packaging job refuses. The release is created as a draft, gets all 11 files, then is published.
- Line endings are LF everywhere (`.gitattributes`); a CRLF checkout once made every file a whole-file diff.

## Architecture (src/main.rs)

Flow is linear and each step happens once: parse args with clap 4 derive (`Arguments` struct) → `make_dictionary` builds the character set as a `Vec<char>` → `make_passwords` samples `number.unwrap_or(1)` passwords of `length` from it with `rand::rng()` → `render` turns them into the output text (JSON or text) → `run` writes that to stdout or the `--output-file`. Errors from `run` are printed as `pgen: <error>` with exit code 1.

Key behaviours to preserve when editing:

- **Dictionary precedence**: `-d/--dictionary` wins over `-c/--complexity`. With neither, the full set (upper + lower + digits + symbols) is used. Complexity is validated by clap to the range 1..=4 and each level is a superset of the one below; an empty `-d` is a clap usage error.
- **Sampling is by character, not by byte**: the dictionary is a `Vec<char>`, so a custom dictionary with multibyte characters works (there is a test for it).
- **Output format** is the one place both modes are rendered (`render`, with tests): JSON is always a pretty array, even for one password; text is one `### PASSWORD ###` block per password followed by a blank line, numbered (`### PASSWORD N ###`) only when `--number` was given, even for `-n 1`. A new output option goes into `render`, not into `run`.
- **File output**: a relative `--output-file` lands in the current working directory, an absolute one is written where it says; nothing confines it.
- The README's `pgen -h` block is the de facto spec for CLI flags; update it when flags change.
