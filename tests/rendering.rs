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

    let state = layout.state(&timer.snapshot());

    check(&state, "luIAAABAPLM=", "default");
}

#[test]
fn font_fallback() {
    // This list is based on the most commonly used writing systems in the
    // world:
    // https://en.wikipedia.org/wiki/List_of_writing_systems#List_of_writing_systems_by_adoption

    let mut run = tests_helper::create_run(&[
        // FIXME: Unfortunately we can't use emojis because the vendors like to
        // update the look of the emojis every now and then. So for example the
        // emojis changed between Windows 10 and 11. We'd have to either detect
        // the version of the emoji font or would have to detect the operating
        // system version. I believe the latter is something they plan on adding
        // into std, so maybe we can eventually use that.

        // Emoji
        // "❤✔👌🤔😂😁🎉💀🤣",

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
        // Canadian Aboriginal Syllabics
        "ᖃᓂᐅᔮᖅᐸᐃᑦ ᒐᐦᑲᓯᓇᐦᐃᑫᐤ ᑯᖾᖹ ᖿᐟᖻ ᓱᖽᐧᖿ ᑐᑊᘁᗕᑋᗸ",
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

    let _state = layout.state(&timer.snapshot());

    // Font fallback inherently requires fonts from the operating system to
    // work. On Windows we have a consistent set of fonts installed for all the
    // different languages. We could do the same check on macOS and possibly a
    // few other operating systems, which also provide a consistent set of
    // fonts, but with a different hash. On Linux however you may have a
    // different set of fonts installed, or possibly even none at all, so we
    // can't do the same check there.
    #[cfg(all(feature = "font-loading", windows))]
    check(&_state, "zeSAgJgEMbI=", "font_fallback");
}

#[test]
fn actual_split_file() {
    let run = lss(run_files::LIVESPLIT_1_0);
    let timer = Timer::new(run).unwrap();
    let mut layout = Layout::default_layout();

    check(
        &layout.state(&timer.snapshot()),
        "jMDAARBAPLM=",
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
        "j/n8/PnZv/c=",
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
        "a+nRyKBfXc0=",
        "timer_delta_background_ahead",
    );

    timer.reset(true);

    check_dims(
        &layout.state(&timer.snapshot()),
        [250, 300],
        "a+nZyaFfX80=",
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

    check_dims(&state, [300, 800], "4en3ocnJp/E=", "all_components");

    check_dims(&state, [150, 800], "SXfHSWVpRkc=", "all_components_thin");
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

    check_dims(&state, [300, 400], "jOCAAQTABjc=", "score_split");
}

#[test]
fn dark_layout() {
    let run = llanfair::parse(run_files::LLANFAIR).unwrap();
    let timer = Timer::new(run).unwrap();
    let mut layout = lsl(layout_files::DARK);

    check(
        &layout.state(&timer.snapshot()),
        "T8AQQABqwYc=",
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
        "8/vz8/Pz/+c=",
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
        "Q0UaMs1J0sA=",
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
        "ABJtmxt4YZA=",
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

    let mut path = PathBuf::from_iter(["target", "renders"]);
    fs::create_dir_all(&path).ok();

    let mut actual_path = path.clone();
    actual_path.push(format!(
        "{name}_{}.png",
        calculated_hash_data.replace('/', "-"),
    ));
    image.save(&actual_path).ok();

    if distance > get_comparison_tolerance() {
        path.push("diff");
        fs::create_dir_all(&path).ok();
        path.pop();

        let mut expected_path = path.clone();
        expected_path.push(format!(
            "{name}_{}.png",
            expected_hash_data.replace('/', "-"),
        ));
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

            let mut diff_path = path.clone();
            diff_path.push("diff");
            diff_path.push(format!("{name}.png"));
            expected_image.save(&diff_path).ok();
            diff_path
        } else {
            PathBuf::from("Not found")
        };

        panic!(
            "Render mismatch for {name}
expected: {expected_hash_data} {}
actual: {calculated_hash_data} {}
diff: {}
distance: {distance}",
            expected_path.display(),
            actual_path.display(),
            diff_path.display(),
        );
    }
}
