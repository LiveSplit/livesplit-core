standard_flags = -s EXPORTED_FUNCTIONS=['_Segment_new','_SegmentList_new','_SegmentList_push','_Run_new','_Run_from_file','_Run_set_game','_Run_set_category','_Timer_new','_Timer_drop','_Timer_start','_Timer_split','_Timer_skip_split','_Timer_undo_split','_Timer_reset','_Timer_pause','_Timer_print_debug','_Timer_save_run_as_lss','_TimerComponent_new','_TimerComponent_drop','_TimerComponent_state','_TitleComponent_new','_TitleComponent_drop','_TitleComponent_state','_SplitsComponent_new','_SplitsComponent_drop','_SplitsComponent_state','_PreviousSegmentComponent_new','_PreviousSegmentComponent_drop','_PreviousSegmentComponent_state'] -s MODULARIZE=1 -s EXPORT_NAME='LiveSplitCore' -s TOTAL_MEMORY=67108864 -s TOTAL_STACK=16777216
debug_flags = $(standard_flags) -g -s ASSERTIONS=1 -s DEMANGLE_SUPPORT=1
release_flags = $(standard_flags) -O3 --memory-init-file 0

asmjs:
	@cargo rustc --release --target asmjs-unknown-emscripten -- -C link-args="$(release_flags)"
	@cp target/asmjs-unknown-emscripten/release/livesplit*.js* .

debug:
	@cargo rustc --target asmjs-unknown-emscripten -- -C link-args="$(debug_flags)"
	@cp target/asmjs-unknown-emscripten/debug/livesplit*.js* .

clean:
	@cargo clean
	@rm livesplit*.js*
