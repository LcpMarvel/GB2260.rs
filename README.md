# GB2260.rs

The Rust implementation for looking up the Chinese administrative divisions

## Installation

First, add the following to your Cargo.toml:

```rust
[dependencies]
gb2260 = "0.1.0"
```

Next, add this to your crate root:

```rust
extern crate libc;
```

## Usage

More example can be found in tests.

```rust

use gb2260::Division;
use gb2260::Source;

/// build a custom division
let division = Division {
    source: Source::GB,
    code: "110000".to_string(),
    name: "北京市",
    revision: "200712"
};

/// get by code
let division = gb2260::get(Source::GB, "200712", "330105".to_string());

/// list all prefectures for beijing
division.prefectures().unwrap();

/// list all counties for beijing
division.counties().unwrap();

/// list all counties for the prefecture of beijing
let prefectures = division.prefectures().unwrap();
let prefecture = prefectures.first().unwrap();

prefecture.counties().unwrap();

```

## Be Happy!
