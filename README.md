# LibreSplit-converter

A WASM library for converting [LiveSplit](https://livesplit.org) `.lss` files into the [LibreSplit](https://libresplit.org) `.json` format.

An online converter using this library is available at [libresplit.org/converter](https://libresplit.org/converter).

## Install
```bash
npm i @libresplit/libresplit-converter
```

## Build
### Prerequisites
- Rust toolchain: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- WASM compiler target: `rustup target add wasm32-unknown-unknown`
- wasm-pack: <br>
`cargo install wasm-pack` <br>
 or <br>
`curl https://drager.github.io/wasm-pack/installer/init.sh -sSf | sh`

### Building
```sh
git clone https://github.com/LibreSplit/LibreSplit-converter
cd LibreSplit-converter
wasm-pack build --scope libresplit --target web --release
```

This outputs a compiled WASM package under `pkg/`.

---

Published as an npm package under [`@libresplit/libresplit-converter`](https://npmjs.com/package/@libresplit/libresplit-converter).
