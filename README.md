# authorization-rs

![GitHub](https://img.shields.io/badge/language-Rust-green)
![GitHub](https://img.shields.io/github/license/CodeDead/authorization-rs)
![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/CodeDead/authorization-rs?include_prereleases)
[![Deploy](https://github.com/CodeDead/authorization-rs/actions/workflows/deploy.yml/badge.svg)](https://github.com/CodeDead/authorization-rs/actions/workflows/deploy.yml)
[![Test](https://github.com/CodeDead/authorization-rs/actions/workflows/test.yml/badge.svg)](https://github.com/CodeDead/authorization-rs/actions/workflows/test.yml)

`authorization-rs` is a (micro)service that can be used to authenticate, register and authorize users using roles, permissions and a MongoDB backend.

## Building

In order to build `authorization-rs`, you can run the following command:

```shell
cargo build
```

A `release` build with extra optimizations can be built by issuing the following command:

```shell
cargo build --release
```

## Running

You can run `authorization-rs` directly by issuing the following command:

```shell
cargo run
```

## Credits

* [uuid](https://crates.io/crates/uuid)
* [chrono](https://crates.io/crates/chrono)
* [serde](https://crates.io/crates/serde)
* [serde_json](https://crates.io/crates/serde_json)
* [mongodb](https://crates.io/crates/mongodb)
* [futures](https://crates.io/crates/futures)
* [actix-web](https://crates.io/crates/actix-web)
* [actix-cors](https://crates.io/crates/actix-cors)
* [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
* [config](https://crates.io/crates/config)
* [dotenv](https://crates.io/crates/dotenv)
* [bcrypt](https://crates.io/crates/bcrypt)
* [regex](https://crates.io/crates/regex)

## About

This library is maintained by CodeDead. You can find more about us using the following links:

* [Website](https://codedead.com)
* [Twitter](https://twitter.com/C0DEDEAD)
* [Facebook](https://facebook.com/deadlinecodedead)

Copyright © 2022 CodeDead
