<div align="center">
  <h1>The Bitcoin Dev Kit: WebAssembly</h1>

  <img src="./static/bdk.png" width="220" />

  <p>
    <strong>The Bitcoin Dev Kit for Browsers, Node, and React Native</strong>
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

## Example Node.js Usage
```sh
npm install
npm run test
```

## Notes on WASM Specific Considerations

> ⚠️ **Important:** There are several limitations to using BDK in WASM. Basically any functionality that requires OS access is not directly available in WASM and must therefore be handled in JavaScript. However, there are viable workarounds documented below. Some key limitations include:
>
> - No access to the file system
> - No access to the system time
> - Network access is limited to http(s)

## WASM Considerations Overview

### No access to the file system
With no direct access to the file system, persistence cannot be handled by BDK directly. Instead, an in memory wallet must be used in the WASM environment, and the data must be exported using `wallet.take_staged()`. This will export the changeset for the updates to the wallet state, which must then be merged with current wallet state in JS (will depend on your persistence strategy). When you restart your app, you can feed your persisted wallet data to `wallet.load()` to recover the wallet state.

### No access to the system time
Any function that requires system time, such as any sort of timestamp, must access system time through a wasm binding to the JavaScript environment. However, in the case of `wallet.apply_update()`, this is being handled behind the scenes. If it weren't you'd have to use `wallet.apply_update_at()` instead. But it's worth keeping this limitation in mind in case you do hit an error related to system time access limitations.

### Network access is limited to http(s)
This effectively means that the blockchain client must be an Esplora instance. Both RPC and Electrum clients require sockets and will not work for BDK in a WASM environment out of the box.

### Troubleshooting
WASM errors can be quite cryptic, so it's important to understand the limitations of the WASM environment. One common error you might see while running a BDK function through a WASM binding in the browser is `unreachable`. This indicates you're trying to access OS level functionality through rust that isn't available in WASM.


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

Additionally, if you're using rust-analyzer in VSCode, you'll want to add the following to your `.vscode/settings.json` file:

```json
{
    "rust-analyzer.server.extraEnv": {
        "AR": "/opt/homebrew/opt/llvm/bin/llvm-ar",
        "CC": "/opt/homebrew/opt/llvm/bin/clang"
    },
    "rust-analyzer.cargo.target": "wasm32-unknown-unknown"
}
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

> Works with `--firefox`, `--chrome` or `--safari`.

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
