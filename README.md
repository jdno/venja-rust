# Venja

[![Build Status](https://dev.azure.com/jdno/Venja/_apis/build/status/jdno.venja?branchName=master)](https://dev.azure.com/jdno/Venja/_build/latest?definitionId=1&branchName=master)
[![codecov](https://codecov.io/gh/jdno/venja/branch/master/graph/badge.svg)](https://codecov.io/gh/jdno/venja)

A simple, beautifully designed habit tracker.

## Getting Started

The backend of **Venja** is built using [Rust].

Make sure you have the latest stable version of Rust installed before
continuing. Installation instructions for most systems can be found at:
https://www.rust-lang.org/learn/get-started.

Once Rust has been installed, it's dependency manager [Cargo] can be used to
install additional tooling for local development.

    $ cargo install cargo-make diesel_cli

Data is stored in a Postgres database. Make sure you have Postgres installed and
running, and run the following command to create and configure the database for
development.

    $ diesel setup

Finally, the web server can be started using the following command:

    $ cargo run

Make sure to run the QA script before committing any changes:

    $ cargo make qa

## License

Copyright (c) 2019 Jan David Nose

**Venja** is [source-available] software. It is licensed under the terms of the
[Commons Clause License](https://commonsclause.com). See the [LICENSE] for more
details.

[cargo]: https://doc.rust-lang.org/cargo/
[cargo-make]: https://github.com/sagiegurari/cargo-make
[diesel]: http://diesel.rs
[license]: LICENSE.txt
[rust]: https://www.rust-lang.org
[source-available]: https://en.wikipedia.org/wiki/Source-available_software
