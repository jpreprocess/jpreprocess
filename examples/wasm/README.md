# example-wasm

Example of using jpreprocess from WebAssembly.

## Usage

1. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
2. Go to `examples/wasm` directory
   ```bash
   cd examples/wasm
   ```
3. Build wasm package
   ```bash
   wasm-pack build --target nodejs
   ```
4. Run javascript code
   ```bash
   cd js
   pnpm install
   pnpm build
   pnpm start
   ```
