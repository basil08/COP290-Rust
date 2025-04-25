# COP290 Rust Lab

Proposal doc [here](https://docs.google.com/document/d/1ubq7qUoZWUW3Z5_JWA7752A7dIp099BpinfH_-0RWpw/edit?tab=t.20peh7ytu0r9#heading=h.9zlaqa1egvcl)

This is the top-level README explaining how to run the base as well as three extensions on top of it.
## TUI
# Standard Mode

```
$ cd rustlab
$ make 
$ ./target/release/spreadsheet 10 10
```
# Extended Mode
```
$ cd rustlab
$ make 
$ ./target/release/spreadsheet -extended 10 10
```

UNDO
```
> undo
```

REDO
```
> redo
```

AUTOFILL
```
> =autofill <column name> <length>
```

ASSIGN STRING TO A CELL
```
> <cell>="<content>"
```


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

