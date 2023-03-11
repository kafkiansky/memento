## Async memcached client for Rust.

### Examples
```rust
#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    let gets = memento
        .execute(memento::gets(vec!["x", "y"]))
        .await?;

    match gets {
        memento::CommandResp::Value(values) => {
            for (key, item) in values {
                println!(
                    "{key}: {item}",
                    key = key.to_string(),
                    item = item.to_string()
                )
            }
        }
        _ => println!("other"),
    }

    Ok(())
}

```