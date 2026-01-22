# sheen ✨

[![Crates.io](https://img.shields.io/crates/v/sheen.svg)](https://crates.io/crates/sheen)
[![Docs.rs](https://docs.rs/sheen/badge.svg)](https://docs.rs/sheen)

A minimal, colorful logging library for Rust.

![sheen demo](sheen.gif)

## Quick Start

```rust
fn main() {
    sheen::init();
    sheen::info!("Server started", port = 3000);
}
```

Output:

```
14:32:15 INFO  Server started port=3000
```

## Features

- Colorful, human-readable output
- Structured key=value logging
- Builder pattern configuration
- Zero config defaults

## Installation

```toml
[dependencies]
sheen = "0.1"
```

## Usage

### Quick Start

```rust
fn main() {
    sheen::init();
    
    sheen::info!("Server started", port = 3000);
    sheen::debug!("Loading config");
    sheen::warn!("Cache miss", key = "user_123");
    sheen::error!("Connection failed", attempts = 3);
}
```

### Custom Configuration

```rust
use sheen::{Logger, Level};

fn main() {
    sheen::init_with(
        Logger::new()
            .level(Level::Trace)
            .prefix("myapp")
            .timestamp(true)
    );

    sheen::trace!("verbose output");
    sheen::info!("ready");
}
```

### Structured Fields

All log macros accept key-value pairs:

```rust
sheen::info!("User logged in", user = "alice", session = 42, admin = true);
// Output: 14:32:15 INFO  User logged in user="alice" session=42 admin=true
```

### Using Logger Directly

```rust
use sheen::{Logger, Level};

let logger = Logger::new()
    .level(Level::Debug)
    .prefix("api");

logger.info("Request handled", &[("status", &200), ("path", &"/users")]);
```
