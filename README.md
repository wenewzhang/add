# add workspace: add/Cargo.toml
```
[workspace]

members = [
    "adder",
    "add-one",
]

```

build release:
```
cargo build --release
```

build the specified project
```
cargo build -p adder
```

create project
```
cargo new add-one --lib

or

cargo new adder --bin
```
