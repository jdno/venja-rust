# Venja

[![Build Status](https://dev.azure.com/6a64/Venja/_apis/build/status/venja-app.venja-backend?branchName=master)](https://dev.azure.com/6a64/Venja/_build/latest?definitionId=2&branchName=master)
[![codecov](https://codecov.io/gh/venja-app/venja-backend/branch/master/graph/badge.svg)](https://codecov.io/gh/venja-app/venja-backend)

A simple, beautifully designed habit tracker.

## Getting Started

The backend of **Venja** is built using Rust. Make sure you have the latest
stable version of Rust installed before continuing.

In this project, [Diesel](http://diesel.rs) is used to work with the database.
It comes with its own CLI that helps manage the project. The CLI must be
installed separately using the following command:

    $ cargo install diesel_cli

Once Diesel's CLI is installed, run the following command to set up the
database:

    $ diesel setup

## License

Copyright (c) 2019 Jan David Nose

**Venja** is [source-available] software. It is licensed under the terms of the
[Commons Clause License](https://commonsclause.com). See the [LICENSE] for more
details.

[license]: LICENSE.txt
[source-available]: https://en.wikipedia.org/wiki/Source-available_software
