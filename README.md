# Vertigo Tutorial Done

This repository allows you to check the final outcome
of the **[Vertigo TUTORIAL](https://github.com/vertigo-web/vertigo/blob/master/tutorial.md)**


## Usage

Make sure you're using nigthly version of rust:

- `rustup default nightly`

Add target wasm32

- `rustup target add wasm32-unknown-unknown`

Install vertigo-cli:

- `cargo install vertigo-cli --version=0.2.0-alpha`

Use one of `vertigo` commands:

- `vertigo build` - Build app
- `vertigo serve` - Start app at http://localhost:4444/
- `vertigo watch` - Rebuild and serve app at every change in code
