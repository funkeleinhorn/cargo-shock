# Cargo Shock ⚡

[![crates.io](https://img.shields.io/crates/v/cargo-shock.svg)](https://crates.io/crates/cargo-shock)

Let Cargo MAKE you learn Rust by punishing you if your code fails to compile.

Inspired by the great [Cargo Mommy](https://github.com/Gankra/cargo-mommy) and [Cargo Vibe](https://github.com/Shadlock0133/cargo-vibe).

# Installation

You can `cargo install cargo-shock`

# Usage

To let Cargo Shock trigger your shock collar use: `cargo shock build`

To use it everytime you can `alias cargo="cargo shock"`.

Cargo Shock can also be combined with other tools like [Cargo Mommy](https://github.com/Gankra/cargo-mommy) and [Cargo Vibe](https://github.com/Shadlock0133/cargo-vibe) like this:
`cargo mommy vibe shock build ...`

# Configuration

Cargo Shock uses [OpenShock](https://openshock.org/) to interact with your shocking devices.

To be able to do so you need to give it an API Key via the `CARGO_SHOCK_TOKEN=...` environment variable.

To select which shock collar to trigger specify its ID from OpenShock via the `CARGO_SHOCK_ID=...` environment variable.

How intense, long and when you get shocked is determined by selecting a random pattern from the `CARGO_SHOCK_PATTERN=25 0.3s 1s / ...` environment variable.

This variable consists of possible shock configurations of the form `intensity duration delay` seperated by `/`.

Is you use an own OpenShock server you can give its API adress via the `CARGO_SHOCK_ADDR=...` environment variable.

For easier use it is recommended to set this variables once in your `.profile` or so.

# License

This project is licensed under GPLv3. For details see the LICENSE file.
