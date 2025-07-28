PS = powershell -Command

.PHONY: build
build: 
	$(PS) cd engine; $(PS) cargo build --release
	$(PS) cp engine/target/release/engine.dll package/src/rpa_engine

all: build