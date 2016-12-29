standard_flags = -s EXPORTED_FUNCTIONS=['_Segment_new','_SegmentList_new','_SegmentList_push','_Run_new','_Timer_new','_Timer_drop','_Timer_start','_Timer_split','_Timer_skip_split','_Timer_undo_split','_Timer_reset','_Timer_pause','_Timer_print_debug'] -s MODULARIZE=1 -s EXPORT_NAME='LiveSplitCore'
debug_flags = $(standard_flags) -g -s ASSERTIONS=2 -s DEMANGLE_SUPPORT=1
release_flags = $(standard_flags) -O3 --memory-init-file 0

asmjs:
	@cargo rustc --release --target asmjs-unknown-emscripten -- -C link-args="$(release_flags)"
	@cp target/asmjs-unknown-emscripten/release/deps/livesplit*.js.mem target/asmjs-unknown-emscripten/release/.
	@cp target/asmjs-unknown-emscripten/release/livesplit*.js* .
	@tsc
	@cat bindings.js >> livesplit.js
	@rm bindings.js

debug:
	@cargo rustc --target asmjs-unknown-emscripten -- -C link-args="$(debug_flags)"
	@cp target/asmjs-unknown-emscripten/debug/livesplit*.js* .
	@tsc

clean:
	@cargo clean
	@rm livesplit*.js*
