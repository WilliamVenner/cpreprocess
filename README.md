[![crates.io](https://img.shields.io/crates/v/cpreprocess.svg)](https://crates.io/crates/cpreprocess)
[![docs.rs](https://docs.rs/cpreprocess/badge.svg)](https://docs.rs/cpreprocess/)
[![license](https://img.shields.io/crates/l/cpreprocess)](https://github.com/WilliamVenner/cpreprocess/blob/master/LICENSE)

# cpreprocess

Stupid and cursed Rust procedural macro that runs a C preprocessor on the input

# Usage

```toml
[dependencies]
cpreprocess = "*"
```

# Example

```rust
fn main() {
    cpreprocess::cpreprocess! {"
        #define MACRO(NAME) fn print_ ## NAME () { println!(\"hello world\"); }

        MACRO(hello_world)

        print_hello_world()
    "}
}
```
