use {
    super::render,
    crate::{
        layout::{self, Layout, LayoutState},
        run::parser::{livesplit, llanfair, wsplit},
        tests_helper, Run, Timer,
    },
    crc::crc32::checksum_ieee,
    std::{
        fs::{self, File},
        io::BufReader,
    },
};

fn file(path: &str) -> BufReader<File> {
    BufReader::new(File::open(path).unwrap())
}

fn lss(path: &str) -> Run {
    livesplit::parse(file(path), None).unwrap()
}

fn lsl(path: &str) -> Layout {
    layout::parser::parse(file(path)).unwrap()
}

#[test]
fn default() {
    let mut run = tests_helper::create_run(&["A", "B", "C", "D"]);
    run.set_game_name("Some Game Name");
    run.set_category_name("Some Category Name");
    run.set_attempt_count(1337);
    let mut timer = Timer::new(run).unwrap();
    let mut layout = Layout::default_layout();

    tests_helper::start_run(&mut timer);
    tests_helper::make_progress_run_with_splits_opt(&mut timer, &[Some(5.0), None, Some(10.0)]);

    let state = layout.state(&timer);

    check(&state, 0xf12f499a, "default");
}

#[test]
fn actual_split_file() {
    let run = lss("tests/run_files/livesplit1.0.lss");
    let timer = Timer::new(run).unwrap();
    let mut layout = Layout::default_layout();

    check(&layout.state(&timer), 0x7ca4c8db, "actual_split_file");
}

#[test]
fn wsplit() {
    let run = wsplit::parse(file("tests/run_files/wsplit"), false).unwrap();
    let timer = Timer::new(run).unwrap();
    let mut layout = lsl("tests/layout_files/WSplit.lsl");

    check_dims(&layout.state(&timer), [250, 300], 0x09afc02f, "wsplit");
}

#[test]
fn all_components() {
    let run = lss("tests/run_files/livesplit1.6_gametime.lss");
    let mut timer = Timer::new(run).unwrap();
    tests_helper::start_run(&mut timer);
    tests_helper::make_progress_run_with_splits_opt(
        &mut timer,
        &[Some(10.0), None, Some(20.0), Some(55.0)],
    );

    let mut layout = lsl("tests/layout_files/All.lsl");

    check_dims(
        &layout.state(&timer),
        [300, 800],
        0xcd09c868,
        "all_components",
    );
}

#[test]
fn score_split() {
    use crate::{component::timer, layout::ComponentState};

    let run = lss("tests/run_files/livesplit1.0.lss");
    let timer = Timer::new(run).unwrap();
    let mut layout = Layout::default_layout();

    let mut state = layout.state(&timer);
    let prev_seg = state.components.pop().unwrap();
    state.components.pop();
    let mut timer_state = timer::Component::new().state(&timer, layout.general_settings());
    timer_state.time = "50346".into();
    timer_state.fraction = "PTS".into();
    state.components.push(ComponentState::Timer(timer_state));
    state.components.push(prev_seg);

    check_dims(&state, [300, 400], 0x2ca22c6d, "score_split");
}

#[test]
fn dark_layout() {
    let run = llanfair::parse(file("tests/run_files/llanfair")).unwrap();
    let timer = Timer::new(run).unwrap();
    let mut layout = lsl("tests/layout_files/dark.lsl");

    check(&layout.state(&timer), 0x54a82761, "dark_layout");
}

#[test]
fn subsplits_layout() {
    let run = lss("tests/run_files/Celeste - Any% (1.2.1.5).lss");
    let mut timer = Timer::new(run).unwrap();
    let mut layout = lsl("tests/layout_files/subsplits.lsl");

    tests_helper::start_run(&mut timer);
    tests_helper::make_progress_run_with_splits_opt(
        &mut timer,
        &[Some(10.0), None, Some(20.0), Some(55.0)],
    );

    check_dims(
        &layout.state(&timer),
        [300, 800],
        0x33fefeb8,
        "subsplits_layout",
    );
}

fn check(state: &LayoutState, expected_checksum: u32, name: &str) {
    check_dims(state, [300, 500], expected_checksum, name);
}

fn check_dims(state: &LayoutState, dims: [usize; 2], expected_checksum: u32, name: &str) {
    let image = render(state, dims);
    let calculated_checksum = checksum_ieee(&image);

    fs::create_dir_all("target/renders").ok();
    let path = format!("target/renders/{}_{:08x}.png", name, calculated_checksum);
    image.save(&path).ok();

    assert_eq!(
        calculated_checksum, expected_checksum,
        "Render mismatch for {} before: {:#08x}, after: {:#08x}",
        name, expected_checksum, calculated_checksum
    );
}
