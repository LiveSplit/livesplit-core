asmjs:
	@cargo rustc --release --target asmjs-unknown-emscripten -- -C opt-level=z
	@cp ../../target/asmjs-unknown-emscripten/release/livesplit*.js* livesplit.js

node: asmjs
	(cd ../bind_gen && cargo run)
	npm install
	npx tsc
	mv node/livesplit_core.d.ts node/index.d.ts
	cp ../bindings/emscripten/livesplit_core.js node/index.js
	cp livesplit.js node/livesplit_core.js
	cp ../../README.md node/README.md
	npx typedoc --mode file --out ../../docs

wasm:
	@-rm ../../target/wasm32-unknown-emscripten/release/deps/*.wasm
	@-rm ../../target/wasm32-unknown-emscripten/release/deps/*.js
	@cargo build --release --target wasm32-unknown-emscripten
	@cp ../../target/wasm32-unknown-emscripten/release/livesplit*.js* livesplit.js
	@cp ../../target/wasm32-unknown-emscripten/release/deps/*.wasm livesplit.wasm

debug:
	@cargo build --target asmjs-unknown-emscripten
	@cp ../../target/asmjs-unknown-emscripten/debug/livesplit*.js* livesplit.js

wasm-unknown:
	@cargo build -p cdylib --target wasm32-unknown-unknown --release
	@cp ../../target/wasm32-unknown-unknown/release/livesplit_core.wasm livesplit_core.wasm

bindings:
	@(cd ../bind_gen && cargo run)

clean:
	@cargo clean
	@rm livesplit*.js*
