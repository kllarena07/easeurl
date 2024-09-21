# A URL shortener app

I decided to make this project to learn Rust, Redis, and multithreading (yes, the web server is multithreaded) to learn how to handle requests concurrently using Rust

## Crates Used

- Fred (a Redis driver built on top of Tokio)
- Hyper (HTTP framework used to parse requests easier)
- Serde JSON (JSON parsing)
- Rand (for generating random shortened URLs)

# TO DO list
- [x] Connect to Redis client
- [x] Get Redis Client values
- [x] Set Redis Client values
- [ ] HTML client view
  - [ ] URL creation
  - [ ] Real URL obtaining and redirection
  - [ ] Error handling