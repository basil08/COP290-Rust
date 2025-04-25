# COP290 Rust Lab

## Run

1. `make` builds base TUI
2. `make run` runs base TUI
2. `make ext1` builds and runs extended TUI
3. `make ext2` builds and runs web application
4. `make ext3` builds and runs WebSocket and CRDT based web application

## Demo

WebSocket and CRDT-based web application: https://drive.google.com/file/d/1wHeTSfXqq1nNGKBzuj7Uh3y8yFwbZ7H6/view?usp=sharing

Please see rustlab/README.md for instructions to run the different extensions and base app.
Makefile is inside rustlab/.

## Report

Running make docs inside rustlab/ generates a pdf report from latex source inside docs/ and builds rust documentation for the project. WARNING: The report uses a lot of extra fonts and needs a large disk space to download the font packages if not already in the system. For this reason, we have built and given a report at rustlab/docs/COP290_Report.pdf


Stray process kill command: sudo kill -9 $(lsof -t -i:<port>)
