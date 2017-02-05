standard_flags = -s EXPORTED_FUNCTIONS=['_Segment_new','_SegmentList_new','_SegmentList_push','_Run_new','_Run_parse','_Run_set_game','_Run_set_category','_Timer_new','_Timer_drop','_Timer_start','_Timer_split','_Timer_skip_split','_Timer_undo_split','_Timer_reset','_Timer_pause','_Timer_current_timing_method','_Timer_set_current_timing_method','_Timer_current_comparison','_Timer_switch_to_next_comparison','_Timer_switch_to_previous_comparison','_Timer_print_debug','_Timer_save_run_as_lss','_TimerComponent_new','_TimerComponent_drop','_TimerComponent_state','_TitleComponent_new','_TitleComponent_drop','_TitleComponent_state','_SplitsComponent_new','_SplitsComponent_drop','_SplitsComponent_state','_SplitsComponent_scroll_up','_SplitsComponent_scroll_down','_PreviousSegmentComponent_new','_PreviousSegmentComponent_drop','_PreviousSegmentComponent_state','_SumOfBestComponent_new','_SumOfBestComponent_drop','_SumOfBestComponent_state','_PossibleTimeSaveComponent_new','_PossibleTimeSaveComponent_drop','_PossibleTimeSaveComponent_state'] -s MODULARIZE=1 -s EXPORT_NAME='LiveSplitCore' -s TOTAL_MEMORY=33554432 -s TOTAL_STACK=33554432 -s ALLOW_MEMORY_GROWTH=1
debug_flags = $(standard_flags) -g -s ASSERTIONS=1 -s DEMANGLE_SUPPORT=1
release_flags = $(standard_flags) -O3 --memory-init-file 0

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
