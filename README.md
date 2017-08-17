# Tock Rust Template
Template for creating Tock applications in Tock


## Getting Started

First steps:

1. Get a copy of the latest nightly, in this repo's root:

    `rustup upgrade`
    `rustup override set nightly`

    Your rustc should be at least this new:
    ```
    $ rustc --version
    rustc 1.21.0-nightly (7ac979d8c 2017-08-16)
    ```

2. Need to grab a copy of the rust sources:

    `rustup component add rust-src`

3. Now you should be able to build with:

    `make`

4. To upload code on to a board:

   `tockloader install`


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
