[<img src="./docs/logo.png" width="500" style="padding-left: 250px;"/>](./docs/logo.png)

# CluStr

[![Test](https://github.com/TristanBester/clustr/actions/workflows/test.yaml/badge.svg)](https://github.com/TristanBester/clustr/actions/workflows/test.yaml)
[![codecov](https://codecov.io/gh/TristanBester/clustr/branch/main/graph/badge.svg?token=1ISWO9KU9S)](https://codecov.io/gh/TristanBester/clustr)
![Crates.io](https://img.shields.io/crates/v/clustr?color=blue)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat&logo=git)](https://opensource.org/licenses/MIT)

---

**Documentation**: <a href="https://docs.rs/clustr/0.1.2/clustr" target="_blank">https://docs.rs/clustr/0.1.2/clustr/</a>

**Crate**: <a href="https://crates.io/crates/clustr" target="_blank">https://crates.io/crates/clustr</a>

**Source Code**: <a href="https://github.com/TristanBester/clustr" target="_blank">https://github.com/TristanBester/clustr</a>

---

## Description
This crate provides a scalable string clustering implementation.

Strings are aggregated into clusters based on pairwise Levenshtein distance. If the distance is below a set fraction of the shorter string’s length, the strings are added to the same cluster.

## Multithreading Model
- The input strings are evenly paritioned across the set of allocated threads.
- Once each thread has clustered its associated input strings, result aggregation is started.
- Clusters are merged in pairs accross multiple threads in a manner that is similar to traversing a binary tree from the leaves up to the root. The root of the tree is the final clustering.
- Thus, if there are N threads allocated, there will be ceil(log2(N)) merge operations.

## Installation
```
[dependencies]
clustr = "0.1.2"
```

### Getting Started
Basic usage:
```rust
let inputs = vec!["aaaa", "aaax", "bbbb", "bbbz"];
let expected = vec![vec!["aaaa", "aaax"], vec!["bbbb", "bbbz"]];

let clusters = clustr::cluster_strings(&inputs, 0.25, 1)?;

assert_eq!(clusters, expected);
```

Multithreading:
```rust
let inputs = vec!["aa", "bb", "aa", "bb"];
let expected = vec![vec!["aa", "aa"], vec!["bb", "bb"]];

let results = clustr::cluster_strings(&inputs, 0.0, 4)?;
  
// Order of returned clusters nondeterministic
for e in expected {
    assert!(results.contains(&e));
}
```





