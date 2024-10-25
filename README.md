# EaseURL: a URL shortener app

I decided to make this project to learn Rust, Redis, and multithreading (Tokio) to learn how to handle requests concurrently using Rust.

Live deployment on https://ease-url-bgrv.shuttle.app/

NOTE: app may not work due to Redis's new pricing policy. If the database is inactive and no one uses it for a while, the database will be deleted. Due to the nature of this being a side project, that likelihood of that happening is fairly high.

## Crates Used

- [fred](https://github.com/aembke/fred.rs)
- [actix](https://actix.rs/)
- [serde-rs/json](https://github.com/serde-rs/json)
- [rand](https://github.com/rust-random/rand)
- [shuttle](https://www.shuttle.dev/)

## TO DO list
- [X] Connect to Redis client
- [X] Get Redis Client values
- [X] Set Redis Client values
- [X] HTML client view
  - [X] URL creation
  - [X] Real URL obtaining and redirection
  - [X] Error handling

