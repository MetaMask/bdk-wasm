# Build on MacOS

## Native toolchain setup

On MacOS, you should replace the default `llvm` with the one from `brew`:

```sh
brew install llvm
```

We recommend creating a `.cargo` folder at the root of this repository with the following
`config.toml` file:

```toml
[env]
AR = "/opt/homebrew/opt/llvm/bin/llvm-ar"
CC = "/opt/homebrew/opt/llvm/bin/clang"
```

Additionally, if you're using rust-analyzer in VSCode, you'll want to add the following to
your `.vscode/settings.json` file:

```json
{
  "rust-analyzer.server.extraEnv": {
    "AR": "/opt/homebrew/opt/llvm/bin/llvm-ar",
    "CC": "/opt/homebrew/opt/llvm/bin/clang"
  },
  "rust-analyzer.cargo.target": "wasm32-unknown-unknown"
}
```

## Troubleshooting

### `rustup` installation (using Homebrew)

System-wide installation of `rust` might conflict with `brew`-managed `rustup` installation, see:
- https://github.com/rust-lang/rustup/issues/1236
- https://rust-lang.github.io/rustup/installation/other.html#homebrew

To use `brew`-managed `rustup` package, you can use:

```sh
brew uninstall rust
brew unlink rust
brew install rustup
export PATH="${PATH}:$(brew --prefix rustup)/bin" # .{bash,zsh}rc
```
