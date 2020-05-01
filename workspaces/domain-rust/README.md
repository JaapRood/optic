# Experimental implementation of the Optic domain in Rust, targeting WASM.

## Requirements

### Rust Toolchain

You will need the standard Rust toolchain, including `rustup`, `rustc`, and `cargo`.

[Follow these instructions to install the Rust toolchain.](https://www.rust-lang.org/tools/install)

The Rust and WebAssembly experience is riding the Rust release trains to stable! That means we don't require any experimental feature flags. However, we do require Rust 1.30 or newer.

### wasm-pack

`wasm-pack` is your one-stop shop for building, testing, and publishing Rust-generated WebAssembly.

[Get `wasm-pack` here!](https://rustwasm.github.io/wasm-pack/installer/)

## Usage

Include the Webpack plugin in the building of your app to build the Rust domain into WASM. Will automatically watch / incrementally build as Webpack does.

```js
// in your webpack.config.js

const DomainRustPlugin = require('@useoptic/domain-rust/webpack.plugin');

module.exports = {
  // ...

  plugins: [DomainRustPlugin()],
};
```

Then use like any other module!

```js
import OpticDomain from '@useoptic/domain-rust';
```
