# grafo-rs
[![Rust](https://github.com/GrageraE/grafo-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/GrageraE/grafo-rs/actions/workflows/rust.yml)
[![Docs](https://github.com/GrageraE/grafo-rs/actions/workflows/docs.yml/badge.svg)](https://gragerae.github.io/grafo-rs/)

Implementación de un Grafo en Rust

### Tests

Para correr los tests unitarios, es necesario desactivar la paralelización y enviar `stdout`. Los tests de arbol en profundidad lo necesitan.
```
cargo test -j1 -- --nocapture
```
