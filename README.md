# basic_reverse_proxy
a simple reverse proxy server with an in memory cache possesing a TTL, to maintain the simplicity reverse proxy only expects GET requests to the index path ("/"). All the other request
types return to 404 Not Found. If the GET request doesn't have the origin server as the query parameter,
it returns to 400 Bad Request.

#### Start Server

```
cargo run
```

#### Example Query
Specify the origin server in the query parameters with a key of `url`
```
curl localhost:9000/?url=https://blockstream.info/api/blocks/0
```

#### Run Tests
```
cargo test
```
