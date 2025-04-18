edsm-dumps-model
=======

Data models for [EDSM nightly dump files](https://www.edsm.net/nightly-dumps).

## Features

* `type_hash`
    - Add derive `type_hash::TypeHash` from [type_hash](https://crates.io/crates/type_hash) to model types

## License

`edsm-dumps-model` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).

## Development tips

### Tests with nightly dump data

You can test this crate with [EDSM nightly dump data](https://www.edsm.net/en/nightly-dumps).

For testing, you need to download dump files with `make download-dumps`.
Then, you can run tests with `cargo t -- --ignored`.
