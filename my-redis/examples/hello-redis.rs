use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    // open a connection to mini-redis
    let mut client = client::connect("127.0.0.1:8309").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("got value from the server; result = {:?}", result);

    Ok(())
}
