# A URL shortener app

I decided to make this project to learn Rust, Redis, and multithreading (Tokio) to learn how to handle requests concurrently using Rust.

## Crates Used

- [fred](https://github.com/aembke/fred.rs)
- [actix](https://actix.rs/)
- [serde-rs/json](https://github.com/serde-rs/json)
- [rand](https://github.com/rust-random/rand)
- [dotenv](https://github.com/dotenv-rs/dotenv)

## TO DO list
- [X] Connect to Redis client
- [X] Get Redis Client values
- [X] Set Redis Client values
- [X] HTML client view
  - [X] URL creation
  - [X] Real URL obtaining and redirection
  - [X] Error handling

## How to run this app yourself
1. Clone this repository
```
git clone https://github.com/kllarena07/url-shortener-redis.git
```
2. Open the cargo binary
```
cd url-shortener redis
```
3. Create a Redis account and cloud/Docker container database
4. Create a `.env` file and define the environment variables
```
REDIS_USERNAME=YOUR_USERNAME_HERE
REDIS_PASSWORD=YOUR_PASSWORD_HERE
REDIS_HOST=YOUR_HOST_HERE
REDIS_PORT=YOUR_PORT_HERE
```
5. Run the binary
```
cargo run
```

## Endpoints:
### Shortened URL Creation

POST to '/create' using the following JSON:
```
{
  "url": "your url here"
}
```
Alternatively, you can simply open `127.0.0.1:3000` in your web browser to access a user-friendly web interface that provides this functionality.

### Getting real URL
GET to '/[shortened_url_id_here]':
- Returns a string to the real url

