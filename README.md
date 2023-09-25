# [scs-sdk-telemetry](https://crates.io/crates/scs-sdk-telemetry)

[![](https://img.shields.io/crates/v/scs-sdk-telemetry.svg)](https://crates.io/crates/scs-sdk-telemetry) [![Docs](https://docs.rs/scs-sdk-telemetry/badge.svg)](https://docs.rs/scs-sdk-telemetry)

This library reads telemetry data from Euro Truck Simulator 2 and American Truck Simulator then parse data into Rust struct.

Based on [RenCloud/scs-sdk-plugin](https://github.com/RenCloud/scs-sdk-plugin)

## Getting Started
Follow the installation section in [RenCloud/scs-sdk-plugin](https://github.com/RenCloud/scs-sdk-plugin#installation)

## Examples
Read data:
```rust
// main.rs

use scs_sdk_telemetry::shared_memory::SharedMemory;

fn main() {
  let mut shared_mem: SharedMemory = SharedMemory::connect();

  println!("{:#?}", shared_mem.read());
}
```

With [serde-rs/json](https://github.com/serde-rs/json):
```toml
# Cargo.toml

[dependencies.scs_sdk_telemetry]
features = ["json"]
```
```rust
// main.rs

use scs_sdk_telemetry::shared_memory::SharedMemory;

fn main() {
  let mut shared_mem: SharedMemory = SharedMemory::connect();

  println!("{:#?}", shared_mem.read().to_json().unwrap().to_string());
}
```
See the [docs](https://docs.rs/scs-sdk-telemetry) for all the details.

## License
This project is licensed under MPL-2.0.