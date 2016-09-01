# loggly-rs
Unofficial Rust library for the Loggly API

[![crates.io](http://meritbadge.herokuapp.com/loggly)](https://crates.io/crates/loggly)
[![docs.rs](https://docs.rs/loggly/badge.svg)](https://docs.rs/loggly)
[![Build Status](https://travis-ci.org/pastoraleman/loggly-rs.svg?branch=master)](https://travis-ci.org/pastoraleman/loggly-rs)

loggly-rs implements an HTTPS Event Endpoint for sending messages to the Loggly RESTful API.
To use loggly-rs you must first create a Loggly account and generate an API key, which can be found within the account management interface. For more information on the API, see the [Loggly API Documentation](https://www.loggly.com/docs/http-endpoint/).

loggly-rs is available on [crates.io](https://crates.io/crates/loggly) and can be included in your Cargo.toml as follows:

```toml
[dependencies]
loggly = "0.1.0"
```

## Build Dependencies

This library utilises [hyper](https://crates.io/crates/hyper), which in turn requires the OpenSSL headers to be available during compilation. For more information on how to configure OpenSSL, see: [rust-openssl](https://github.com/sfackler/rust-openssl).

## Documentation

Documentation can be found at the official documentation repository: https://docs.rs/loggly

## Example

An example of using this library can be found in the examples directory, which can be run as follows:

```shell
cargo run --example simple
```

You will need to specify your authorised Loggly API key in order to use the example. For production usage, you should specify your API key through the process environment (e.g. [std::env::var](https://doc.rust-lang.org/std/env/fn.var.html)), the command line (e.g. [clap](https://crates.io/crates/clap)), or by parsing a configuration file (e.g. [toml](https://crates.io/crates/toml)).

# Feedback and Enhancements

I welcome feedback and enhancements to this library. Please create a Github Issue or a Pull Request!

# License
MIT
