#![cfg(feature = "software-rendering")]

mod layout_files;
mod run_files;
#[path = "../src/util/tests_helper.rs"]
mod tests_helper;

use image::Rgba;
use img_hash::{HasherConfig, ImageHash};
use livesplit_core::{
    component::{self, timer},
    layout::{self, Component, ComponentState, Layout, LayoutDirection, LayoutState},
    rendering::software::Renderer,
    run::parser::{livesplit, llanfair, wsplit},
    Run, Segment, TimeSpan, Timer, TimingMethod,
};
use std::fs;

fn lss(data: &str) -> Run {
    livesplit::parse(data).unwrap()
}

fn lsl(data: &str) -> Layout {
    layout::parser::parse(data).unwrap()
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

    let state = layout.state(&timer.snapshot());

    check(&state, "luoAAABANDM=", "default");
}

#[test]
fn actual_split_file() {
    let run = lss(run_files::LIVESPLIT_1_0);
    let timer = Timer::new(run).unwrap();
    let mut layout = Layout::default_layout();

    check(
        &layout.state(&timer.snapshot()),
        "jMTAARBAPLM=",
        "actual_split_file",
    );
}

#[test]
fn wsplit() {
    let run = wsplit::parse(run_files::WSPLIT, false).unwrap();
    let timer = Timer::new(run).unwrap();
    let mut layout = lsl(layout_files::WSPLIT);

    check_dims(
        &layout.state(&timer.snapshot()),
        [250, 300],
        "j/n83PnZv/c=",
        "wsplit",
    );
}

#[test]
fn timer_delta_background() {
    let run = lss(run_files::LIVESPLIT_1_0);
    let mut timer = Timer::new(run).unwrap();
    let mut layout = lsl(layout_files::WITH_TIMER_DELTA_BACKGROUND);
    tests_helper::start_run(&mut timer);
    tests_helper::make_progress_run_with_splits_opt(&mut timer, &[Some(5.0), None, Some(10.0)]);

    check_dims(
        &layout.state(&timer.snapshot()),
        [250, 300],
        "a+nRyOCfXU0=",
        "timer_delta_background_ahead",
    );

    timer.reset(true);

    check_dims(
        &layout.state(&timer.snapshot()),
        [250, 300],
        "a+nZyeGfW80=",
        "timer_delta_background_stopped",
    );
}

#[test]
fn all_components() {
    let mut layout = lsl(layout_files::ALL);
    let run = lss(run_files::LIVESPLIT_1_6_GAMETIME);
    let mut timer = Timer::new(run).unwrap();
    tests_helper::start_run(&mut timer);
    tests_helper::make_progress_run_with_splits_opt(
        &mut timer,
        &[Some(10.0), None, Some(20.0), Some(55.0)],
    );

    let state = layout.state(&timer.snapshot());

    check_dims(&state, [300, 800], "4en3ocnJJ/E=", "all_components");

    check_dims(&state, [150, 800], "SXfHSWVpRlc=", "all_components_thin");
}

#[test]
fn score_split() {
    let run = lss(run_files::LIVESPLIT_1_0);
    let timer = Timer::new(run).unwrap();
    let mut layout = Layout::default_layout();

    let mut state = layout.state(&timer.snapshot());
    let prev_seg = state.components.pop().unwrap();
    state.components.pop();
    let mut timer_state =
        timer::Component::new().state(&timer.snapshot(), layout.general_settings());
    timer_state.time = "50346".into();
    timer_state.fraction = "PTS".into();
    state.components.push(ComponentState::Timer(timer_state));
    state.components.push(prev_seg);

    check_dims(&state, [300, 400], "jOCAAQTAJjc=", "score_split");
}

#[test]
fn dark_layout() {
    let run = llanfair::parse(run_files::LLANFAIR).unwrap();
    let timer = Timer::new(run).unwrap();
    let mut layout = lsl(layout_files::DARK);

    check(
        &layout.state(&timer.snapshot()),
        "T8QIQABIw4c=",
        "dark_layout",
    );
}

