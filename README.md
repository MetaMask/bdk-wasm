<div align="center">
  <h1>The Bitcoin Dev Kit: WebAssembly</h1>

  <img src="./static/bdk.png" width="220" />

  <p>
    <strong>The Bitcoin Dev Kit for Browsers and Node</strong>
  </p>

  <p>
    <a href=""><img alt="NPM Package" src="https://img.shields.io/npm/v/bitcoindevkit.svg"/></a>
    <a href="https://github.com/MetaMask/bdk-wasm/blob/master/LICENSE"><img alt="MIT or Apache-2.0 Licensed" src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg"/></a>
    <a href="https://coveralls.io/github/MetaMask/bdk-wasm?branch=main"><img src="https://coveralls.io/repos/github/MetaMask/bdk-wasm/badge.svg?branch=main"/></a>
    <a href="https://blog.rust-lang.org/2022/08/11/Rust-1.63.0.html"><img alt="Rustc Version 1.63.0+" src="https://img.shields.io/badge/rustc-1.63.0%2B-lightgrey.svg"/></a>
    <a href="https://discord.gg/d7NkDKm"><img alt="Chat on Discord" src="https://img.shields.io/discord/753336465005608961?logo=discord"></a>
  </p>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>

</div>

## About

The `bdk-wasm` library aims at providing access to the excellent [BitcoinDevKit](https://github.com/bitcoindevkit/bdk) to JS and Node environments (and eventually any device supporting WebAssembly).
It specializes in compiling BDK on the `wasm32-unknown-unknown` target and use [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) to create TypeScript bindings.

This repo handles the packaging and publishing of the `bitcoindevkit` NPM package, using `wasm-pack`.

This library offers all the desired functionality to build a Bitcoin wallet out of the box:

- UTXO management
- Coin selection
- Wallet upates by syncing and scanning the chain data
- Bitcoin descriptors for flexibility in the definition of spending conditions. Supports all address types from legacy to Taproot.
- State update and persistence
- Transaction creation, signing and broadcasting
- Dynamic addresses
- and much more

For a lightweight library providing stateless utility functions, see [`bitcoinjs`](https://github.com/bitcoinjs/bitcoinjs-lib).

## Browser Usage

```sh
yarn add bitcoindevkit
```

## Development Environment

### Requirements

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

#### MacOS special requirement

On MacOS, you should replace the default `llvm` with the one from `brew`:

```sh
brew install llvm
```

We recommend creating a `.cargo` folder at the root of the repo with the following `config.toml` file:

```toml
[env]
AR = "/opt/homebrew/opt/llvm/bin/llvm-ar"
CC = "/opt/homebrew/opt/llvm/bin/clang"
```

### Build with `wasm-pack build`

```sh
wasm-pack build
```

> Choose your desired features when building: `--features "esplora"`

### Test in Headless Browsers with `wasm-pack test`

```sh
wasm-pack test --headless --firefox
```

> Works with `--firefox`, `--chrome` or `safari`.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
