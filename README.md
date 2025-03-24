# Creating a Rust Project

## Initialize a new Rust project:
```sh
cargo new my_project
cd my_project
```

## Build and run your project:
```sh
cargo run
```

## Compile a Rust file manually (without Cargo):
```sh
rustc main.rs
./main
```

## Format Rust code:
```sh
rustfmt main.rs
```

## Check for code errors without compiling:
```sh
cargo check
```

## View documentation for dependencies:
```sh
cargo doc --open
```

## Run tests:
```sh
cargo test
```

## Add a dependency:
```sh
cargo add crate_name
```

## Update dependencies:
```sh
cargo update
```

## Build a release version (optimized for performance):
```sh
cargo build --release
```

## Run Clippy (Linter for Rust):
```sh
cargo clippy
```

## Clean up build files:
```sh
cargo clean
```