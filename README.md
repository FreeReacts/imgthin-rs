# imgthin-rs
[![GitHub Actions](https://github.com/FreeReacts/imgthin-rs/workflows/Main/badge.svg)](https://github.com/FreeReacts/imgthin-rs/actions) [![crates.io](http://meritbadge.herokuapp.com/imgthin)](https://crates.io/crates/imgthin) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE) [![Released API docs](https://docs.rs/imgthin/badge.svg)](http://docs.rs/imgthin) [![Master API docs](https://img.shields.io/badge/docs-master-green.svg)](https://docs.rs/imgthin)

A rust image thinning library inspired by

> A fast parallel algorithm for thinning digital patterns. (Improved/ Original)

## Research Abstract

> A fast parallel thinning algorithm is proposed
> in this paper. It consists of two subiterations: one aimed at
> deleting the south-east boundary points and the north-west
> corner points while the other one is aimed at deleting the
> north-west boundary points and the south-east corner
> points. End points and pixel connectivity are preserved.
> Each pattern is thinned down to a "skeleton" of unitary
> thickness. Experimental results show that this method is
> very effective.

- [A fast parallel algorithm for thinning digital patterns](https://www-prima.inrialpes.fr/perso/Tran/Draft/gateway.cfm.pdf)
- [A modified fast parallel algorithm for thinning digital patterns](https://www.researchgate.net/publication/222456229)

## Installation

Add `imgthin` as a dependency to the `Cargo.toml` file.

To use the original algorithm from Zhang and Suen:-

```toml
# Cargo.toml

imgthin = "0.1.0"
```

To use the improved algorithm from Yung Sheng and Wen-Hsing:-

```toml
# Cargo.toml

imgthin = {version = "0.1.0", features=["improved_ysc_whh"]}

```

## Usage

```rust
use imgthin::imgthin;

//    _________ Vec<Vec<bool>>
//    v
let thinned = imgthin(vec!(
     vec!(false, false, true, true, false),
     vec!(false, false, true, true, false),
     vec!(false, false, true, true, false)
)).expect("Can not thin the image.");

```

## Contributions

All PRs and issues are welcome. And stars are also welcome.
