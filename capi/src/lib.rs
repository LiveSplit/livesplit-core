#![allow(non_snake_case)]

extern crate livesplit_core;
extern crate libc;

use livesplit_core::{Segment, Run, RunMetadata, RunEditor, TimeSpan, Time, Timer, TimerPhase,
                     TimingMethod, SegmentHistory, Attempt, parser, saver};
use livesplit_core::component::{timer, title, splits, previous_segment, sum_of_best,
                                possible_time_save};
use libc::c_char;
use std::ffi::CStr;
use std::cell::{Cell, RefCell};
use std::io::Cursor;
use std::{mem, ptr, slice};
use std::collections::hash_map;

type SegmentHistoryIter = hash_map::Iter<'static, i32, Time>;
type SegmentHistoryElement = (&'static i32, &'static Time);

thread_local! {
    static OUTPUT_STR: RefCell<String> = RefCell::new(String::new());
    static OUTPUT_VEC: RefCell<Vec<u8>> = RefCell::new(Vec::new());
    static TIME_SPAN: Cell<TimeSpan> = Cell::default();
    static TIME: Cell<Time> = Cell::default();
    static SEGMENT_HISTORY_ELEMENT: Cell<SegmentHistoryElement> = Cell::new(unsafe { mem::uninitialized() });
}

fn output_time_span(time_span: TimeSpan) -> *const TimeSpan {
    TIME_SPAN.with(|output| {
        output.set(time_span);
        output.as_ptr() as *const TimeSpan
    })
}

fn output_time(time: Time) -> *const Time {
    TIME.with(|output| {
        output.set(time);
        output.as_ptr() as *const Time
    })
}

fn output_str<S: AsRef<str>>(s: S) -> *const c_char {
    output_str_with(|o| { o.push_str(s.as_ref()); })
}

fn output_str_with<F>(f: F) -> *const c_char
    where F: FnOnce(&mut String)
{
    OUTPUT_STR.with(|output| {
        let mut output = output.borrow_mut();
        output.clear();
        f(&mut output);
        output.push('\0');
        output.as_ptr() as *const c_char
    })
}

fn output_vec<F>(f: F) -> *const c_char
    where F: FnOnce(&mut Vec<u8>)
{
    OUTPUT_VEC.with(|output| {
        let mut output = output.borrow_mut();
        output.clear();
        f(&mut output);
        output.push(0);
        output.as_ptr() as *const c_char
    })
}

unsafe fn str(s: *const c_char) -> &'static str {
    CStr::from_ptr(s).to_str().unwrap()
}

unsafe fn alloc<T>(data: T) -> *mut T {
    Box::into_raw(Box::new(data))
}

unsafe fn own<T>(data: *mut T) -> T {
    *Box::from_raw(data)
}

unsafe fn acc_mut<T>(data: *mut T) -> &'static mut T {
    &mut *data
}

unsafe fn acc<T>(data: *const T) -> &'static T {
    &*data
}


#[no_mangle]
pub unsafe extern "C" fn TimeSpan_clone(this: *const TimeSpan) -> *mut TimeSpan {
    alloc(*acc(this))
}

#[no_mangle]
pub unsafe extern "C" fn TimeSpan_drop(this_drop: *mut TimeSpan) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn TimeSpan_total_seconds(this: *const TimeSpan) -> f64 {
    acc(this).total_seconds()
}

#[no_mangle]
pub unsafe extern "C" fn Time_clone(this: *const Time) -> *mut Time {
    alloc(*acc(this))
}

#[no_mangle]
pub unsafe extern "C" fn Time_drop(this_drop: *mut Time) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn Time_real_time(this: *const Time) -> *const TimeSpan {
    acc(this).real_time.as_ref().map(|t| t as *const _).unwrap_or_else(ptr::null)
}

#[no_mangle]
pub unsafe extern "C" fn Time_game_time(this: *const Time) -> *const TimeSpan {
    acc(this).game_time.as_ref().map(|t| t as *const _).unwrap_or_else(ptr::null)
}

#[no_mangle]
pub unsafe extern "C" fn Time_index(this: *const Time,
                                    timing_method: TimingMethod)
                                    -> *const TimeSpan {
    acc(this)[timing_method].as_ref().map(|t| t as *const _).unwrap_or_else(ptr::null)
}

#[no_mangle]
pub unsafe extern "C" fn Segment_new(name: *const c_char) -> *mut Segment {
    alloc(Segment::new(str(name)))
}

#[no_mangle]
pub unsafe extern "C" fn Segment_drop(this_drop: *mut Segment) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn Segment_name(this: *const Segment) -> *const c_char {
    output_str(acc(this).name())
}

