# LibreSplit Converter

------------

Web app for converting LiveSplit files into the JSON format for [LibreSplit](https://github.com/wins1ey/LibreSplit "LibreSplit"). Loosely based on Loomeh's [LiveSplitToLAST](https://github.com/Loomeh/LiveSplitToLAST "LiveSplitToLAST").

------------

## Dependencies
- Rust toolchain `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- WASM compiler target `rustup target add wasm32-unknown-unknown`
- wasm-pack `cargo install wasm-pack`
- NPM

------------

## Building
```bash
git clone https://github.com/JackTench/libresplit-converter
cd libresplit-converter
npm install
npm run build:wasm
npm start
```
