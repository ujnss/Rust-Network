
- [Rust-Network](#rust-network)
  - [Overview](#overview)
  - [Examples](#examples)
  - [Usage](#usage)
  - [Docs](#docs)
  - [TODO](#todo)


# Rust-Network


## Overview


## Examples

Run in 3 terminal consoles:

```sh
cargo run --example ex01 -- --party_id 0
cargo run --example ex01 -- --party_id 1
cargo run --example ex01 -- --party_id 2
```

[or] run the following script for quick test:

```sh
./run.sh
```

For more details ref [ex01.rs](./examples/ex01.rs), [ex02.rs](./examples/ex02.rs), [ex03.rs](./examples/ex03.rs).


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

## Docs

- First, generate the document by running the following commands:

```sh
cargo doc --no-deps --all; cargo doc --no-deps --examples
```

- Then view the document by running the following commands:

```sh
$ cd target/doc
$ python3 -m http.server
Serving HTTP on 0.0.0.0 port 8000 (http://0.0.0.0:8000/) ...
```

- Open a browser and input like `127.0.0.1:8000/xio/index.html`.




## TODO

- add error/stop/...
- implement a thread version to reduce the communication size (use thread-id[int] instead of msgid[str]).
