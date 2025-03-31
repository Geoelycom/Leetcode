use mini_redis::{client, Result};
#[tokio::main]

async fn main() -> Result<()> {
    // open a redis connection
    let mut client = client::connect("127.0.0.1:6378").await?;

    // set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("got the value from the server; result={:?}", result);

    Ok(())
}
