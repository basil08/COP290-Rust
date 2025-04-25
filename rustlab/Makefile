# Variables
CARGO = cargo
BUILD_DIR = target
EXECUTABLE = spreadsheet

# Default arguments for the CLI for easy debugging
ARGS ?= 100 100

PORT ?= 8080

# Default target
all: build

# Build the project
build:
	$(CARGO) build --release -p cli

# Run the project with prdefined arguments
run: build
	./$(BUILD_DIR)/release/$(EXECUTABLE) $(ARGS)

server:
	cd dummy_server && cargo run

client:
	@echo "Serving on port $(PORT)..."
	cd frontend && trunk serve --port $(PORT) 

# Clean the build artifacts
clean:
	$(CARGO) clean

# Test the project
test:
	$(CARGO) test

docs:
	$(CARGO) doc --open

.PHONY: all build run clean test docs
