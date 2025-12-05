# 在 Rust 中使用 Rudis

本指南介绍了如何在 Rust 应用程序中使用 Rudis。

## 安装

将以下内容添加到您的 `Cargo.toml` 文件中：

```toml
[dependencies]
rudis = "0.1"
```

## 基本用法

以下是一个简单的示例，展示如何连接到 Rudis 并执行操作：

```rust
use rudis::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::connect("127.0.0.1:6379")?;
    
    // 设置一个键
    client.set("key", "value")?;
    
    // 获取一个键
    let value = client.get("key")?;
    println!("值: {}", value);
    
    Ok(())
}
```

## 连接管理

对于生产应用程序，请考虑使用连接池：

```rust
use rudis::{Client, Pool};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Pool::new("127.0.0.1:6379", 10)?;
    
    let mut conn = pool.get()?;
    conn.set("key", "value")?;
    
    Ok(())
}
```

## 错误处理

所有 Rudis 操作都返回 `Result` 类型，允许进行健壮的错误处理：

```rust
use rudis::Client;

fn main() {
    let mut client = match Client::connect("127.0.0.1:6379") {
        Ok(client) => client,
        Err(e) => {
            eprintln!("连接失败: {}", e);
            return;
        }
    };
    
    if let Err(e) = client.set("key", "value") {
        eprintln!("设置键失败: {}", e);
    }
}
```