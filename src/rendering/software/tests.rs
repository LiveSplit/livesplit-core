use {
    super::render,
    crate::{
        component,
        layout::{self, Component, Layout, LayoutDirection, LayoutState},
        run::parser::{livesplit, llanfair, wsplit},
        tests_helper, Run, Timer,
    },
    image::Rgba,
    img_hash::{HasherConfig, ImageHash},
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

    check(&state, "luCCVRJIPLE=", "default");
}

#[test]
fn actual_split_file() {
    let run = lss("tests/run_files/livesplit1.0.lss");
    let timer = Timer::new(run).unwrap();
    let mut layout = Layout::default_layout();

    check(&layout.state(&timer), "jMDEKxBAPLE=", "actual_split_file");
}

#[test]
fn wsplit() {
    let run = wsplit::parse(file("tests/run_files/wsplit"), false).unwrap();
    let timer = Timer::new(run).unwrap();
    let mut layout = lsl("tests/layout_files/WSplit.lsl");

    check_dims(&layout.state(&timer), [250, 300], "j/jc3dn8t/c=", "wsplit");
}

#[test]
fn all_components() {
    let mut layout = lsl("tests/layout_files/All.lsl");
    let run = lss("tests/run_files/livesplit1.6_gametime.lss");
    let mut timer = Timer::new(run).unwrap();
    tests_helper::start_run(&mut timer);
    tests_helper::make_progress_run_with_splits_opt(
        &mut timer,
        &[Some(10.0), None, Some(20.0), Some(55.0)],
    );

    let state = layout.state(&timer);

    check_dims(&state, [300, 800], "4eH3scnJt7E=", "all_components");

    check_dims(&state, [150, 800], "SWPXSWFFa2s=", "all_components_thin");
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

    check_dims(&state, [300, 400], "jODEIwLQJDc=", "score_split");
}

#[test]
fn dark_layout() {
    let run = llanfair::parse(file("tests/run_files/llanfair")).unwrap();
    let timer = Timer::new(run).unwrap();
    let mut layout = lsl("tests/layout_files/dark.lsl");

    check(&layout.state(&timer), "D8IAQiBYxgM=", "dark_layout");
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
        "8/Pz4/Pz/8c=",
        "subsplits_layout",
    );
}

#[test]
fn display_two_rows() {
    let timer = tests_helper::create_timer(&["A"]);
    let mut layout = Layout::new();
    let mut component = component::text::Component::new();
    let settings = component.settings_mut();
    settings.display_two_rows = true;
    settings.text =
        component::text::Text::Split(String::from("World Record"), String::from("Some Guy"));
    layout.push(component);

    let mut component = component::delta::Component::new();
    component.settings_mut().display_two_rows = true;
    layout.push(component);

    check_dims(
        &layout.state(&timer),
        [200, 100],
        "R00aWs1J9sE=",
        "display_two_rows",
    );
}

#[test]
fn single_line_title() {
    let mut run = tests_helper::create_run(&["A"]);
    run.set_game_name("Some Game");
    run.set_category_name("Some Category");
    run.set_attempt_count(1337);
    let timer = Timer::new(run).unwrap();
    let mut layout = Layout::new();
    let mut component = component::title::Component::new();
    let settings = component.settings_mut();
    settings.display_as_single_line = true;
    settings.show_attempt_count = true;
    settings.show_finished_runs_count = true;
    layout.push(component);

    check_dims(
        &layout.state(&timer),
        [150, 30],
        "QCRZz09hahA=",
        "single_line_title",
    );
}

#[test]
fn horizontal() {
    let run = lss("tests/run_files/Celeste - Any% (1.2.1.5).lss");
    let mut timer = Timer::new(run).unwrap();
    let mut layout = Layout::default_layout();
    layout.general_settings_mut().direction = LayoutDirection::Horizontal;
    match &mut layout.components[1] {
        Component::Splits(splits) => splits.settings_mut().visual_split_count = 4,
        _ => unreachable!("We wanted to configure the splits"),
    }
    layout.push(component::separator::Component::new());
    layout.push(component::graph::Component::new());
    layout.push(component::separator::Component::new());
    layout.push(Box::new(
        component::detailed_timer::Component::with_settings(component::detailed_timer::Settings {
            display_icon: true,
            ..Default::default()
        }),
    ));

    tests_helper::start_run(&mut timer);
    tests_helper::make_progress_run_with_splits_opt(
        &mut timer,
        &[Some(10.0), None, Some(20.0), Some(55.0)],
    );

    check_dims(
        &layout.state(&timer),
        [1500, 40],
        "YmJicmJSUmM=",
        "horizontal",
    );
}

fn get_comparison_tolerance() -> u32 {
    // Without MMX the floating point calculations don't follow IEEE 754, so the tests require a
    // tolerance that is greater than 0.
    // FIXME: We use SSE as an approximation for the cfg because MMX isn't supported by Rust yet.
    if cfg!(all(target_arch = "x86", not(target_feature = "sse"),)) {
        4
    } else {
        0
    }
}

fn check(state: &LayoutState, expected_hash_data: &str, name: &str) {
    check_dims(state, [300, 500], expected_hash_data, name);
}

fn check_dims(state: &LayoutState, dims: [usize; 2], expected_hash_data: &str, name: &str) {
    let image = render(state, dims);
    let hasher = HasherConfig::with_bytes_type::<[u8; 8]>().to_hasher();

    let calculated_hash = hasher.hash_image(&image);
    let calculated_hash_data = calculated_hash.to_base64();
    let expected_hash = ImageHash::<[u8; 8]>::from_base64(expected_hash_data).unwrap();
    let distance = calculated_hash.dist(&expected_hash);

    fs::create_dir_all("target/renders").ok();

    let path = format!(
        "target/renders/{}_{}.png",
        name,
        calculated_hash_data.replace("/", "$"),
    );
    image.save(&path).ok();

    if distance > get_comparison_tolerance() {
        fs::create_dir_all("target/renders/diff").ok();

        let expected_path = format!(
            "target/renders/{}_{}.png",
            name,
            expected_hash_data.replace("/", "$"),
        );
        if let Ok(expected_image) = image::open(expected_path) {
            let mut expected_image = expected_image.to_rgba();
            for (x, y, Rgba([r, g, b, a])) in expected_image.enumerate_pixels_mut() {
                if x < image.width() && y < image.height() {
                    let Rgba([r2, g2, b2, a2]) = image.get_pixel(x, y);
                    *r = (*r as i16).wrapping_sub(*r2 as i16).abs() as u8;
                    *g = (*g as i16).wrapping_sub(*g2 as i16).abs() as u8;
                    *b = (*b as i16).wrapping_sub(*b2 as i16).abs() as u8;
                    *a = (*a).max(*a2);
                }
            }
            let diff_path = format!("target/renders/diff/{}.png", name);
            expected_image.save(&diff_path).ok();
        }

        panic!(
            "Render mismatch for {} expected: {}, actual: {}, distance: {}",
            name, expected_hash_data, calculated_hash_data, distance
        );
    }
}
