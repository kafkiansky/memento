#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut memento = memento::new("localhost:11211").await?;

    let resp = memento
        .execute(memento::set("x", memento::Item::timeless("lol")))
        .await?;

    println!("{:#?}", resp);

    let get_resp = memento
        .execute(memento::gets(vec!["kek", "x", "xxxx"]))
        .await?;

    println!("{:#?}", get_resp);

    let stats_resp = memento.execute(memento::stats()).await?;

    println!("{:#?}", stats_resp);

    Ok(())
}
