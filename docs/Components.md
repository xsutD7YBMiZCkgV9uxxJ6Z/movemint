Learn more about [Abscissa]'s ECS-like design. This is a framework we plan to use and improve upon.

`movemint` will feature the following commands / components:

- Node
- REPL
- Compiler
- Verifier
- Wallet

Some of these components will be re-used / shared within the `cargo-move` and `cargo-movemint` tools, which are geared towards researchers and developers whereas `movemint` is (when production-ready) meant for node operators.

We expect `cargo-move` to be used in Rust-Move work (say by PL researchers, vulnerabilty researchers, etc.)

We expect `cargo-movemint` to drive a movemint node for development purposes, as well as connecting with Trillian-backed Move repos in the future.


[Abscissa]: https://github.com/iqlusioninc/abscissa
