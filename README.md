# with-env

A simple utility to load `.env` before starting a command, for the lifetime of the command.

## Installing

Install [Rust](https://rustup.rs/), then run `cargo install --git https://github.com/narigama/with-env`.

## Usage

Take any command and prefix `with-env` to automatically load `.env` before
running the command. The contents will be available to the command when it runs,
but wont infect your shell afterwards.
