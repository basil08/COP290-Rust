# COP290 Rust Lab

This is the top-level README explaining how to run the base as well as three extensions on top of it.

## TUI


## Web Application


## WebSocket and CRDT based Web Application

The app is distributed among `server/`, `ws_client`, and `crdt` 

To start server, run

```
$ cd rustlab/server
$ RUST_LOG=info cargo run
```

The default port is 3030. It can be changed as follows

```
$ RUST_LOG=info cargo run <port_number>
```

To start client, make sure `trunk` is installed.

```
$ cargo install trunk
```

Then, run

```
$ cd rustlab/ws_client
$ trunk serve
```

This will start the client at port 8080.

Browse to `http::/localhost:8080` to access the app.

