[![Rust](https://github.com/drivasperez/tsconfig/actions/workflows/rust.yml/badge.svg)](https://github.com/drivasperez/tsconfig/actions/workflows/rust.yml)

# tsconfig

A Rust crate for parsing TypeScript's TSConfig files into a Rust struct.

A TSConfig file in a directory indicates that the directory is the root of a TypeScript or JavaScript project.

The TSConfig file can be either a tsconfig.json or jsconfig.json; both have the same behavior and the same set of config variables. One TSConfig can inherit fields from another if it is specified in the 'extends' field.

## Example usage

```rust
use tsconfig::TsConfig;
use std::path::Path;
let path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
    .join("test/tsconfig.default.json");
let config = TsConfig::parse_file(&path).unwrap();
```

## Links

- Documentation [can be found here](https://docs.rs/tsconfig)
- [crates.io package](https://crates.io/crates/tsconfig)
