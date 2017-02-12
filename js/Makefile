standard_flags = -s MODULARIZE=1 -s EXPORT_NAME='LiveSplitCore' -s TOTAL_MEMORY=33554432 -s ALLOW_MEMORY_GROWTH=1
debug_flags = $(standard_flags) -s ASSERTIONS=1
release_flags = $(standard_flags)

asmjs:
	@CARGO_INCREMENTAL=1 cargo rustc --release --target asmjs-unknown-emscripten -- -C codegen-units=16 -C link-args="$(release_flags)"
	@cp target/asmjs-unknown-emscripten/release/livesplit*.js* .

optimized:
	@cargo rustc --release --target asmjs-unknown-emscripten -- -C codegen-units=16 -C link-args="$(release_flags)"
	@cp target/asmjs-unknown-emscripten/release/livesplit*.js* .

wasm:
	@CARGO_INCREMENTAL=1 cargo rustc --release --target wasm32-unknown-emscripten -- -C no-vectorize-loops -C no-vectorize-slp -C link-args="-fno-vectorize -s SIMD=0 $(release_flags)"
	@cp target/wasm32-unknown-emscripten/release/livesplit*.js* .

debug:
	@CARGO_INCREMENTAL=1 cargo rustc --target asmjs-unknown-emscripten -- -C link-args="$(debug_flags)"
	@cp target/asmjs-unknown-emscripten/debug/livesplit*.js* .

clean:
	@cargo clean
	@rm livesplit*.js*
