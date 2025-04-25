# COP290 Rust Lab

## Github
Repo link: https://github.com/basil08/COP290-Rust

## Run

0. `cd rustlab`
1. `make` builds base TUI
2. `make run` runs base TUI
2. `make ext1` builds and runs extended TUI
3. `make ext2` builds and runs web application
4. `make ext3` builds and runs WebSocket and CRDT based web application

## Demo


Demo video for Client Server: https://csciitd-my.sharepoint.com/:v:/g/personal/ee1230978_iitd_ac_in/ERheCVFbamNGuioyMfLU_qoBPc0Bi7PNp76wGh4ko9E9XQ?e=MqsM8R


WebSocket and CRDT-based web application: https://drive.google.com/file/d/1wHeTSfXqq1nNGKBzuj7Uh3y8yFwbZ7H6/view?usp=sharing

Please see below for detailed commands on how to run each extension.

Makefile is inside rustlab/.

## Report

Running make docs inside rustlab/ generates a pdf report from latex source inside docs/ and builds rust documentation for the project. WARNING: The report uses a lot of extra fonts and needs a large disk space to download the font packages if not already in the system. For this reason, we have built and given a report at rustlab/docs/COP290_Report.pdf


Stray process kill command: sudo kill -9 $(lsof -t -i:<port>)

---

  
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

Demo video [here](https://csciitd-my.sharepoint.com/:v:/g/personal/ee1230978_iitd_ac_in/ERheCVFbamNGuioyMfLU_qoBPc0Bi7PNp76wGh4ko9E9XQ?e=MqsM8R)

This is GUI with client-server model, 
server will start on a fixed port (i.e 3001)
clients can be initiated in different ports

The app is distributed among `frontend`, `dummy_server` 

To start server, run (The default port is 3001)
```
$ cd rustlab/dummy_server
$ cargo run
```

To start client, make sure `trunk` is installed.
```
$ cargo install trunk
```

Then, run
multiple client can also be running simultaneously
```
cd rustlab/frontend
trunk server --port <port_number>
```

Browse to `http::/localhost:<port_number>` to access the app.

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
