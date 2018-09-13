# Compiler configuration
GENERAL_ARGS = --release
FRONTEND_TARGET = --target wasm32-unknown-unknown
FRONTEND_ARGS = $(FRONTEND_TARGET) --no-default-features --features=frontend
BACKEND_TARGET = $(GENERAL_ARGS)
BACKEND_ARGS = $(BACKEND_TARGET)

.PHONY: \
		checkfrontend \
		checkbackend \
		backend \
		frontend \
		flow 

checkfrontend:
	cargo check -p frontend
checkbackend:
	cargo check -p backend 
backend:
	cargo run -p backend --bin backend
frontend:
	cargo build -p frontend $(FRONTEND_TARGET)
flow:
	node_modules/.bin/flow check