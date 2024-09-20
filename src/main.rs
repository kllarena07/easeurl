#![allow(clippy::disallowed_names)]
#![allow(clippy::let_underscore_future)]

use dotenv::dotenv;
use fred::prelude::*;
use std::env::var;

#[tokio::main]
async fn main() -> Result<(), RedisError> {
    dotenv().ok();

    let username = var("REDIS_USERNAME").expect("REDIS_USERNAME must be set.");
    let password = var("REDIS_PASSWORD").expect("REDIS_PASSWORD must be set.");
    let host = var("REDIS_HOST").expect("REDIS_HOST must be set.");
    let port = var("REDIS_PORT").expect("REDIS_PORT must be set.");

    let redis_url = format!("redis://{}:{}@{}:{}", username, password, host, port);

    // create a config from a URL
    let config = RedisConfig::from_url(&redis_url)?;
    // see the `Builder` interface for more information
    let client = Builder::from_config(config).build()?;
    // callers can manage the tokio task driving the connections
    let _connection_task = client.init().await?;
    // convert response types to most common rust types
    let foo: Option<String> = client.get("foo").await?;
    println!("Foo: {:?}", foo);

    client
        .set(
            "foo",
            "bar",
            Some(Expiration::EX(1)),
            Some(SetOptions::NX),
            false,
        )
        .await?;

    // or use turbofish. the first type is always the response type.
    println!("Foo: {:?}", client.get::<Option<String>, _>("foo").await?);

    client.quit().await?;
    Ok(())
}
