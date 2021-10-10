# stubgen
Automatically generate `.pyi` stub files.
Intended to be used for Rust functions exposed in Python with pyo3 bindings.

Run:
```
cargo test stubgen --features stubgen -- --nocapture | grep "^def" | cat HEADER.txt - > stubs.pyi
```
