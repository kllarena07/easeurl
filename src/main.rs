#![allow(clippy::disallowed_names)]
#![allow(clippy::let_underscore_future)]

use dotenv::dotenv;
use fred::prelude::*;
use std::env::var;

fn get_redis_client() -> Result<RedisClient, RedisError> {
    let username = var("REDIS_USERNAME").expect("REDIS_USERNAME must be set.");
    let password = var("REDIS_PASSWORD").expect("REDIS_PASSWORD must be set.");
    let host = var("REDIS_HOST").expect("REDIS_HOST must be set.");
    let port = var("REDIS_PORT").expect("REDIS_PORT must be set.");

    let redis_url = format!("redis://{}:{}@{}:{}", username, password, host, port);

    let config = RedisConfig::from_url(&redis_url)?;

    let client = Builder::from_config(config).build()?;

    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), RedisError> {
    dotenv().ok();

    let client = get_redis_client().unwrap();

    // callers can manage the tokio task driving the connections
    let _connection_task = client.init().await?;

    // convert response types to most common rust types
    let foo: Option<String> = client.get("foo").await?;
    println!("Foo: {:?}", foo);

    client
        .set(
            "foo",
            "bar",
            Some(Expiration::KEEPTTL),
            Some(SetOptions::NX),
            false,
        )
        .await?;

    client.quit().await?;
    Ok(())
}
