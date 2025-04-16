# Variables
CARGO = cargo
BUILD_DIR = target
EXECUTABLE = rustlab

# Default target
all: build

# Build the project
build:
	$(CARGO) build --release

# Run the project
run: build
	./$(BUILD_DIR)/release/$(EXECUTABLE)

web:
	cd frontend && trunk serve

# Clean the build artifacts
clean:
	$(CARGO) clean

# Test the project
test:
	$(CARGO) test

.PHONY: all build run clean test