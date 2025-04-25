# Variables
CARGO = cargo
BUILD_DIR = target
EXECUTABLE = spreadsheet
LATEX = pdflatex
BIBER = biber

# Default arguments for the CLI for easy debugging
ARGS ?= 100 100

PORT ?= 8080

# Terminal command based on OS
ifeq ($(OS),Windows_NT)
    TERMINAL = start
else
    TERMINAL = gnome-terminal -- 
endif

# Default target
all: build

# Build the project
build:
	$(CARGO) build --release -p cli

# Run the project with prdefined arguments
run: build
	./$(BUILD_DIR)/release/$(EXECUTABLE) $(ARGS)

# Run extension 1 in a new terminal
ext1: build
	./$(BUILD_DIR)/release/$(EXECUTABLE) -extended $(ARGS)

# Run server in background
server:
	cd dummy_server && cargo run &

# Run client in foreground
client:
	@echo "Serving on port $(PORT)..."
	cd frontend && trunk serve --port $(PORT)

# Run extension 2 with server in background and client in foreground
ext2: server client

# Run websocket server in background
ws_server:
	@echo "Serving websocket server"
	cd server && nohup cargo run > server.log 2>&1 &
	@echo "Server started in background. Logs in server.log"

# Run websocket client in foreground
ws_client:
	@echo "Serving websocket client"
	cd ws_client && trunk serve

# Run extension 3 with websocket server in background and client in foreground
ext3: ws_server ws_client

# Clean LaTeX intermediate files
clean-latex:
	@echo "Cleaning LaTeX intermediate files..."
	cd docs && rm -f *.aux *.bbl *.blg *.log *.out *.toc *.fls *.fdb_latexmk *.synctex.gz *.bcf

# Clean the build artifacts
clean: clean-latex
	$(CARGO) clean

# Test the project
test:
	$(CARGO) test

# Check and install required LaTeX packages
check-latex-deps:
	@echo "Checking LaTeX dependencies..."
	@if ! kpsewhich mdframed.sty > /dev/null 2>&1 || ! kpsewhich adforn.sty > /dev/null 2>&1 || ! kpsewhich matlab-prettifier.sty > /dev/null 2>&1 || ! kpsewhich biblatex.sty > /dev/null 2>&1 || ! command -v biber > /dev/null 2>&1; then \
		echo "Installing required LaTeX packages..."; \
		sudo apt-get update && sudo apt-get install -y texlive-latex-extra texlive-fonts-extra texlive-science texlive-bibtex-extra biber; \
	fi

# Generate documentation
docs: check-latex-deps latex-docs cargo-docs

latex-docs:
	@echo "Generating LaTeX documentation..."
	cd docs && \
	TEXINPUTS=.:./tau-class:./figures: $(LATEX) main.tex && \
	$(BIBER) main && \
	TEXINPUTS=.:./tau-class:./figures: $(LATEX) main.tex && \
	TEXINPUTS=.:./tau-class:./figures: $(LATEX) main.tex && \
	mv main.pdf ../report.pdf

coverage:
	cd sheet && cargo tarpaulin --out Html
cargo-docs:
	@echo "Generating Rust documentation..."
	$(CARGO) doc --open

.PHONY: all build run ext1 ext2 ext3 server client ws_server ws_client clean clean-latex test docs latex-docs cargo-docs check-latex-deps
