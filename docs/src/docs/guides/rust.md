# Using Rudis in Rust

This guide explains how to use Rudis in Rust applications.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rudis = "0.1"
```

## Basic Usage

Here's a simple example of how to connect to Rudis and perform operations:

```rust
use rudis::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::connect("127.0.0.1:6379")?;
    
    // Set a key
    client.set("key", "value")?;
    
    // Get a key
    let value = client.get("key")?;
    println!("Value: {}", value);
    
    Ok(())
}
```

## Connection Management

For production applications, consider using a connection pool:

```rust
use rudis::{Client, Pool};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Pool::new("127.0.0.1:6379", 10)?;
    
    let mut conn = pool.get()?;
    conn.set("key", "value")?;
    
    Ok(())
}
```

## Error Handling

All Rudis operations return a `Result` type, allowing for robust error handling:

```rust
use rudis::Client;

fn main() {
    let mut client = match Client::connect("127.0.0.1:6379") {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return;
        }
    };
    
    if let Err(e) = client.set("key", "value") {
        eprintln!("Failed to set key: {}", e);
    }
}
```