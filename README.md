<p align="center">
  <img
    alt="A snake inside of a gear shape"
    src="./assets/logo.svg"
    height="192px"
  >
</p>

# Hebi

This repository hosts a dynamically typed language, its compiler, and VM.

🚧 This branch contains a rewrite of the language. It is currently heavily WIP! 🚧

## Usage

Install the library (currently only available via git):

```
$ cargo add --git https://github.com/jprochazk/hebi.git --branch v2
```

Import it, and run some code:

```rust
use hebi::Hebi;

fn main() {
  let mut hebi = Hebi::new();

  println!("1 + 1 = {}", hebi.eval("1 + 1").unwrap());
}
```

Hebi can do much more than this, though! Here are some of its features:

- Syntax similar to Python, including significant indentation
- First-class functions
- Classes with single inheritance
- Easy Rust function and struct binding
- Async support

Visit the [examples](./examples) directory to see Hebi in action.

You can run an example using `cargo run --example <name>`:
```
$ cargo run --example basic
```

As well as with the CLI:
```rs
$ cargo run -- examples/hebi/tic-tac-toe.hebi
```

## Development

The first step is to install Rust and Cargo via [rustup](https://rustup.rs/).

### xtask

This repository uses [cargo-xtask](https://github.com/matklad/cargo-xtask) for various utilities, scripts, and tasks. That means you don't need anything other than Rust and Cargo. No makefiles, Python, or Bash.

To see all available tasks, run:
```
$ cargo xtask
```

To run one of them, use:
```
$ cargo xtask <task>
```

Or the slightly shorter:
```
$ cargo x <task>
```

For example:
```
# Run all tests and examples
$ cargo xtask test
```

Some tasks use tools which you'll have to install, though these are kept to just a select few, and ideally always installed through either `rustup` or `cargo`.

- [Miri](https://github.com/rust-lang/miri) (`rustup +nightly component add miri`)
- [Insta](https://insta.rs/) (`cargo install --locked cargo-insta`)
- [mdBook](https://github.com/rust-lang/mdBook) (`cargo install --locked mdbook`)

### CLI
The Hebi CLI provides a few tools for running and debugging scripts. It can be used by through `cargo run` or directly after installing the binary with `cargo install --path cli`. The examples below will demonstrate both ways.

```bash
# Print help
$ cargo run -- --help
Usage: hebi-cli [FILE] [COMMAND]

Commands:
  run          Execute a Hebi file [default]
  disassemble  Disassemble a Hebi file [aliases: dis]
  repl         
  help         Print this message or the help of the given subcommand(s)

Arguments:
  [FILE]  The path to the Hebi file to process. Required unless code is supplied through stdin

Options:
  -h, --help     Print help
  -V, --version  Print version

# Execute a script
$ echo 'print "Hello, world!"' | hebi 
Hello, world!

# Disassemble a script
$ hebi dis test.hebi
function `__main__` (registers: 3, length: 46, constants: 5)
.code
  0  | load_smi 0
  2  | store_global [0]; x
  4  | load_global [1]; to_str
  6  | store r1
<...>

# Run a script and dump the VM state on exit
$ cargo run -- run test.hebi --dump
```

## Why Hebi?

I thought it was appropriate, because the language is in the Python family, and Hebi (蛇) means snake in Japanese. 

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
