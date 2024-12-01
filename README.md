# LibreSplit Converter

------------

Web app for converting LiveSplit files into the JSON format for [LibreSplit](https://github.com/wins1ey/LibreSplit "LibreSplit"). Loosely based on Loomeh's [LiveSplitToLAST](https://github.com/Loomeh/LiveSplitToLAST "LiveSplitToLAST").

------------
## Buiding (with Docker)
There is a dockerfile set up to ease building the project. Run the following to build and serve the app.
```bash
git clone https://github.com/JackTench/libresplit-converter
cd libresplit-converter
docker build -t libresplit-converter .
docker run -p 8080:80 libresplit-converter
```
------------
## Building (manual)
### Dependencies
- Rust toolchain `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- WASM compiler target `rustup target add wasm32-unknown-unknown`
- wasm-pack `cargo install wasm-pack`
- NPM

### Building
```bash
git clone https://github.com/JackTench/libresplit-converter
cd libresplit-converter
npm install
npm run build:wasm
npm start
```