#[no_mangle]
pub unsafe extern "C" fn Segment_icon(this: *const Segment) -> *const c_char {
    output_str(acc(this).icon().url())
}

#[no_mangle]
pub unsafe extern "C" fn Segment_comparison(this: *const Segment,
                                            comparison: *const c_char)
                                            -> *const Time {
    output_time(acc(this).comparison(str(comparison)))
}

#[no_mangle]
pub unsafe extern "C" fn Segment_personal_best_split_time(this: *const Segment) -> *const Time {
    output_time(acc(this).personal_best_split_time())
}

#[no_mangle]
pub unsafe extern "C" fn Segment_best_segment_time(this: *const Segment) -> *const Time {
    output_time(acc(this).best_segment_time())
}

#[no_mangle]
pub unsafe extern "C" fn Segment_segment_history(this: *const Segment) -> *const SegmentHistory {
    acc(this).segment_history()
}

#[no_mangle]
pub unsafe extern "C" fn SegmentHistory_iter(this: *const SegmentHistory)
                                             -> *mut SegmentHistoryIter {
    alloc(acc(this).iter())
}

#[no_mangle]
pub unsafe extern "C" fn SegmentHistoryIter_drop(this_drop: *mut SegmentHistoryIter) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn SegmentHistoryIter_next(this: *mut SegmentHistoryIter)
                                                 -> *const SegmentHistoryElement {
    if let Some(element) = acc_mut(this).next() {
        SEGMENT_HISTORY_ELEMENT.with(|output| {
            output.set(element);
            output.as_ptr() as *const SegmentHistoryElement
        })
    } else {
        ptr::null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn SegmentHistoryElement_index(this: *const SegmentHistoryElement) -> i32 {
    *acc(this).0
}

#[no_mangle]
pub unsafe extern "C" fn SegmentHistoryElement_time(this: *const SegmentHistoryElement)
                                                    -> *const Time {
    acc(this).1
}

#[no_mangle]
pub unsafe extern "C" fn SegmentList_new() -> *mut Vec<Segment> {
    alloc(Vec::new())
}

#[no_mangle]
pub unsafe extern "C" fn SegmentList_push(this: *mut Vec<Segment>, segment_drop: *mut Segment) {
    acc_mut(this).push(own(segment_drop));
}

#[no_mangle]
pub unsafe extern "C" fn Run_new(segments_drop: *mut Vec<Segment>) -> *mut Run {
    alloc(Run::new(own(segments_drop)))
}

#[no_mangle]
pub unsafe extern "C" fn Run_drop(this_drop: *mut Run) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn Run_parse(data: *const u8, length: usize) -> *mut Run {
    match parser::composite::parse(Cursor::new(slice::from_raw_parts(data, length)),
                                   None,
                                   false) {
        Ok(run) => alloc(run),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn Run_game_name(this: *const Run) -> *const c_char {
    output_str(acc(this).game_name())
}

#[no_mangle]
pub unsafe extern "C" fn Run_set_game_name(this: *mut Run, game: *const c_char) {
    acc_mut(this).set_game_name(str(game));
}

#[no_mangle]
pub unsafe extern "C" fn Run_game_icon(this: *const Run) -> *const c_char {
    output_str(acc(this).game_icon().url())
}

#[no_mangle]
pub unsafe extern "C" fn Run_category_name(this: *const Run) -> *const c_char {
    output_str(acc(this).category_name())
}

#[no_mangle]
pub unsafe extern "C" fn Run_set_category_name(this: *mut Run, category: *const c_char) {
    acc_mut(this).set_category_name(str(category));
}

#[no_mangle]
pub unsafe extern "C" fn Run_extended_category_name(this: *const Run,
                                                    show_region: bool,
                                                    show_platform: bool,
                                                    show_variables: bool)
                                                    -> *const c_char {
    output_str(acc(this).extended_category_name(show_region, show_platform, show_variables))
}

#[no_mangle]
pub unsafe extern "C" fn Run_attempt_count(this: *const Run) -> u32 {
    acc(this).attempt_count()
}

#[no_mangle]
pub unsafe extern "C" fn Run_metadata(this: *const Run) -> *const RunMetadata {
    acc(this).metadata()
}

#[no_mangle]
pub unsafe extern "C" fn Run_offset(this: *const Run) -> *const TimeSpan {
    output_time_span(acc(this).offset())
}

#[no_mangle]
pub unsafe extern "C" fn Run_len(this: *const Run) -> usize {
    acc(this).len()
}

#[no_mangle]
pub unsafe extern "C" fn Run_segment(this: *const Run, index: usize) -> *const Segment {
    acc(this).segment(index)
}

#[no_mangle]
pub unsafe extern "C" fn Run_attempt_history_len(this: *const Run) -> usize {
    acc(this).attempt_history().len()
}

#[no_mangle]
pub unsafe extern "C" fn Run_attempt_history_index(this: *const Run,
                                                   index: usize)
                                                   -> *const Attempt {
    &acc(this).attempt_history()[index]
}

#[no_mangle]
pub unsafe extern "C" fn Attempt_index(this: *const Attempt) -> i32 {
    acc(this).index()
}

#[no_mangle]
pub unsafe extern "C" fn Attempt_time(this: *const Attempt) -> *const Time {
    output_time(acc(this).time())
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_run_id(this: *const RunMetadata) -> *const c_char {
    output_str(acc(this).run_id())
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_platform_name(this: *const RunMetadata) -> *const c_char {
    output_str(acc(this).platform_name())
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_uses_emulator(this: *const RunMetadata) -> bool {
    acc(this).uses_emulator()
}

#[no_mangle]
pub unsafe extern "C" fn RunMetadata_region_name(this: *const RunMetadata) -> *const c_char {
    output_str(acc(this).region_name())
}

#[no_mangle]
pub unsafe extern "C" fn Timer_new(run_drop: *mut Run) -> *mut Timer {
    alloc(Timer::new(own(run_drop)))
}

#[no_mangle]
pub unsafe extern "C" fn Timer_drop(this_drop: *mut Timer) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn Timer_start(this: *mut Timer) {
    acc_mut(this).start();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_split(this: *mut Timer) {
    acc_mut(this).split();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_skip_split(this: *mut Timer) {
    acc_mut(this).skip_split();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_undo_split(this: *mut Timer) {
    acc_mut(this).undo_split();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_reset(this: *mut Timer, update_splits: bool) {
    acc_mut(this).reset(update_splits);
}

#[no_mangle]
pub unsafe extern "C" fn Timer_pause(this: *mut Timer) {
    acc_mut(this).pause();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_current_timing_method(this: *const Timer) -> TimingMethod {
    acc(this).current_timing_method()
}

#[no_mangle]
pub unsafe extern "C" fn Timer_set_current_timing_method(this: *mut Timer, method: TimingMethod) {
    acc_mut(this).set_current_timing_method(method);
}

#[no_mangle]
pub unsafe extern "C" fn Timer_current_comparison(this: *const Timer) -> *const c_char {
    output_str(acc(this).current_comparison())
}

#[no_mangle]
pub unsafe extern "C" fn Timer_switch_to_next_comparison(this: *mut Timer) {
    acc_mut(this).switch_to_next_comparison();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_switch_to_previous_comparison(this: *mut Timer) {
    acc_mut(this).switch_to_previous_comparison();
}

#[no_mangle]
pub unsafe extern "C" fn Timer_current_phase(this: *const Timer) -> TimerPhase {
    acc(this).current_phase()
}

#[no_mangle]
pub unsafe extern "C" fn Timer_clone_run(this: *const Timer) -> *mut Run {
    alloc(acc(this).run().clone())
}

#[no_mangle]
pub unsafe extern "C" fn Timer_print_debug(this: *mut Timer) {
    println!("{:#?}", acc_mut(this));
}

#[no_mangle]
pub unsafe extern "C" fn Timer_save_run_as_lss(this: *const Timer) -> *const c_char {
    output_vec(|o| { saver::livesplit::save(acc(this).run(), o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_new() -> *mut timer::Component {
    alloc(timer::Component::new())
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_drop(this_drop: *mut timer::Component) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn TimerComponent_state(this: *const timer::Component,
                                              timer: *const Timer)
                                              -> *const c_char {
    output_vec(|o| { acc(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_new() -> *mut title::Component {
    alloc(title::Component::new())
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_drop(this_drop: *mut title::Component) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn TitleComponent_state(this: *mut title::Component,
                                              timer: *const Timer)
                                              -> *const c_char {
    output_vec(|o| { acc_mut(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_new() -> *mut splits::Component {
    alloc(splits::Component::new())
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_drop(this_drop: *mut splits::Component) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_state(this: *mut splits::Component,
                                               timer: *const Timer)
                                               -> *const c_char {
    output_vec(|o| { acc_mut(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_scroll_up(this: *mut splits::Component) {
    acc_mut(this).scroll_up();
}

#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_scroll_down(this: *mut splits::Component) {
    acc_mut(this).scroll_down();
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_new() -> *mut previous_segment::Component {
    alloc(previous_segment::Component::new())
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_drop(this_drop: *mut previous_segment::Component) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn PreviousSegmentComponent_state(this: *const previous_segment::Component,
                                                        timer: *const Timer)
                                                        -> *const c_char {
    output_vec(|o| { acc(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_new() -> *mut sum_of_best::Component {
    alloc(sum_of_best::Component::new())
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_drop(this_drop: *mut sum_of_best::Component) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn SumOfBestComponent_state(this: *const sum_of_best::Component,
                                                  timer: *const Timer)
                                                  -> *const c_char {
    output_vec(|o| { acc(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponent_new() -> *mut possible_time_save::Component {
    alloc(possible_time_save::Component::new())
}

#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponent_drop(this_drop: *mut possible_time_save::Component) {
    own(this_drop);
}

#[no_mangle]
pub unsafe extern "C" fn PossibleTimeSaveComponent_state(this: *const possible_time_save::Component,
                                                         timer: *const Timer)
                                                         -> *const c_char {
    output_vec(|o| { acc(this).state(acc(timer)).write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_new(run_drop: *mut Run) -> *mut RunEditor {
    alloc(RunEditor::new(own(run_drop)))
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_close(this_drop: *mut RunEditor) -> *mut Run {
    alloc(own(this_drop).close())
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_state(this: *mut RunEditor) -> *const c_char {
    output_vec(|o| { acc_mut(this).state().write_json(o).unwrap(); })
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_select_timing_method(this: *mut RunEditor,
                                                        method: TimingMethod) {
    acc_mut(this).select_timing_method(method);
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_unselect(this: *mut RunEditor, index: usize) {
    acc_mut(this).unselect(index);
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_select_additionally(this: *mut RunEditor, index: usize) {
    acc_mut(this).select_additionally(index);
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_select_only(this: *mut RunEditor, index: usize) {
    acc_mut(this).select_only(index);
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_set_game_name(this: *mut RunEditor, game: *const c_char) {
    acc_mut(this).set_game_name(str(game));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_set_category_name(this: *mut RunEditor,
                                                     category: *const c_char) {
    acc_mut(this).set_category_name(str(category));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_parse_and_set_offset(this: *mut RunEditor,
                                                        offset: *const c_char)
                                                        -> bool {
    acc_mut(this).parse_and_set_offset(str(offset)).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_parse_and_set_attempt_count(this: *mut RunEditor,
                                                               attempts: *const c_char)
                                                               -> bool {
    acc_mut(this).parse_and_set_attempt_count(str(attempts)).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_set_game_icon(this: *mut RunEditor,
                                                 data: *const u8,
                                                 length: usize) {
    acc_mut(this).set_game_icon(slice::from_raw_parts(data, length));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_insert_segment_above(this: *mut RunEditor) {
    acc_mut(this).insert_segment_above();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_insert_segment_below(this: *mut RunEditor) {
    acc_mut(this).insert_segment_below();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_remove_segments(this: *mut RunEditor) {
    acc_mut(this).remove_segments();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_move_segments_up(this: *mut RunEditor) {
    acc_mut(this).move_segments_up();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_move_segments_down(this: *mut RunEditor) {
    acc_mut(this).move_segments_down();
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_set_icon(this: *mut RunEditor,
                                                     data: *const u8,
                                                     length: usize) {
    acc_mut(this).selected_segment().set_icon(slice::from_raw_parts(data, length));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_set_name(this: *mut RunEditor, name: *const c_char) {
    acc_mut(this).selected_segment().set_name(str(name));
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_parse_and_set_split_time(this: *mut RunEditor,
                                                                     time: *const c_char)
                                                                     -> bool {
    acc_mut(this).selected_segment().parse_and_set_split_time(str(time)).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_parse_and_set_segment_time(this: *mut RunEditor,
                                                                       time: *const c_char)
                                                                       -> bool {
    acc_mut(this).selected_segment().parse_and_set_segment_time(str(time)).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_parse_and_set_best_segment_time(this: *mut RunEditor,
                                                                            time: *const c_char)
                                                                            -> bool {
    acc_mut(this).selected_segment().parse_and_set_best_segment_time(str(time)).is_ok()
}

#[no_mangle]
pub unsafe extern "C" fn RunEditor_selected_parse_and_set_comparison_time(
    this: *mut RunEditor, comparison: *const c_char, time: *const c_char) -> bool {
    acc_mut(this)
        .selected_segment()
        .parse_and_set_comparison_time(str(comparison), str(time))
        .is_ok()
}
