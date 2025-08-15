> [!WARNING]
> This repository has [moved to the official Bitcoindevkit organization](https://github.com/bitcoindevkit/bdk-wasm) and will no longer be maintained.

<div align="center">
  <h1>The Bitcoin Dev Kit: WebAssembly</h1>

  <img src="./static/bdk.png" width="220" />

  <p>
    <strong>The Bitcoin Dev Kit for Browsers, Node, and React Native</strong>
  </p>

  <p>
    <a href="https://www.npmjs.com/package/@metamask/bitcoindevkit"><img alt="NPM Package" src="https://img.shields.io/npm/v/@metamask/bitcoindevkit.svg"/></a>
    <a href="https://github.com/MetaMask/bdk-wasm/blob/master/LICENSE"><img alt="MIT or Apache-2.0 Licensed" src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg"/></a>
    <a href="https://blog.rust-lang.org/2023/10/05/Rust-1.73.0.html"><img alt="Rustc Version 1.73.0+" src="https://img.shields.io/badge/rustc-1.73.0%2B-lightgrey.svg"/></a>
    <a href="https://discord.gg/d7NkDKm"><img alt="Chat on Discord" src="https://img.shields.io/discord/753336465005608961?logo=discord"></a>
  </p>

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

## Notes on WASM Specific Considerations

> [!WARNING]
> There are several limitations to using BDK in WASM. Basically any functionality that requires the OS standard library is not directly available in WASM. However, there are viable workarounds documented below. Some key limitations include:
>
> - No access to the file system
> - Network access is limited to http(s)

### WASM Considerations Overview

#### No access to the file system

With no direct access to the file system, persistence cannot be handled by BDK directly. Instead, an in memory wallet must be used in the WASM environment, and the data must be exported using `wallet.take_staged()`. This will export the changeset for the updates to the wallet state, which must then be merged with current wallet state in JS (will depend on your persistence strategy). The persisted `ChangeSet` can be passed to `wallet.load()` to recover the wallet.

#### Network access is limited to http(s)

This essentially means the library only supports [Esplora](https://github.com/blockstream/esplora/blob/master/API.md) as blockchain client. Both RPC and Electrum clients require sockets and will not work for BDK in a WASM environment out of the box.

## Development Environment

### Requirements

- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

#### MacOS special requirement

Refers to [this section](./DEVELOPMENT.md#build-on-macos).

### Build with `wasm-pack build`

> [!IMPORTANT]
> You need the `wasm32-unknown-unknown` toolchain to be installed:
>
> ```sh
> rustup target add wasm32-unknown-unknown
> ```

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
