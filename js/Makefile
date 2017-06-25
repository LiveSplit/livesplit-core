asmjs:
	@cargo rustc --release --target asmjs-unknown-emscripten -- -C opt-level=z
	@cp ../target/asmjs-unknown-emscripten/release/livesplit*.js* livesplit.js
	@cat exports.js >> livesplit.js

wasm:
	@-rm ../target/wasm32-unknown-emscripten/release/deps/*.wasm
	@-rm ../target/wasm32-unknown-emscripten/release/deps/*.js
	@cargo build --release --target wasm32-unknown-emscripten
	@cp ../target/wasm32-unknown-emscripten/release/livesplit*.js* livesplit.js
	@cat exports.js >> livesplit.js
	@cp ../target/wasm32-unknown-emscripten/release/deps/*.wasm livesplit.wasm

debug:
	@cargo build --target asmjs-unknown-emscripten
	@cp ../target/asmjs-unknown-emscripten/debug/livesplit*.js* livesplit.js
	@cat exports.js >> livesplit.js

bindings:
	@(cd ../capi/bind_gen && cargo run)

clean:
	@cargo clean
	@rm livesplit*.js*
