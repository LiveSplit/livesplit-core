docs: bindings
	npm install
	npx typedoc --out ../../docs ../bindings/wasm/livesplit_core.ts

bindings:
	@(cd ../bind_gen && cargo run)
