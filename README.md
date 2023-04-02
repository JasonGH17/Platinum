# Platinum

Language that compiles to WASM

## How to use

```bash
cargo run ./examples/main.plat
```

- Reads in main.plat
- Prints out the tokenized program
- Outputs main.wasm

```bash
wasmer ./examples/main.wasm -i main
```

Prints out the `main` function's result.