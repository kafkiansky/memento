#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    let get_resp = memento
        .execute(memento::gets(vec!["kek", "x", "xxxx"]))
        .await?;

    match get_resp {
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
