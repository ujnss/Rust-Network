
- [Rust-Network](#rust-network)
  - [Overview](#overview)
  - [Examples](#examples)
  - [Usage](#usage)
  - [TODO](#todo)


# Rust-Network


## Overview


## Examples

Ref [ex01.rs](./examples/ex01.rs).

Run in 3 terminal consoles:

```sh
cargo run --example ex01 -- --party_id 0
cargo run --example ex01 -- --party_id 1
cargo run --example ex01 -- --party_id 2
```

[or]


```sh
./run.sh
```

## Usage

If this repo is public:

```toml
[dependencies]
xio = { git = "https://path/to/Rust-Network" }
```

If this repo is private, `clone this repo first`, and then:

```toml
[dependencies]
xio = { path = "/path/to/Rust-Network" }
```


## TODO

add error/stop/...
