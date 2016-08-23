# algorithms-and-data-structures

[![Join the chat at https://gitter.im/Gitlings/algorithms-and-data-structures](https://badges.gitter.im/Gitlings/algorithms-and-data-structures.svg)](https://gitter.im/Gitlings/algorithms-and-data-structures?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge) [![Build Status](https://travis-ci.org/Gitlings/algorithms-and-data-structures.svg?branch=master)](https://travis-ci.org/Gitlings/algorithms-and-data-structures)

Implementing some usual algorithms and data structures in Rust

## Running
```
cargo run --bin <your_executable>
```

## Testing
```
cargo test
```

## Executables
There are executables provided for each algorithm, allowing you to input any data from keyboard and test them manually.

**Here is the list:**

* sorting_insertion

## Contributing
### Sorting algorithms
For now the project contains only sorting algorithms. For adding your own, you have to create a new module inside `src/sorting/base` folder. Create a new folder there with `mod.rs` file inside. You may want to take a look at `src/sorting/base/insertion/mod.rs` example. Before starting to write the code, make sure to declare your new module inside `src/sorting/base/mod.rs`.
```
pub mod insertion;
```

After adding your implementation, create a new source file for binary. Again, you may want to take a look at `src/sorting/base/insertion.rs` also. You also have to specify this file inside `Cargo.toml` which is located in the project's root.
```
[[bin]]
name = "sorting_insertion"
path = "src/sorting/insertion.rs"
```

The final and the easiest part is to write some unit tests for your algorithm. Here you just need to add test functions to `src/sorting/base/test.rs`, specifying your newly created module namespace.
