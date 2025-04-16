# Variables
CARGO = cargo
BUILD_DIR = target
EXECUTABLE = spreadsheet

# Default arguments for the CLI for easy debugging
ARGS ?= 100 100

# Default target
all: build

# Build the project
build:
	$(CARGO) build --release -p cli

# Run the project with prdefined arguments
run: build
	./$(BUILD_DIR)/release/$(EXECUTABLE) $(ARGS)

web:
	cd frontend && trunk serve --open

# Clean the build artifacts
clean:
	$(CARGO) clean

# Test the project
test:
	$(CARGO) test

.PHONY: all build run clean test