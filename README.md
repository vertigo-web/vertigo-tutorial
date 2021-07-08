Vertigo App Template
===================

This repository allows you to quickly start developing a vertigo application.

Using this template
--------------

Make sure you're using nigthly version of rust:
- `rustup default nightly`

Install cargo-generate and cargo-make:
- `cargo install cargo-generate cargo-make`

Bootstrap using cargo-generate:
- `cargo generate https://github.com/vertigo-web/vertigo-app-template`

Use cargo-make commands defined in Makefile:
- `makers start` - Build app and start development http server at url http://localhost:3000/
- `makers watch` - Rebuild and restart app at every change in code
- `makers serve` - Start development server without rebuilding

Different build profiles
--------------
- `makers start --profile profiling`
- `makers start --profile release`