#[test]
fn subsplits_layout() {
    let run = lss(run_files::CELESTE);
    let mut timer = Timer::new(run).unwrap();
    let mut layout = lsl(layout_files::SUBSPLITS);

    tests_helper::start_run(&mut timer);
    tests_helper::make_progress_run_with_splits_opt(
        &mut timer,
        &[Some(10.0), None, Some(20.0), Some(55.0)],
    );

    check_dims(
        &layout.state(&timer.snapshot()),
        [300, 800],
        "8/vz6/Pz/+c=",
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
        &layout.state(&timer.snapshot()),
        [200, 100],
        "QU0aWs1J0sA=",
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
        &layout.state(&timer.snapshot()),
        [300, 60],
        "ABJkm0toYZA=",
        "single_line_title",
    );
}

#[test]
fn horizontal() {
    let run = lss(run_files::CELESTE);
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
        &layout.state(&timer.snapshot()),
        [1500, 40],
        "YnJjcnJSUmM=",
        "horizontal",
    );
}

fn get_comparison_tolerance() -> u32 {
    // Without MMX the floating point calculations don't follow IEEE 754, so the tests require a
    // tolerance that is greater than 0.
    // FIXME: We use SSE as an approximation for the cfg because MMX isn't supported by Rust yet.
    if cfg!(all(target_arch = "x86", not(target_feature = "sse"))) {
        3
    } else {
        0
    }
}

#[track_caller]
fn check(state: &LayoutState, expected_hash_data: &str, name: &str) {
    check_dims(state, [300, 500], expected_hash_data, name);
}

#[track_caller]
fn check_dims(
    state: &LayoutState,
    dims @ [width, height]: [u32; 2],
    expected_hash_data: &str,
    name: &str,
) {
    let mut renderer = Renderer::new();
    renderer.render(state, dims);
    let hash_image =
        img_hash::image::RgbaImage::from_raw(width, height, renderer.into_image_data()).unwrap();
    let image =
        image::ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, hash_image.as_raw().as_slice())
            .unwrap();

    let hasher = HasherConfig::with_bytes_type::<[u8; 8]>().to_hasher();

    let calculated_hash = hasher.hash_image(&hash_image);
    let calculated_hash_data = calculated_hash.to_base64();
    let expected_hash = ImageHash::<[u8; 8]>::from_base64(expected_hash_data).unwrap();
    let distance = calculated_hash.dist(&expected_hash);

    fs::create_dir_all("target/renders").ok();

    let path = format!(
        "target/renders/{name}_{}.png",
        calculated_hash_data.replace('/', "-"),
    );
    image.save(&path).ok();

    if distance > get_comparison_tolerance() {
        fs::create_dir_all("target/renders/diff").ok();

        let expected_path = format!(
            "target/renders/{name}_{}.png",
            expected_hash_data.replace('/', "-"),
        );
        let diff_path = if let Ok(expected_image) = image::open(&expected_path) {
            let mut expected_image = expected_image.to_rgba8();
            for (x, y, Rgba([r, g, b, a])) in expected_image.enumerate_pixels_mut() {
                if x < hash_image.width() && y < hash_image.height() {
                    let img_hash::image::Rgba([r2, g2, b2, a2]) = *hash_image.get_pixel(x, y);
                    *r = r.abs_diff(r2);
                    *g = g.abs_diff(g2);
                    *b = b.abs_diff(b2);
                    *a = (*a).max(a2);
                }
            }
            let diff_path = format!("target/renders/diff/{name}.png");
            expected_image.save(&diff_path).ok();
            diff_path
        } else {
            String::from("Not found")
        };

        panic!(
            "Render mismatch for {name}
expected: {expected_hash_data} {expected_path}
actual: {calculated_hash_data} {path}
diff: {diff_path}
distance: {distance}",
        );
    }
}
