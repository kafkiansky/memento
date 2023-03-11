#[tokio::main]
async fn main() -> memento::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    memento
        .execute(memento::set("i".parse()?, memento::Item::timeless(0)))
        .await?;

    memento.execute(memento::incr("i".parse()?, 1)).await?;

    Ok(())
}
