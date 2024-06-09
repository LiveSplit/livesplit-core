#![cfg(all(
    any(feature = "software-rendering", feature = "svg-rendering"),
    not(all(target_arch = "x86", not(target_feature = "sse"))),
))]

mod layout_files;
mod run_files;
#[path = "../src/util/tests_helper.rs"]
mod tests_helper;

use livesplit_core::{
    component::{self, timer},
    layout::{self, Component, ComponentState, Layout, LayoutDirection, LayoutState},
    rendering,
    run::parser::{livesplit, llanfair, wsplit},
    settings::ImageCache,
    Run, Segment, TimeSpan, Timer, TimingMethod,
};
use std::{fs, path::PathBuf};

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

    let mut image_cache = ImageCache::new();
    let state = layout.state(&mut image_cache, &timer.snapshot());

    check(
        &state,
        &image_cache,
        "e32259a84233e364",
        "0520a7a32958470a",
        "default",
    );
}

// Font fallback inherently requires fonts from the operating system to
// work. On Windows we have a consistent set of fonts installed for all the
// different languages. We could do the same check on macOS and possibly a
// few other operating systems, which also provide a consistent set of
// fonts, but with a different hash. On Linux however you may have a
// different set of fonts installed, or possibly even none at all, so we
// can't do the same check there.
#[cfg(all(feature = "font-loading", windows))]
#[test]
fn font_fallback() {
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let cur_ver = hklm
        .open_subkey(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion")
        .unwrap();
    let build_number: String = cur_ver.get_value("CurrentBuildNumber").unwrap();
    let build_number: u64 = build_number.parse().unwrap();
    let revision: u32 = cur_ver.get_value("UBR").unwrap();

    if (build_number, revision) < (22631, 3672) {
        // The hash is different before Windows 11 23H2 (end of May 2024 update).
        println!(
            "Skipping font fallback test on Windows with build number {build_number}.{revision}.",
        );
        return;
    }

    // This list is based on the most commonly used writing systems in the
    // world:
    // https://en.wikipedia.org/wiki/List_of_writing_systems#List_of_writing_systems_by_adoption

    let mut run = tests_helper::create_run(&[
        // Emoji
        "❤✔👌🤔😂😁🎉💀🤣",
        // Braille
        "⠃⠗⠁⠊⠇⠇⠑",
        // Hebrew
        "עברית",
        // Arabic
        "اَلْعَرَبِيَّةُ",
        // Dhivehi
        "ދިވެހި",
        // Devanagari
        "देवनागरी",
        // Assamese
        "বাংলা-অসমীয়া",
        // Gujarati
        "ગુજરાતી",
        // Tamil
        "தமிழ்",
        // Telugu
        "తెలుగు",
        // Malayalam
        "മലയാളം",
        // Sinhala
        "සිංහල",
        // Thai
        "ไทย",
        // Burmese
        "မြန်မာ",
        // Hanzi, Kana
        "汉字 漢字 かな カナ",
    ]);
    run.set_game_name("한국어도 돼요"); // Hangul
    run.set_category_name("Кирилица"); // Cyrillic
    run.set_attempt_count(1337);
    let mut timer = Timer::new(run).unwrap();
    let mut layout = Layout::default_layout();

    tests_helper::start_run(&mut timer);
    tests_helper::make_progress_run_with_splits_opt(&mut timer, &[Some(5.0), None, Some(10.0)]);

    let mut image_cache = ImageCache::new();
    let state = layout.state(&mut image_cache, &timer.snapshot());

    check(
        &state,
        &image_cache,
        "924286709a5b32f3",
        "88f140654343de5f",
        "font_fallback",
    );
}

#[test]
fn actual_split_file() {
    let run = lss(run_files::LIVESPLIT_1_0);
    let timer = Timer::new(run).unwrap();
    let mut layout = Layout::default_layout();

    let mut image_cache = ImageCache::new();
    check(
        &layout.state(&mut image_cache, &timer.snapshot()),
        &image_cache,
        "86ccc8595787d41b",
        "ec4e4283ff1aaf7c",
        "actual_split_file",
    );
}

#[test]
fn wsplit() {
    let run = wsplit::parse(run_files::WSPLIT, false).unwrap();
    let timer = Timer::new(run).unwrap();
    let mut layout = lsl(layout_files::WSPLIT);

    let mut image_cache = ImageCache::new();
    check_dims(
        &layout.state(&mut image_cache, &timer.snapshot()),
        &image_cache,
        [250, 300],
        "be70116db88c23c1",
        "cf399969d8ac9baf",
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

    let mut image_cache = ImageCache::new();

    check_dims(
        &layout.state(&mut image_cache, &timer.snapshot()),
        &image_cache,
        [250, 300],
        "fc8e7890593f9da6",
        "0140697763078566",
        "timer_delta_background_ahead",
    );

    timer.reset(true);

    check_dims(
        &layout.state(&mut image_cache, &timer.snapshot()),
        &image_cache,
        [250, 300],
        "c56d1f6715627391",
        "75b3c2a49c0f0b93",
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

    let mut image_cache = ImageCache::new();

    let state = layout.state(&mut image_cache, &timer.snapshot());

    check_dims(
        &state,
        &image_cache,
        [300, 800],
        "7e7aa83a3b80e1da",
        "39b5d1923053c5d9",
        "all_components",
    );

    check_dims(
        &state,
        &image_cache,
        [150, 800],
        "97afa51bfd8a8597",
        "82b26ae781d58b78",
        "all_components_thin",
    );
}

#[test]
fn score_split() {
    let run = lss(run_files::LIVESPLIT_1_0);
    let timer = Timer::new(run).unwrap();
    let mut layout = Layout::default_layout();

    let mut image_cache = ImageCache::new();

    let mut state = layout.state(&mut image_cache, &timer.snapshot());
    let prev_seg = state.components.pop().unwrap();
    state.components.pop();
    let mut timer_state =
        timer::Component::new().state(&timer.snapshot(), layout.general_settings());
    timer_state.time = "50346".into();
    timer_state.fraction = "PTS".into();
    state.components.push(ComponentState::Timer(timer_state));
    state.components.push(prev_seg);

    check_dims(
        &state,
        &image_cache,
        [300, 400],
        "f5dadfe58e621e7b",
        "c7fec6cb25b993bc",
        "score_split",
    );
}

#[test]
fn dark_layout() {
    let run = llanfair::parse(run_files::LLANFAIR).unwrap();
    let timer = Timer::new(run).unwrap();
    let mut layout = lsl(layout_files::DARK);

    let mut image_cache = ImageCache::new();

    check(
        &layout.state(&mut image_cache, &timer.snapshot()),
        &image_cache,
        "a47c590792c1bab5",
        "3f8dfb2da2d43648",
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

    let mut image_cache = ImageCache::new();

    check_dims(
        &layout.state(&mut image_cache, &timer.snapshot()),
        &image_cache,
        [300, 800],
        "39ab965781d0ceee",
        "405baac87e52acc5",
        "subsplits_layout",
    );
}

#[test]
fn background_image() {
    let run = lss(run_files::CELESTE);
    let mut timer = Timer::new(run).unwrap();
    let mut layout = lsl(layout_files::WITH_BACKGROUND_IMAGE);

    tests_helper::start_run(&mut timer);
    tests_helper::make_progress_run_with_splits_opt(
        &mut timer,
        &[Some(10.0), None, Some(20.0), Some(55.0)],
    );

    let mut image_cache = ImageCache::new();

    check_dims(
        &layout.state(&mut image_cache, &timer.snapshot()),
        &image_cache,
        [300, 300],
        "b5238ec57ba70c3a",
        "e4df7276b1603cd5",
        "background_image",
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

    let mut image_cache = ImageCache::new();

    check_dims(
        &layout.state(&mut image_cache, &timer.snapshot()),
        &image_cache,
        [200, 100],
        "d174c2f9a0c54d66",
        "12b6ab31502b1baa",
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

    let mut image_cache = ImageCache::new();

    check_dims(
        &layout.state(&mut image_cache, &timer.snapshot()),
        &image_cache,
        [300, 60],
        "db305e29ec814f33",
        "1848cdb01676cfec",
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

    let mut image_cache = ImageCache::new();

    check_dims(
        &layout.state(&mut image_cache, &timer.snapshot()),
        &image_cache,
        [1500, 40],
        "4a9c12d00233f3c1",
        "9353fac22b4cfde4",
        "horizontal",
    );
}

#[track_caller]
fn check(
    state: &LayoutState,
    image_cache: &ImageCache,
    png_hash: &str,
    svg_hash: &str,
    name: &str,
) {
    check_dims(state, image_cache, [300, 500], png_hash, svg_hash, name);
}

#[track_caller]
fn check_dims(
    state: &LayoutState,
    image_cache: &ImageCache,
    dims: [u32; 2],
    _png_hash: &str,
    _svg_hash: &str,
    name: &str,
) {
    #[cfg(feature = "software-rendering")]
    check_software(state, image_cache, dims, _png_hash, name);
    #[cfg(feature = "svg-rendering")]
    check_svg(state, image_cache, dims, _svg_hash, name);
}

#[cfg(feature = "software-rendering")]
#[track_caller]
fn check_software(
    state: &LayoutState,
    image_cache: &ImageCache,
    dims: [u32; 2],
    expected_hash: &str,
    name: &str,
) {
    let mut renderer = rendering::software::Renderer::new();
    renderer.render(state, image_cache, dims);

    let hash_image = renderer.image();
    let calculated_hash = seahash::hash(&hash_image);
    let calculated_hash = format!("{calculated_hash:016x}");

    let mut path = PathBuf::from_iter(["target", "renders"]);
    fs::create_dir_all(&path).ok();

    let mut actual_path = path.clone();
    actual_path.push(format!("{name}_{calculated_hash}.png"));
    hash_image.save(&actual_path).ok();

    if calculated_hash != expected_hash {
        path.push("diff");
        fs::create_dir_all(&path).ok();
        path.pop();

        let mut expected_path = path.clone();
        expected_path.push(format!("{name}_{expected_hash}.png"));
        let diff_path = if let Ok(expected_image) = image::open(&expected_path) {
            let mut expected_image = expected_image.to_rgba8();
            for (x, y, image::Rgba([r, g, b, a])) in expected_image.enumerate_pixels_mut() {
                if x < hash_image.width() && y < hash_image.height() {
                    let image::Rgba([r2, g2, b2, a2]) = *hash_image.get_pixel(x, y);
                    *r = r.abs_diff(r2);
                    *g = g.abs_diff(g2);
                    *b = b.abs_diff(b2);
                    *a = (*a).max(a2);
                }
            }

            let mut diff_path = path.clone();
            diff_path.push("diff");
            diff_path.push(format!("{name}.png"));
            expected_image.save(&diff_path).ok();
            diff_path
        } else {
            PathBuf::from("Not found")
        };

        panic!(
            "Software render mismatch for {name}
expected: {expected_hash} {}
actual: {calculated_hash} {}
diff: {}",
            expected_path.display(),
            actual_path.display(),
            diff_path.display(),
        );
    }
}

#[cfg(feature = "svg-rendering")]
#[track_caller]
fn check_svg(
    state: &LayoutState,
    image_cache: &ImageCache,
    dims: [u32; 2],
    expected_hash: &str,
    name: &str,
) {
    let mut hash_image = String::new();
    let mut renderer = rendering::svg::Renderer::new();
    renderer
        .render(&mut hash_image, state, image_cache, dims.map(|v| v as f32))
        .unwrap();

    let calculated_hash = seahash::hash(hash_image.as_bytes());
    let calculated_hash = format!("{calculated_hash:016x}");

    let mut path = PathBuf::from_iter(["target", "renders"]);
    fs::create_dir_all(&path).ok();

    let mut actual_path = path.clone();
    actual_path.push(format!("{name}_{calculated_hash}.svg"));
    fs::write(&actual_path, hash_image).ok();

    if calculated_hash != expected_hash {
        path.push("diff");
        fs::create_dir_all(&path).ok();
        path.pop();

        let mut expected_path = path.clone();
        expected_path.push(format!("{name}_{expected_hash}.svg"));

        panic!(
            "SVG render mismatch for {name}
expected: {expected_hash} {}
actual: {calculated_hash} {}",
            expected_path.display(),
            actual_path.display(),
        );
    }
}
