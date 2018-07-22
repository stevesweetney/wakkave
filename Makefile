# Compiler configuration
GENERAL_ARGS = --release
FRONTEND_TARGET = $(GENERAL_ARGS) --target wasm32-unknown-unknown
FRONTEND_ARGS = $(FRONTEND_TARGET) --no-default-features --features=frontend
BACKEND_TARGET = $(GENERAL_ARGS)
BACKEND_ARGS = $(BACKEND_TARGET)

checkfrontend:
	cargo check --no-default-features --features frontend
checkbackend:
	cargo check 
backend:
	cargo run --bin backend
frontend:
	cargo web start $(FRONTEND_ARGS) --bin frontend