# A URL shortener app

I decided to make this project to learn Rust, Redis, and multithreading (yes, the web server is multithreaded) to learn how to handle requests concurrently using Rust

## Crates Used

- Fred (a Redis driver built on top of Tokio)
- Hyper (HTTP framework used to parse requests easier)
- Serde JSON (JSON parsing)
- Rand (for generating random shortened URLs)
- dotenv (using environment variables from .env file)

## TO DO list
- [x] Connect to Redis client
- [x] Get Redis Client values
- [x] Set Redis Client values
- [ ] HTML client view
  - [ ] URL creation
  - [ ] Real URL obtaining and redirection
  - [ ] Error handling

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

POST to '/' using the following JSON:
```
{
  "real_url": "your url here"
}
```

### Getting real URL
GET to '/[shortened_url_id_here]':
- Returns a string to the real url