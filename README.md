## Async memcached client for Rust.

[![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg?style=flat-square)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/memento.svg)](https://crates.io/crates/memento)

## Contents
- [Usage](#usage)
    - [Set](#set)
    - [Get](#get)
    - [Gets](#gets)
    - [Increment](#increment)
    - [Decrement](#decrement)
    - [Delete](#delete)
    - [Add](#add)
    - [Append](#append)
    - [Prepend](#prepend)
    - [Replace](#replace)
    - [Version](#version)
    - [Quit](#quit)
    - [Stats](#stats)

## Usage
### Set
```rust
use memento::Item;

#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento.set("x".parse()?, Item::timeless("y")).await? {
        memento::CommandResp::Stored => println!("OK"),
        cmd => println!("{:#?}", cmd),
    }
    
    Ok(())
}
```

### Get
```rust
use memento::Item;
    
#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento.get("x").await? {
        memento::CommandResp::Value { key, item } => {
            println!(
                "{key}: {value}",
                key = key.to_string(),
                value = item.to_string()
            )
        }
        cmd => println!("{:#?}", cmd),
    }
    
    Ok(())
}
```

### Gets
```rust
use memento::Item;

#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento.set("a".parse()?, Item::timeless("b")).await? {
        memento::CommandResp::Stored => println!("OK"),
        cmd => println!("{:#?}", cmd),
    }

    match memento.set("c".parse()?, Item::timeless("d")).await? {
        memento::CommandResp::Stored => println!("OK"),
        cmd => println!("{:#?}", cmd),
    }

    match memento.gets(vec!["a".parse()?, "c".parse()?]).await? {
        memento::CommandResp::Values(values) => {
            for (key, value) in values {
                println!(
                    "{key}: {value}",
                    key = key.to_string(),
                    value = value.to_string()
                )
            }
        }
        cmd => println!("{:#?}", cmd),
    }

    Ok(())
}
```

### Increment
```rust
use memento::Item;

#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento.set("counter".parse()?, Item::timeless(0)).await? {
        memento::CommandResp::Stored => println!("OK"),
        cmd => println!("{:#?}", cmd),
    }

    match memento.incr("counter".parse()?, 1).await? {
        memento::CommandResp::Counter(value) => {
            println!("counter: {value}")
        }
        cmd => println!("{:#?}", cmd),
    }
    
    Ok(())
}
```

### Decrement
```rust
use memento::Item;
    
#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento.set("counter".parse()?, Item::timeless(0)).await? {
        memento::CommandResp::Stored => println!("OK"),
        cmd => println!("{:#?}", cmd),
    }

    match memento.decr("counter".parse()?, 1).await? {
        memento::CommandResp::Counter(value) => {
            println!("counter: {value}")
        }
        cmd => println!("{:#?}", cmd),
    }
    
    Ok(())
}
```

### Delete
```rust
use memento::Item;
    
#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento.set("counter".parse()?, Item::timeless(0)).await? {
        memento::CommandResp::Stored => println!("OK"),
        cmd => println!("{:#?}", cmd),
    }

    match memento.delete("counter".parse()?).await? {
        memento::CommandResp::Deleted => println!("deleted"),
        cmd => println!("{:#?}", cmd),
    }
    
    Ok(())
}
```

### Add
```rust
use memento::Item;
    
#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento.set("counter".parse()?, Item::timeless(0)).await? {
        memento::CommandResp::Stored => println!("OK"),
        cmd => println!("{:#?}", cmd),
    }

    match memento.add("counter".parse()?, Item::timeless(10)).await? {
        memento::CommandResp::NotStored => println!("value exists"),
        cmd => println!("{:#?}", cmd),
    }

    match memento
        .add("another_counter".parse()?, Item::timeless(10))
        .await?
    {
        memento::CommandResp::Stored => println!("value stored"),
        cmd => println!("{:#?}", cmd),
    }
    
    Ok(())
}
```

### Append
```rust
use memento::Item;

#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento
        .set("language".parse()?, Item::timeless("rust"))
        .await?
    {
        memento::CommandResp::Stored => println!("OK"),
        cmd => println!("{:#?}", cmd),
    }

    match memento
        .append("language".parse()?, Item::timeless(" c++"))
        .await?
    {
        memento::CommandResp::Stored => println!("OK"),
        cmd => println!("{:#?}", cmd),
    }

    match memento.get("language".parse()?).await? {
        memento::CommandResp::Value { key, item } => {
            println!(
                "{key}: {value}",
                key = key.to_string(),
                value = item.to_string()
            )
        }
        cmd => println!("{:#?}", cmd),
    }

    Ok(())
}
```

### Prepend
```rust
use memento::Item;

#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento
        .set("language".parse()?, Item::timeless("rust"))
        .await?
    {
        memento::CommandResp::Stored => println!("OK"),
        cmd => println!("{:#?}", cmd),
    }

    match memento
        .prepend("language".parse()?, Item::timeless("c++ "))
        .await?
    {
        memento::CommandResp::Stored => println!("OK"),
        cmd => println!("{:#?}", cmd),
    }

    match memento.get("language".parse()?).await? {
        memento::CommandResp::Value { key, item } => {
            println!(
                "{key}: {value}",
                key = key.to_string(),
                value = item.to_string()
            )
        }
        cmd => println!("{:#?}", cmd),
    }

    Ok(())
}
```

### Replace
```rust
use memento::Item;
use std::time::Duration;

#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento.set("a".parse()?, Item::timeless("b")).await? {
        memento::CommandResp::Stored => println!("OK"),
        cmd => println!("{:#?}", cmd),
    }

    match memento
        .replace("a".parse()?, Item::expires("d", Duration::from_secs(10)))
        .await?
    {
        memento::CommandResp::Stored => println!("Replaced"),
        cmd => println!("{:#?}", cmd),
    }

    match memento.get("a".parse()?).await? {
        memento::CommandResp::Value { key, item } => {
            println!(
                "{key}: {value}",
                key = key.to_string(),
                value = item.to_string()
            )
        }
        cmd => println!("{:#?}", cmd),
    }

    Ok(())
}
```

### Version
```rust
#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento.version().await? {
        memento::CommandResp::Version(version) => println!("version {version}"),
        cmd => println!("{:#?}", cmd),
    }

    Ok(())
}
```

### Quit
```rust
#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento.quit().await? {
        memento::CommandResp::NoResponse => println!("connection closed"),
        cmd => println!("{:#?}", cmd),
    }

    Ok(())
}
```

### Stats
```rust
use memento::Stat;

#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    match memento.stats().await? {
        memento::CommandResp::Stats(stats) => {
            for stat in stats {
                match stat {
                    Stat::Pid(pid) => println!("pid {pid}"),
                    Stat::CmdSet(sets) => println!("sets {sets}"),
                    Stat::CmdGet(gets) => println!("gets {gets}"),
                    Stat::Other { name, value } => {
                        println!("{name}: {value}")
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    Ok(())
}
```