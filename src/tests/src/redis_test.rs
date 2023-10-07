use mini_redis::client;
use tokio_stream::StreamExt;

pub async fn redis_test() -> () {
    tokio::spawn(async {
        publish().await
    });

    subscribe().await.expect("failed");

    println!("DONE")
}

async fn publish() -> mini_redis::Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await.expect("failed");

    // Publish some data
    client.publish("numbers", "1".into()).await.expect("failed");
    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await.expect("failed");
    let subscriber = client.subscribe(vec!["numbers".to_string()]).await.expect("failed");
    let messages = subscriber.into_stream();

    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg);
    }

    Ok(())
}

