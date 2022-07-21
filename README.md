# rustemo

LR/GLR parser generator for Rust (currently only LR). In the early phase of
development.

## Why this name?

Rustemo is pronounced the same as Serbian word "растемо" which means "we grow".
The name is a tribute to the awesome and ever growing Rust community.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Credits

Bootstrapping approach and the pieces of code are taken from the [LALRPOP
project](https://github.com/lalrpop/lalrpop).

## Similar projects

The architecture and the general idea of Rustemo is loosely based on a similar
project for Python, called
[parglare](https://github.com/igordejanovic/parglare), I've started several
years ago.

I have found a lot of inspiration and ideas in the following projects:

- [LALRPOP](https://github.com/lalrpop/lalrpop) - LR(1) parser generator for
  Rust. This project is the most similar to Rustemo so I've found there a lot of
  nice ideas.
- [Nom](https://github.com/Geal/nom) - Parser combinator library. Nice
  architecture and nicely designed traits.
- [pest](https://github.com/pest-parser/pest) - PEG parser for Rust. Seems nice
  and well maintained.
