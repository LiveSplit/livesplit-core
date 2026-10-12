#![cfg(feature = "default-text-engine")]

#[path = "../src/util/tests_helper.rs"]
mod tests_helper;

use livesplit_core::{
    Lang, Run, Segment, TimeSpan, Timer, TimingMethod,
    component::{detailed_timer, timer},
    layout::{ComponentState, Layout},
    rendering::{
        Entity, FontKind, Image, Label, PathBuilder, ResourceAllocator, SceneManager,
        SharedOwnership, Transform,
        default_text_engine::{self, TextEngine},
    },
    settings::{Font, Image as SettingsImage, ImageCache},
    timing::formatter::Accuracy,
};

// Use the real font measurements, while discarding paths that aren't needed
// for checking the positions and sizes in the scene.
struct EmptyPath;

impl PathBuilder for EmptyPath {
    type Path = ();
    fn move_to(&mut self, _: f32, _: f32) {}
    fn line_to(&mut self, _: f32, _: f32) {}
    fn quad_to(&mut self, _: f32, _: f32, _: f32, _: f32) {}
    fn curve_to(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: f32) {}
    fn close(&mut self) {}
    fn finish(self) {}
}

impl SharedOwnership for EmptyPath {
    fn share(&self) -> Self {
        Self
    }
}

impl Image for EmptyPath {
    fn aspect_ratio(&self) -> f32 {
        1.0
    }
}

struct Allocator(TextEngine<()>, usize);

impl ResourceAllocator for Allocator {
    type PathBuilder = EmptyPath;
    type Path = ();
    type Image = EmptyPath;
    type Font = default_text_engine::Font;
    type Label = default_text_engine::Label<()>;

    fn path_builder(&mut self) -> EmptyPath {
        EmptyPath
    }
    fn create_image(&mut self, data: &[u8]) -> Option<EmptyPath> {
        (!data.is_empty()).then_some(EmptyPath)
    }
    fn create_font(&mut self, font: Option<&Font>, kind: FontKind) -> Self::Font {
        self.0.create_font(font, kind)
    }
    fn create_label(
        &mut self,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) -> Self::Label {
        self.1 += usize::from(matches!(text, "8:88:88" | ".88"));
        self.0.create_label(|| EmptyPath, text, font, max_width)
    }
    fn update_label(
        &mut self,
        label: &mut Self::Label,
        text: &str,
        font: &mut Self::Font,
        max_width: Option<f32>,
    ) {
        self.1 += usize::from(matches!(text, "8:88:88" | ".88"));
        self.0
            .update_label(|| EmptyPath, label, text, font, max_width);
    }
}

type Manager =
    SceneManager<(), EmptyPath, default_text_engine::Font, default_text_engine::Label<()>>;

#[track_caller]
fn timer_labels(manager: &Manager) -> [(Transform, f32); 2] {
    let labels: Vec<_> = manager
        .scene()
        .bottom_layer()
        .iter()
        .chain(manager.scene().top_layer())
        .filter_map(|entity| match entity {
            Entity::Label(label, _, _, transform) => {
                Some((*transform, label.width(transform.scale_x)))
            }
            _ => None,
        })
        .collect();
    labels.try_into().ok().unwrap()
}

#[track_caller]
fn assert_fits(manager: &Manager, [width, height]: [f32; 2]) -> f32 {
    let [(fraction, fraction_width), (time, time_width)] = timer_labels(manager);
    for (transform, label_width) in [(fraction, fraction_width), (time, time_width)] {
        assert!(transform.x >= -0.001, "left edge: {}", transform.x);
        assert!(transform.x + label_width <= width + 0.001);
        assert!(transform.scale_x.is_finite());
        assert!(transform.scale_y <= height);
        assert!(transform.y >= 0.0 && transform.y <= height);
    }
    assert!((fraction.scale_x - 0.7 * time.scale_x).abs() < 0.001);
    assert!((time.x + time_width - fraction.x).abs() < 0.001);
    assert!((time.y - fraction.y).abs() < 0.001);
    time.scale_x
}

#[test]
fn remains_stable_through_expected_digit_growth_and_fits_overruns() {
    let mut timer = tests_helper::create_timer(&["A"]);
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    let mut image_cache = ImageCache::new();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    let dims = [160.0, 120.0];

    tests_helper::start_run(&mut timer);
    let mut expected_scale = None;
    for seconds in [0.0, 1.0, 9.0, 10.0, 59.0, 60.0, 600.0, 3600.0] {
        timer
            .set_game_time(TimeSpan::from_seconds(seconds))
            .unwrap();
        let state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
        manager.update_scene(&mut allocator, dims, &state, &image_cache);
        let scale = assert_fits(&manager, dims);
        assert!(scale < dims[1]);
        assert_eq!(*expected_scale.get_or_insert(scale), scale);
    }
    // The two canonical hints are shaped once and reused on subsequent frames.
    assert_eq!(allocator.1, 2);
    for seconds in [36000.0, 360000.0, 3600000.0] {
        timer
            .set_game_time(TimeSpan::from_seconds(seconds))
            .unwrap();
        let state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
        manager.update_scene(&mut allocator, dims, &state, &image_cache);
        let scale = assert_fits(&manager, dims);
        assert!(scale < expected_scale.replace(scale).unwrap());
    }
}

#[test]
fn fits_negative_times_all_accuracies_custom_fonts_and_window_resizes() {
    let mut timer = tests_helper::create_timer(&["A"]);
    let mut component = timer::Component::new();
    let mut image_cache = ImageCache::new();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    let mut layout = Layout::new();
    layout.general_settings_mut().timer_font = Some(Font {
        family: "Fira Sans".into(),
        ..Default::default()
    });
    tests_helper::start_run(&mut timer);
    for (seconds, accuracy) in [
        (-0.1, Accuracy::Seconds),
        (-60.123, Accuracy::Tenths),
        (-360000.123, Accuracy::Hundredths),
        (-360000.123, Accuracy::Milliseconds),
    ] {
        timer
            .set_game_time(TimeSpan::from_seconds(seconds))
            .unwrap();
        component.settings_mut().accuracy = accuracy;
        for lang in [
            Lang::English,
            #[cfg(feature = "localization")]
            Lang::German,
        ] {
            let mut state = layout.state(&mut image_cache, &timer.snapshot(), lang);
            state.components.push(ComponentState::Timer(component.state(
                &timer.snapshot(),
                layout.general_settings(),
                lang,
            )));
            for dims in [[160.0, 120.0], [80.0, 120.0], [320.0, 60.0], [1.0, 120.0]] {
                manager.update_scene(&mut allocator, dims, &state, &image_cache);
                assert_fits(&manager, dims);
            }
        }
    }
}

#[test]
fn keeps_configured_size_when_there_is_room() {
    let timer = tests_helper::create_timer(&["A"]);
    let mut layout = Layout::new();
    layout.push(timer::Component::new());
    let mut image_cache = ImageCache::new();
    let state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    let dims = [1000.0, 60.0];
    manager.update_scene(&mut allocator, dims, &state, &image_cache);
    assert!((assert_fits(&manager, dims) - 60.0).abs() < 0.001);
}

#[track_caller]
fn assert_detailed_timer_fits(manager: &Manager, dims: [f32; 2], comparisons: usize) -> f32 {
    // Keep the ticking timers in the top layer so the comparison labels can
    // be checked independently of the main timer above them.
    let timer_labels: Vec<_> = manager
        .scene()
        .top_layer()
        .iter()
        .filter_map(|entity| match entity {
            Entity::Label(label, _, _, transform) => {
                Some((*transform, label.width(transform.scale_x)))
            }
            _ => None,
        })
        .collect();
    assert_eq!(timer_labels.len(), 4);
    for (transform, width) in &timer_labels {
        assert!(transform.scale_x.is_finite() && transform.scale_x > 0.0);
        assert!(transform.x >= -0.001);
        assert!(transform.x + width <= dims[0] + 0.001);
        assert!(transform.y > 0.0 && transform.y <= dims[1]);
    }
    let [(fraction, fraction_width), (time, time_width)] = timer_labels[2..] else {
        unreachable!();
    };
    assert!(fraction_width >= 0.0);
    assert!((fraction.scale_x - 0.7 * time.scale_x).abs() < 0.001);
    assert!((time.x + time_width - fraction.x).abs() < 0.001);

    let top_height = dims[1] * 40.0 / 65.0;
    let comparison_labels: Vec<_> = manager
        .scene()
        .bottom_layer()
        .iter()
        .filter_map(|entity| match entity {
            Entity::Label(label, _, _, transform) if transform.y > top_height => {
                Some((*transform, label.width(transform.scale_x)))
            }
            _ => None,
        })
        .collect();
    assert_eq!(comparison_labels.len(), 2 * comparisons);
    let mut name_end = 0.0_f32;
    for (index, (transform, width)) in comparison_labels.iter().enumerate() {
        assert!(transform.scale_x.is_finite() && transform.scale_x > 0.0);
        assert!(transform.x >= 0.0);
        assert!(
            transform.x + width < time.x,
            "comparison overlaps segment timer"
        );
        assert!(transform.y <= dims[1]);
        assert!((transform.scale_x - comparison_labels[0].0.scale_x).abs() < 0.001);
        if index < comparisons {
            name_end = name_end.max(transform.x + width);
        } else {
            assert!(transform.x > name_end, "comparison name overlaps its value");
        }
    }
    for entity in manager.scene().bottom_layer() {
        if let Entity::Label(label, _, _, transform) = entity
            && transform.y < top_height
        {
            let main_timer = timer_labels[1].0;
            assert!(
                transform.x + label.width(transform.scale_x) < main_timer.x,
                "segment name overlaps main timer"
            );
            let minimum_name_scale = if comparisons > 0 {
                comparison_labels[0].0.scale_x
            } else {
                0.5 * (dims[1] - top_height)
            }
            .min(0.5 * top_height);
            assert!(transform.scale_x >= minimum_name_scale - 0.001);
            assert!(transform.scale_x <= 0.5 * top_height + 0.001);
        }
        if let Entity::Image(_, transform) = entity {
            let icon_end = transform.x + transform.scale_x;
            assert!(time.x >= icon_end);
            assert!(
                comparison_labels
                    .iter()
                    .all(|(label, _)| label.x >= icon_end)
            );
        }
    }
    time.scale_x
}

// These fixtures edit generated states to cover otherwise difficult layouts.
// A disabled comparison loses its reservation too; temporarily hidden values
// before an attempt retain it.
#[track_caller]
fn remove_comparison(state: &mut detailed_timer::State, index: usize) {
    if index == 0 {
        state.comparison1 = None;
    } else {
        state.comparison2 = None;
    }
    state.comparison_size_hints[index] = None;
}

#[track_caller]
fn set_comparison_name(state: &mut detailed_timer::State, index: usize, name: &str) {
    let comparison = if index == 0 {
        &mut state.comparison1
    } else {
        &mut state.comparison2
    };
    comparison.as_mut().unwrap().name = name.into();
    state.comparison_size_hints[index].as_mut().unwrap().name = name.into();
}

#[test]
fn detailed_timer_shares_row_with_comparisons_without_overlaps() {
    let mut image_cache = ImageCache::new();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    for expected_seconds in [47.6, 800.0, 5400.0, 500_000.0] {
        let mut run = tests_helper::create_run(&["A"]);
        let segment = run.segment_mut(0);
        segment.personal_best_split_time_mut().game_time =
            Some(TimeSpan::from_seconds(expected_seconds));
        segment.best_segment_time_mut().game_time =
            Some(TimeSpan::from_seconds(expected_seconds - 1.0));
        let mut timer = Timer::new(run).unwrap();
        tests_helper::start_run(&mut timer);
        let mut layout = Layout::new();
        layout.general_settings_mut().timer_font = Some(Font {
            family: "Fira Sans".into(),
            ..Default::default()
        });
        layout.push(Box::new(detailed_timer::Component::new()));
        for current_seconds in [37.93, 600.0, 36_000.0, 3_600_000.0] {
            timer
                .set_game_time(TimeSpan::from_seconds(current_seconds))
                .unwrap();
            for comparisons in 0..=2 {
                let mut state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
                let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
                    unreachable!();
                };
                component.timer.updates_frequently = true;
                component.segment_timer.updates_frequently = true;
                if comparisons == 2 {
                    component.segment_name = Some("A Very Long Segment Name".into());
                }
                if comparisons < 2 {
                    remove_comparison(component, 1);
                }
                if comparisons == 0 {
                    remove_comparison(component, 0);
                }
                for dims in [[80.0, 120.0], [200.0, 120.0], [1000.0, 120.0]] {
                    manager.update_scene(&mut allocator, dims, &state, &image_cache);
                    assert_detailed_timer_fits(&manager, dims, comparisons);
                }
            }
        }
    }
}

#[test]
fn detailed_timer_keeps_estimated_size_with_custom_names_and_icon() {
    let mut timer = tests_helper::create_timer(&["A"]);
    let mut layout = Layout::new();
    layout.push(Box::new(detailed_timer::Component::new()));
    let mut image_cache = ImageCache::new();
    let icon = SettingsImage::new([1].into(), SettingsImage::ICON);
    let icon_id = *image_cache.cache(icon.id(), || icon.clone()).id();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    tests_helper::start_run(&mut timer);
    let mut scale = None;
    for seconds in [1.0, 10.0, 60.0, 120.0, 600.0] {
        timer
            .set_game_time(TimeSpan::from_seconds(seconds))
            .unwrap();
        let mut state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
        let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
            unreachable!();
        };
        component.timer.updates_frequently = true;
        component.segment_timer.updates_frequently = true;
        component.icon = icon_id;
        component.display_icon = detailed_timer::IconDisplayMode::BothRows;
        component.segment_name = Some("A Very Long Segment Name".into());
        set_comparison_name(component, 0, "A Very Long Custom Comparison");
        let dims = [260.0, 120.0];
        manager.update_scene(&mut allocator, dims, &state, &image_cache);
        let current_scale = assert_detailed_timer_fits(&manager, dims, 2);
        assert_eq!(*scale.get_or_insert(current_scale), current_scale);
    }
}

#[test]
fn first_row_icon_reclaims_lower_width_and_keeps_comparisons_below_it() {
    use detailed_timer::IconDisplayMode;

    let mut timer = detailed_timer_with_times(&["Start", "Final Boss Battle"]);
    timer.set_game_time(TimeSpan::from_seconds(123.45)).unwrap();
    let mut layout = detailed_layout();
    let mut images = ImageCache::new();
    let icon = SettingsImage::new([1].into(), SettingsImage::ICON);
    let icon_id = *images.cache(icon.id(), || icon.clone()).id();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);

    for dims in [
        [280.0, 120.0],
        [420.0, 120.0],
        [280.0, 260.0],
        [560.0, 260.0],
    ] {
        for count in 0..=2 {
            let mut state = layout.state(&mut images, &timer.snapshot(), Lang::English);
            let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
                unreachable!()
            };
            component.timer.updates_frequently = true;
            component.segment_timer.updates_frequently = true;
            component.icon = icon_id;
            if count < 2 {
                remove_comparison(component, 1);
            }
            if count == 0 {
                remove_comparison(component, 0);
            }
            component.display_icon = IconDisplayMode::Hidden;
            manager.update_scene(&mut allocator, dims, &state, &images);
            assert!(
                !manager
                    .scene()
                    .bottom_layer()
                    .iter()
                    .any(|e| matches!(e, Entity::Image(..)))
            );
            let hidden_scale = placed_labels(&manager, true)[3].transform.scale_y;

            let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
                unreachable!()
            };
            component.display_icon = IconDisplayMode::FirstRow;
            manager.update_scene(&mut allocator, dims, &state, &images);
            let timers = placed_labels(&manager, true);
            assert_eq!(
                timers[3].transform.scale_y, hidden_scale,
                "row one must not consume the segment timer's width"
            );
            let icon = manager
                .scene()
                .bottom_layer()
                .iter()
                .find_map(|e| match e {
                    Entity::Image(_, transform) => Some(*transform),
                    _ => None,
                })
                .unwrap();
            assert!(icon.y >= 0.0);
            assert!(icon.y + icon.scale_y <= dims[1] * 40.0 / 65.0 + 0.001);
            let text = placed_labels(&manager, false);
            for line in &text[text.len() - 2 * count..] {
                assert!(
                    line.bounds[0] >= icon.y + icon.scale_y - 0.001,
                    "comparison enters the icon's first-row space"
                );
            }
            for label in timers.iter().chain(&text) {
                let overlaps_y =
                    label.bounds[0] < icon.y + icon.scale_y && icon.y < label.bounds[1];
                let overlaps_x = label.transform.x < icon.x + icon.scale_x
                    && icon.x < label.transform.x + label.width;
                assert!(
                    !overlaps_x || !overlaps_y,
                    "text overlaps the first-row icon"
                );
            }
        }
    }
}

#[track_caller]
fn icon_transform(manager: &Manager) -> Transform {
    icon_transform_if_present(manager).expect("an enabled icon must remain visible")
}

#[track_caller]
fn assert_icon_does_not_overlap_text(manager: &Manager, icon: Transform) {
    for label in placed_labels(manager, true)
        .into_iter()
        .chain(placed_labels(manager, false))
    {
        let overlaps_y = label.bounds[0] < icon.y + icon.scale_y && icon.y < label.bounds[1];
        let overlaps_x =
            label.transform.x < icon.x + icon.scale_x && icon.x < label.transform.x + label.width;
        assert!(!overlaps_x || !overlaps_y, "text overlaps the icon");
    }
}

#[test]
fn first_row_icon_aligns_with_fixed_timer_center_and_protects_comparison_padding() {
    let mut timer = detailed_timer_with_times(&["Start", "Underground Laboratory Escape"]);
    timer.set_game_time(TimeSpan::from_seconds(123.45)).unwrap();
    let mut layout = detailed_layout();
    let mut images = ImageCache::new();
    let image = SettingsImage::new([1].into(), SettingsImage::ICON);
    let id = *images.cache(image.id(), || image.clone()).id();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    for custom_font in [false, true] {
        let font = custom_font.then(|| Font {
            family: "Fira Sans".into(),
            ..Default::default()
        });
        let reference = timer_reference(&mut allocator, font.as_ref());
        layout.general_settings_mut().timer_font = font;
        for share in [35, 62, 85] {
            for count in 0..=2 {
                let mut state = layout.state(&mut images, &timer.snapshot(), Lang::English);
                let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
                    unreachable!()
                };
                component.icon = id;
                component.display_icon = detailed_timer::IconDisplayMode::FirstRow;
                component.timer.height = share;
                component.segment_timer.height = 100 - share;
                component.timer.updates_frequently = true;
                component.segment_timer.updates_frequently = true;
                if count < 2 {
                    remove_comparison(component, 1);
                }
                if count == 0 {
                    remove_comparison(component, 0);
                }
                for dims in [
                    [160.0, 70.0],
                    [280.0, 120.0],
                    [200.0, 260.0],
                    [560.0, 120.0],
                    [1000.0, 260.0],
                ] {
                    manager.update_scene(&mut allocator, dims, &state, &images);
                    let icon = icon_transform(&manager);
                    let main = placed_labels(&manager, true)[1].transform;
                    let timer_center = main.y + 0.5 * (reference[0] + reference[1]) * main.scale_y;
                    assert!((icon.y + 0.5 * icon.scale_y - timer_center).abs() < 0.001);
                    assert!(icon.scale_x > 0.0 && icon.scale_y > 0.0);
                    let padding =
                        (0.1 * dims[1] * share as f32 / 100.0).min(0.35 * 24.0 * dims[1] / 100.0);
                    assert!(icon.y >= padding - 0.001, "icon needs padding above it");
                    assert!(icon.y + icon.scale_y <= dims[1] - padding + 0.001);
                    let text = placed_labels(&manager, false);
                    for line in &text[text.len() - 2 * count..] {
                        assert!(
                            line.bounds[0] >= icon.y + icon.scale_y + padding - 0.001,
                            "first-row icon touches a comparison"
                        );
                    }
                    assert_icon_does_not_overlap_text(&manager, icon);
                }
            }
        }
    }
}

#[test]
fn icons_shrink_with_width_and_remain_visible_when_text_consumes_the_row() {
    let mut timer = detailed_timer_with_times(&["Start", "Underground Laboratory Escape"]);
    timer
        .set_game_time(TimeSpan::from_seconds(1124.85))
        .unwrap();
    let mut layout = detailed_layout();
    let mut images = ImageCache::new();
    let image = SettingsImage::new([1].into(), SettingsImage::ICON);
    let id = *images.cache(image.id(), || image.clone()).id();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    for mode in [
        detailed_timer::IconDisplayMode::FirstRow,
        detailed_timer::IconDisplayMode::BothRows,
    ] {
        let mut state = layout.state(&mut images, &timer.snapshot(), Lang::English);
        let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
            unreachable!()
        };
        component.icon = id;
        component.display_icon = mode;
        component.timer.updates_frequently = true;
        component.segment_timer.updates_frequently = true;
        let mut sizes = Vec::new();
        for width in [1000.0, 280.0, 160.0] {
            let dims = [width, 160.0];
            manager.update_scene(&mut allocator, dims, &state, &images);
            let icon = icon_transform(&manager);
            sizes.push(icon.scale_x);
            assert!(icon.scale_x > 0.0);
            let main = placed_labels(&manager, true)[1].transform;
            assert!(main.scale_y > 0.0, "the timer must retain drawable width");
            assert!(
                main.x >= icon.x + icon.scale_x,
                "the icon consumes timer space"
            );
            assert_icon_does_not_overlap_text(&manager, icon);
        }
        assert!(sizes[0] > sizes[1] && sizes[1] > sizes[2], "{sizes:?}");
    }
}

#[test]
fn icon_size_and_position_stay_stable_before_start_during_ticks_and_across_splits() {
    let mut images = ImageCache::new();
    let image = SettingsImage::new([1].into(), SettingsImage::ICON);
    let id = *images.cache(image.id(), || image.clone()).id();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    for mode in [
        detailed_timer::IconDisplayMode::FirstRow,
        detailed_timer::IconDisplayMode::BothRows,
    ] {
        for share in [35, 62, 85] {
            for dims in [[160.0, 120.0], [560.0, 120.0], [280.0, 260.0]] {
                let mut timer = timer_with_varied_comparison_times();
                let mut layout = detailed_layout();
                let mut expected_icon = None;
                let mut expected_timers = None;
                for step in 0..=13 {
                    if step == 1 {
                        tests_helper::start_run(&mut timer);
                    }
                    if (2..=7).contains(&step) {
                        let seconds = [0.11, 1.11, 8.88, 59.99, 60.11, 600.11][step - 2];
                        timer
                            .set_game_time(TimeSpan::from_seconds(seconds))
                            .unwrap();
                    } else if (8..=12).contains(&step) {
                        timer
                            .set_game_time(TimeSpan::from_seconds(600.11 + step as f64))
                            .unwrap();
                        timer.split().unwrap();
                    } else if step == 13 {
                        timer.reset(false).unwrap();
                    }
                    let mut state = layout.state(&mut images, &timer.snapshot(), Lang::English);
                    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
                        unreachable!()
                    };
                    component.icon = id;
                    component.display_icon = mode;
                    component.timer.height = share;
                    component.segment_timer.height = 100 - share;
                    component.timer.updates_frequently = true;
                    component.segment_timer.updates_frequently = true;
                    manager.update_scene(&mut allocator, dims, &state, &images);
                    let icon = icon_transform(&manager);
                    let placement = (icon.x, icon.y, icon.scale_x, icon.scale_y);
                    assert_eq!(
                        *expected_icon.get_or_insert(placement),
                        placement,
                        "starting, ticking, splitting, or resetting changed the icon at step {step}"
                    );
                    let timers: Vec<_> = placed_labels(&manager, true)
                        .iter()
                        .map(|label| (label.transform.y, label.transform.scale_y))
                        .collect();
                    assert_eq!(*expected_timers.get_or_insert(timers.clone()), timers);
                }
            }
        }
    }
}

#[test]
fn icon_remains_visible_and_timers_fit_when_expected_times_are_exceeded() {
    let mut timer = detailed_timer_with_times(&["Start"]);
    let mut layout = detailed_layout();
    let mut images = ImageCache::new();
    let image = SettingsImage::new([1].into(), SettingsImage::ICON);
    let id = *images.cache(image.id(), || image.clone()).id();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    let dims = [280.0, 120.0];
    let mut previous = f32::INFINITY;
    for seconds in [1.11, 36_000.11, 360_000.11, 3_600_000.11] {
        timer
            .set_game_time(TimeSpan::from_seconds(seconds))
            .unwrap();
        let mut state = layout.state(&mut images, &timer.snapshot(), Lang::English);
        let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
            unreachable!()
        };
        component.icon = id;
        component.display_icon = detailed_timer::IconDisplayMode::FirstRow;
        component.timer.updates_frequently = true;
        component.segment_timer.updates_frequently = true;
        manager.update_scene(&mut allocator, dims, &state, &images);
        let icon = icon_transform(&manager);
        assert!(icon.scale_y > 0.0);
        let timers = placed_labels(&manager, true);
        let main = timers[1].transform;
        assert!(main.scale_y < previous);
        previous = main.scale_y;
        for label in timers {
            assert!(label.transform.x >= 0.0);
            assert!(label.transform.x + label.width <= dims[0] + 0.001);
        }
        assert_icon_does_not_overlap_text(&manager, icon);
    }
}

#[test]
fn missing_icons_keep_timer_geometry_stable_before_start_and_across_splits() {
    let image = SettingsImage::new([1].into(), SettingsImage::ICON);
    let mut images = ImageCache::new();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    for mode in [
        detailed_timer::IconDisplayMode::FirstRow,
        detailed_timer::IconDisplayMode::BothRows,
    ] {
        let mut run = timer_with_varied_comparison_times().run().clone();
        for index in [0, 2, 4] {
            run.segment_mut(index).set_icon(image.clone());
        }
        let mut timer = Timer::new(run).unwrap();
        timer.set_current_timing_method(TimingMethod::GameTime);
        let mut layout = Layout::new();
        layout.push(Box::new(detailed_timer::Component::with_settings(
            detailed_timer::Settings {
                display_icon: mode,
                show_segment_name: true,
                ..Default::default()
            },
        )));
        let mut expected_timers = None;
        let mut expected_icon: Option<(f32, f32, f32, f32)> = None;
        let mut expected_comparison_geometry = None;
        for step in 0..=7 {
            if step == 1 {
                tests_helper::start_run(&mut timer);
            } else if (2..=6).contains(&step) {
                timer
                    .set_game_time(TimeSpan::from_seconds(step as f64))
                    .unwrap();
                timer.split().unwrap();
            } else if step == 7 {
                timer.reset(false).unwrap();
            }
            let mut state = layout.state(&mut images, &timer.snapshot(), Lang::English);
            let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
                unreachable!()
            };
            component.timer.updates_frequently = true;
            component.segment_timer.updates_frequently = true;
            manager.update_scene(&mut allocator, [280.0, 120.0], &state, &images);
            let timers: Vec<_> = placed_labels(&manager, true)
                .iter()
                .map(|label| (label.transform.y, label.transform.scale_y))
                .collect();
            assert_eq!(
                *expected_timers.get_or_insert(timers.clone()),
                timers,
                "an empty icon must not change the reserved layout at step {step}"
            );
            let has_image = manager
                .scene()
                .bottom_layer()
                .iter()
                .any(|entity| matches!(entity, Entity::Image(..)));
            assert_eq!(has_image, matches!(step, 1 | 3 | 5 | 6));
            if step > 0 && step < 7 {
                let text = placed_labels(&manager, false);
                let comparisons = &text[text.len() - 4..];
                let geometry: Vec<_> = comparisons
                    .iter()
                    .map(|label| (label.transform.y, label.transform.scale_y))
                    .collect();
                assert_eq!(
                    *expected_comparison_geometry.get_or_insert(geometry.clone()),
                    geometry,
                    "missing icons must only translate comparison text"
                );
                let padding = icon_transform_if_present(&manager)
                    .map(|icon| icon.x)
                    .unwrap_or_else(|| expected_icon.unwrap().0);
                if !has_image || mode == detailed_timer::IconDisplayMode::FirstRow {
                    assert!((comparisons[0].transform.x - padding).abs() < 0.001);
                }
            }
            if has_image {
                let icon = icon_transform(&manager);
                let placement = (icon.x, icon.y, icon.scale_x, icon.scale_y);
                assert_eq!(*expected_icon.get_or_insert(placement), placement);
            }
        }
    }
}

#[track_caller]
fn icon_transform_if_present(manager: &Manager) -> Option<Transform> {
    manager.scene().bottom_layer().iter().find_map(|entity| {
        if let Entity::Image(_, transform) = entity {
            Some(*transform)
        } else {
            None
        }
    })
}

#[track_caller]
fn assert_missing_icon_reclamation(
    manager: &mut Manager,
    allocator: &mut Allocator,
    images: &ImageCache,
    state: &mut livesplit_core::layout::LayoutState,
    dims: [f32; 2],
) -> bool {
    use livesplit_core::settings::ImageId;

    let ComponentState::DetailedTimer(component) = &state.components[0] else {
        unreachable!()
    };
    let mode = component.display_icon;
    let id = component.icon;
    let count = [&component.comparison1, &component.comparison2]
        .into_iter()
        .flatten()
        .count();
    manager.update_scene(&mut *allocator, dims, state, images);
    let icon = icon_transform(manager);
    let before_timers = placed_labels(manager, true);
    let before_text = placed_labels(manager, false);
    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
        unreachable!()
    };
    component.icon = *ImageId::EMPTY;
    manager.update_scene(&mut *allocator, dims, state, images);
    let after_timers = placed_labels(manager, true);
    let after_text = placed_labels(manager, false);
    assert!(icon_transform_if_present(manager).is_none());
    for (before, after) in before_timers.iter().zip(&after_timers) {
        assert_eq!(
            (
                before.transform.x,
                before.transform.y,
                before.transform.scale_x,
                before.transform.scale_y
            ),
            (
                after.transform.x,
                after.transform.y,
                after.transform.scale_x,
                after.transform.scale_y
            )
        );
    }
    let before_comparisons = &before_text[before_text.len() - 2 * count..];
    let after_comparisons = &after_text[after_text.len() - 2 * count..];
    let shift = if mode == detailed_timer::IconDisplayMode::BothRows {
        icon.scale_x + icon.x
    } else {
        0.0
    };
    for (before, after) in before_comparisons.iter().zip(after_comparisons) {
        assert!((before.transform.x - after.transform.x - shift).abs() < 0.001);
        assert_eq!(
            (before.transform.y, before.transform.scale_y, before.width),
            (after.transform.y, after.transform.scale_y, after.width)
        );
        assert!(after.transform.x + after.width < after_timers[3].transform.x);
        assert!(after.bounds[0] >= 0.0 && after.bounds[1] <= dims[1] + 0.001);
    }
    let mut name_grew = false;
    if after_text.len() > 2 * count {
        let after = after_text[0];
        assert!((after.transform.x - icon.x).abs() < 0.001);
        assert!(after.transform.x + after.width < after_timers[1].transform.x);
        assert!(after.bounds[0] >= -0.001 && after.bounds[1] <= dims[1] + 0.001);
        if before_text.len() > 2 * count {
            name_grew = after.transform.scale_y > before_text[0].transform.scale_y + 0.001;
        }
    }
    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
        unreachable!()
    };
    component.icon = id;
    name_grew
}

#[test]
fn missing_icons_reclaim_text_space_without_refitting_timers_or_comparisons() {
    let image = SettingsImage::new([1].into(), SettingsImage::ICON);
    let mut images = ImageCache::new();
    let id = *images.cache(image.id(), || image.clone()).id();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    let mut timer = timer_with_varied_comparison_times();
    tests_helper::start_run(&mut timer);
    let mut name_grew = false;
    for mode in [
        detailed_timer::IconDisplayMode::FirstRow,
        detailed_timer::IconDisplayMode::BothRows,
    ] {
        for custom_font in [false, true] {
            let mut layout = Layout::new();
            layout.general_settings_mut().timer_font = custom_font.then(|| Font {
                family: "Fira Sans".into(),
                ..Default::default()
            });
            layout.general_settings_mut().times_font = layout.general_settings().timer_font.clone();
            layout.push(Box::new(detailed_timer::Component::with_settings(
                detailed_timer::Settings {
                    display_icon: mode,
                    show_segment_name: true,
                    ..Default::default()
                },
            )));
            for heights in [[40, 25], [23, 42], [55, 10]] {
                for seconds in [1124.85, 360_000.12, -360_000.12] {
                    timer
                        .set_game_time(TimeSpan::from_seconds(seconds))
                        .unwrap();
                    for count in 0..=2 {
                        let mut state = layout.state(&mut images, &timer.snapshot(), Lang::English);
                        let ComponentState::DetailedTimer(component) = &mut state.components[0]
                        else {
                            unreachable!()
                        };
                        component.timer.height = heights[0];
                        component.segment_timer.height = heights[1];
                        component.timer.updates_frequently = true;
                        component.segment_timer.updates_frequently = true;
                        component.icon = id;
                        for index in count..2 {
                            remove_comparison(component, index);
                        }
                        for dims in [
                            [160.0, 120.0],
                            [360.0, 160.0],
                            [560.0, 120.0],
                            [280.0, 260.0],
                        ] {
                            for name in ["Start", "Underground Laboratory Escape"] {
                                let ComponentState::DetailedTimer(component) =
                                    &mut state.components[0]
                                else {
                                    unreachable!()
                                };
                                component.segment_name = Some(name.into());
                                name_grew |= assert_missing_icon_reclamation(
                                    &mut manager,
                                    &mut allocator,
                                    &images,
                                    &mut state,
                                    dims,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(
        name_grew,
        "reclaimed width should allow larger segment names"
    );
}

#[test]
fn detailed_timer_ellipsizes_name_without_shrinking_main_timer() {
    let mut timer = tests_helper::create_timer(&["A"]);
    tests_helper::start_run(&mut timer);
    timer.set_game_time(TimeSpan::from_seconds(1.23)).unwrap();
    let mut layout = Layout::new();
    layout.push(Box::new(detailed_timer::Component::new()));
    let mut image_cache = ImageCache::new();
    let mut state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
        unreachable!();
    };
    component.timer.updates_frequently = true;
    component.segment_timer.updates_frequently = true;
    component.segment_name = Some("A Very Long Name for the Final Boss Battle".into());
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    let dims = [280.0, 120.0];
    manager.update_scene(&mut allocator, dims, &state, &image_cache);
    assert_detailed_timer_fits(&manager, dims, 2);
    let top_height = dims[1] * 40.0 / 65.0;
    let (name, transform) = manager
        .scene()
        .bottom_layer()
        .iter()
        .find_map(|entity| match entity {
            Entity::Label(label, _, _, transform) if transform.y < top_height => {
                Some((label, transform))
            }
            _ => None,
        })
        .expect("the short current time should leave room for an ellipsized name");
    assert!(name.width(transform.scale_x) < name.width_without_max_width(transform.scale_x));
    let timer_bounds = |manager: &Manager| {
        manager
            .scene()
            .top_layer()
            .iter()
            .filter_map(|entity| match entity {
                Entity::Label(label, _, _, transform) => Some((
                    transform.x,
                    transform.y,
                    transform.scale_x,
                    label.width(transform.scale_x),
                )),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let bounds_with_name = timer_bounds(&manager);
    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
        unreachable!();
    };
    component.segment_name = None;
    manager.update_scene(&mut allocator, dims, &state, &image_cache);
    assert_eq!(timer_bounds(&manager), bounds_with_name);
}

#[track_caller]
fn detailed_name_bounds(manager: &Manager, top_height: f32) -> (f32, f32, f32) {
    manager
        .scene()
        .bottom_layer()
        .iter()
        .find_map(|entity| match entity {
            Entity::Label(label, _, _, transform) if transform.y < top_height => Some((
                transform.scale_x,
                label.width(transform.scale_x),
                label.width_without_max_width(transform.scale_x),
            )),
            _ => None,
        })
        .expect("the segment name should be visible")
}

#[track_caller]
fn detailed_main_timer_scale(manager: &Manager) -> f32 {
    manager
        .scene()
        .top_layer()
        .iter()
        .filter_map(|entity| match entity {
            Entity::Label(_, _, _, transform) => Some(transform.scale_x),
            _ => None,
        })
        .nth(1)
        .unwrap()
}

#[test]
fn detailed_name_fits_before_ellipsizing_at_the_comparison_font_size() {
    let mut timer = tests_helper::create_timer(&["A"]);
    tests_helper::start_run(&mut timer);
    timer.set_game_time(TimeSpan::from_seconds(754.56)).unwrap();
    let mut layout = Layout::new();
    layout.push(Box::new(detailed_timer::Component::new()));
    let mut image_cache = ImageCache::new();
    let mut state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
        unreachable!();
    };
    component.timer.updates_frequently = true;
    component.segment_timer.updates_frequently = true;
    component.segment_name = Some("A".into());
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    let dims = [560.0, 120.0];
    let top_height = dims[1] * 40.0 / 65.0;
    manager.update_scene(&mut allocator, dims, &state, &image_cache);
    let (configured_name_scale, width, full_width) = detailed_name_bounds(&manager, top_height);
    assert!((configured_name_scale - 0.5 * top_height).abs() < 0.001);
    assert_eq!(width, full_width);
    let timer_scale = detailed_main_timer_scale(&manager);

    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
        unreachable!();
    };
    component.segment_name = Some("Final Boss Battle".into());
    manager.update_scene(&mut allocator, dims, &state, &image_cache);
    let (name_scale, width, full_width) = detailed_name_bounds(&manager, top_height);
    assert!(name_scale < configured_name_scale);
    assert_eq!(width, full_width, "shrink to show the full name first");
    assert_eq!(detailed_main_timer_scale(&manager), timer_scale);

    for comparison_name in ["PB", "A Very Long Custom Comparison Name"] {
        let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
            unreachable!();
        };
        component.segment_name =
            Some("Ancient Underground Laboratory Escape and Final Boss Battle".into());
        set_comparison_name(component, 0, comparison_name);
        manager.update_scene(&mut allocator, dims, &state, &image_cache);
        assert_detailed_timer_fits(&manager, dims, 2);
        let comparison_scale = detailed_comparison_scale(&manager, top_height);
        let (name_scale, width, full_width) = detailed_name_bounds(&manager, top_height);
        assert!((name_scale - comparison_scale).abs() < 0.001);
        assert!(width < full_width, "ellipsize only at the comparison size");
        assert_eq!(detailed_main_timer_scale(&manager), timer_scale);
    }

    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
        unreachable!();
    };
    remove_comparison(component, 1);
    remove_comparison(component, 0);
    manager.update_scene(&mut allocator, dims, &state, &image_cache);
    let (name_scale, _, _) = detailed_name_bounds(&manager, top_height);
    assert!((name_scale - 0.5 * (dims[1] - top_height)).abs() < 0.001);
    assert_eq!(detailed_main_timer_scale(&manager), timer_scale);
}

#[test]
fn detailed_name_stays_complete_while_fitting_fractional_widths() {
    let mut timer = tests_helper::create_timer(&["Final Boss Battle"]);
    tests_helper::start_run(&mut timer);
    timer.set_game_time(TimeSpan::from_seconds(754.56)).unwrap();
    let mut layout = Layout::new();
    layout.push(Box::new(detailed_timer::Component::with_settings(
        detailed_timer::Settings {
            show_segment_name: true,
            ..Default::default()
        },
    )));
    let mut image_cache = ImageCache::new();
    let state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    let top_height = 120.0 * 40.0 / 65.0;
    let mut fitted_names = 0;
    for step in 0..2048 {
        let dims = [360.0 + step as f32 * 0.125, 120.0];
        manager.update_scene(&mut allocator, dims, &state, &image_cache);
        assert_detailed_timer_fits(&manager, dims, 2);
        let comparison_scale = detailed_comparison_scale(&manager, top_height);
        let (name_scale, width, full_width) = detailed_name_bounds(&manager, top_height);
        if name_scale > comparison_scale + 0.001 && name_scale < 0.5 * top_height {
            fitted_names += 1;
            assert_eq!(
                width, full_width,
                "unexpected ellipsis at width {}",
                dims[0]
            );
        }
    }
    assert!(
        fitted_names > 100,
        "exercise names that fit through shrinking"
    );
}

#[track_caller]
fn detailed_comparison_scale(manager: &Manager, top_height: f32) -> f32 {
    manager
        .scene()
        .bottom_layer()
        .iter()
        .find_map(|entity| match entity {
            Entity::Label(_, _, _, transform) if transform.y > top_height => {
                Some(transform.scale_x)
            }
            _ => None,
        })
        .unwrap()
}

#[derive(Clone, Copy)]
struct PlacedText {
    transform: Transform,
    width: f32,
    full_width: f32,
    bounds: [f32; 2],
}

#[track_caller]
fn placed_labels(manager: &Manager, top: bool) -> Vec<PlacedText> {
    let entities = if top {
        manager.scene().top_layer()
    } else {
        manager.scene().bottom_layer()
    };
    entities
        .iter()
        .filter_map(|entity| match entity {
            Entity::Label(label, _, _, transform) => {
                let bounds = label.vertical_bounds(transform.scale_y);
                Some(PlacedText {
                    transform: *transform,
                    width: label.width(transform.scale_x),
                    full_width: label.width_without_max_width(transform.scale_x),
                    bounds: [transform.y + bounds[0], transform.y + bounds[1]],
                })
            }
            _ => None,
        })
        .collect()
}

#[track_caller]
fn timer_reference(allocator: &mut Allocator, font: Option<&Font>) -> [f32; 2] {
    let mut font = allocator.create_font(font, FontKind::Timer);
    let label = allocator.create_label("0123456789:−.—", &mut font, None);
    label.vertical_bounds(1.0)
}

#[track_caller]
fn detailed_layout() -> Layout {
    let mut layout = Layout::new();
    layout.push(Box::new(detailed_timer::Component::with_settings(
        detailed_timer::Settings {
            show_segment_name: true,
            ..Default::default()
        },
    )));
    layout
}

#[track_caller]
fn detailed_timer_with_times(names: &[&str]) -> Timer {
    let mut run = tests_helper::create_run(names);
    for (index, segment) in run.segments_mut().iter_mut().enumerate() {
        segment.personal_best_split_time_mut().game_time =
            Some(TimeSpan::from_seconds(3000.0 * (index + 1) as f64));
        segment.best_segment_time_mut().game_time = Some(TimeSpan::from_seconds(2700.0));
    }
    let mut timer = Timer::new(run).unwrap();
    tests_helper::start_run(&mut timer);
    timer
}

#[track_caller]
fn timer_with_varied_comparison_times() -> Timer {
    let mut run = tests_helper::create_run(&[
        "Start",
        "Underground Laboratory Escape",
        "Final Boss Battle",
        "Return",
        "Finish",
    ]);
    for ((segment, pb), best) in run
        .segments_mut()
        .iter_mut()
        .zip([Some(1.0), Some(60.0), None, Some(3800.0), Some(6800.0)])
        .zip([Some(0.5), Some(5.0), None, Some(3200.0), Some(50.0)])
    {
        segment.personal_best_split_time_mut().game_time = pb.map(TimeSpan::from_seconds);
        segment.best_segment_time_mut().game_time = best.map(TimeSpan::from_seconds);
    }
    let mut timer = Timer::new(run).unwrap();
    timer.set_current_timing_method(TimingMethod::GameTime);
    timer
}

#[test]
fn detailed_timer_reserves_comparisons_before_start_and_keeps_the_layout_across_splits() {
    let mut images = ImageCache::new();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    for hide_second in [false, true] {
        for custom_font in [false, true] {
            for dims in [[160.0, 120.0], [420.0, 120.0], [280.0, 260.0]] {
                let mut timer = timer_with_varied_comparison_times();
                let mut layout = Layout::new();
                layout.general_settings_mut().times_font = custom_font.then(|| Font {
                    family: "Fira Sans".into(),
                    ..Default::default()
                });
                layout.push(Box::new(detailed_timer::Component::with_settings(
                    detailed_timer::Settings {
                        show_segment_name: true,
                        hide_second_comparison: hide_second,
                        ..Default::default()
                    },
                )));
                let count = if hide_second { 1 } else { 2 };
                let mut expected_timers = None;
                let mut expected_comparisons = None;
                let mut expected_value_right = None;
                for step in 0..=7 {
                    if step == 1 {
                        tests_helper::start_run(&mut timer);
                    } else if (2..=6).contains(&step) {
                        timer
                            .set_game_time(TimeSpan::from_seconds(step as f64))
                            .unwrap();
                        timer.split().unwrap();
                    } else if step == 7 {
                        timer.reset(false).unwrap();
                    }
                    let mut state = layout.state(&mut images, &timer.snapshot(), Lang::English);
                    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
                        unreachable!()
                    };
                    // Keep all four timer labels in the same layer, including
                    // before starting, to inspect their sizes and baselines.
                    component.timer.updates_frequently = true;
                    component.segment_timer.updates_frequently = true;
                    assert_eq!(
                        component.comparison_size_hints.iter().flatten().count(),
                        count
                    );
                    let visible = step != 0 && step != 7;
                    assert_eq!(component.comparison1.is_some(), visible);
                    manager.update_scene(&mut allocator, dims, &state, &images);
                    let timers = placed_labels(&manager, true);
                    assert_eq!(timers.len(), 4);
                    let positions: Vec<_> = timers
                        .iter()
                        .map(|label| (label.transform.y, label.transform.scale_y))
                        .collect();
                    assert_eq!(
                        *expected_timers.get_or_insert(positions.clone()),
                        positions,
                        "start, split, or reset changed a timer at step {step}"
                    );
                    let text = placed_labels(&manager, false);
                    if visible {
                        assert_detailed_timer_fits(&manager, dims, count);
                        let comparisons = &text[text.len() - 2 * count..];
                        let positions: Vec<_> = comparisons[..count]
                            .iter()
                            .map(|label| {
                                let t = label.transform;
                                (t.x, t.y, t.scale_x, t.scale_y)
                            })
                            .collect();
                        assert_eq!(
                            *expected_comparisons.get_or_insert(positions.clone()),
                            positions,
                            "split changed comparison positions or sizes at step {step}"
                        );
                        let right = comparisons[count].transform.x + comparisons[count].width;
                        assert!(
                            (right - *expected_value_right.get_or_insert(right)).abs() < 0.001,
                            "split moved the reserved value column at step {step}"
                        );
                        for value in &comparisons[count..] {
                            assert!((value.transform.x + value.width - right).abs() < 0.001);
                        }
                    } else {
                        assert!(
                            text.is_empty(),
                            "comparisons stay hidden outside an attempt"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn hidden_comparison_hints_invalidate_the_cached_layout() {
    let timer = timer_with_varied_comparison_times();
    let mut layout = detailed_layout();
    let mut images = ImageCache::new();
    let mut state = layout.state(&mut images, &timer.snapshot(), Lang::English);
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    let dims = [160.0, 120.0];
    manager.update_scene(&mut allocator, dims, &state, &images);
    let timers = placed_labels(&manager, false);
    assert_eq!(timers.len(), 4);
    let before = timers[3].transform.scale_y;
    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
        unreachable!()
    };
    assert!(component.comparison1.is_none() && component.comparison2.is_none());
    component.comparison_size_hints[0].as_mut().unwrap().time = "888:88:88.88".into();
    manager.update_scene(&mut allocator, dims, &state, &images);
    let after = placed_labels(&manager, false);
    assert_eq!(after.len(), 4);
    assert!(after[3].transform.scale_y < before);
}

#[test]
fn detailed_timer_centers_comparison_gap_and_protects_all_text() {
    let mut timer = detailed_timer_with_times(&["Start", "Final Boss Battle"]);
    let mut layout = detailed_layout();
    let mut image_cache = ImageCache::new();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    for custom_font in [false, true] {
        layout.general_settings_mut().timer_font = custom_font.then(|| Font {
            family: "Fira Sans".into(),
            ..Default::default()
        });
        let reference = timer_reference(
            &mut allocator,
            layout.general_settings().timer_font.as_ref(),
        );
        for seconds in [1124.85, 360_000.12, -360_000.12] {
            timer
                .set_game_time(TimeSpan::from_seconds(seconds))
                .unwrap();
            for (first_label, last_label) in [("PB", "Best"), ("Áverage (gyp)", "gyp"), ("", "")] {
                let mut state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
                let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
                    unreachable!()
                };
                set_comparison_name(component, 0, first_label);
                set_comparison_name(component, 1, last_label);
                for dims in [
                    [280.0, 120.0],
                    [420.0, 120.0],
                    [560.0, 120.0],
                    [280.0, 260.0],
                    [1000.0, 260.0],
                ] {
                    manager.update_scene(&mut allocator, dims, &state, &image_cache);
                    assert_detailed_timer_fits(&manager, dims, 2);
                    let timers = placed_labels(&manager, true);
                    let text = placed_labels(&manager, false);
                    let comparisons = &text[text.len() - 4..];
                    let name_bottom = text.first().unwrap().bounds[1];
                    for label in &text {
                        assert!(
                            label.bounds[0] >= -0.001,
                            "ascender above component: {:?}",
                            label.bounds
                        );
                        assert!(
                            label.bounds[1] < dims[1],
                            "descender outside component: {:?}",
                            label.bounds
                        );
                    }
                    let first_bottom = comparisons[0].bounds[1].max(comparisons[2].bounds[1]);
                    let second_top = comparisons[1].bounds[0].min(comparisons[3].bounds[0]);
                    let first_top = comparisons[0].bounds[0].min(comparisons[2].bounds[0]);
                    assert!(first_bottom < second_top, "comparison lines overlap");
                    assert!(first_top > name_bottom, "comparison enters the first row");
                    let center = timers[3].transform.y
                        + 0.5 * (reference[0] + reference[1]) * timers[3].transform.scale_y;
                    // The reference is fixed across digits. Optical overshoot
                    // of the current digits can differ by a fraction of a pixel.
                    assert!(
                        (0.5 * (first_bottom + second_top) - center).abs() < 1.0,
                        "gap is off center"
                    );
                    for comparison in comparisons {
                        assert_eq!(
                            comparison.width, comparison.full_width,
                            "comparisons must never ellipsize"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn detailed_timer_baselines_and_sizes_stay_fixed_while_digits_change() {
    let mut timer = detailed_timer_with_times(&["Start", "Ancient Underground Laboratory Escape"]);
    let mut layout = detailed_layout();
    let mut image_cache = ImageCache::new();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    for custom_font in [false, true] {
        layout.general_settings_mut().timer_font = custom_font.then(|| Font {
            family: "Fira Sans".into(),
            ..Default::default()
        });
        for dims in [[280.0, 120.0], [420.0, 120.0], [280.0, 260.0]] {
            let mut expected = None;
            for seconds in [
                0.0, 1.11, 2.22, 3.33, 4.44, 5.55, 6.66, 7.77, 8.88, 9.99, 10.0, 59.99, 60.0,
                99.99, 600.0, 1124.85, 3599.99,
            ] {
                timer
                    .set_game_time(TimeSpan::from_seconds(seconds))
                    .unwrap();
                let mut state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
                let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
                    unreachable!()
                };
                // Also cover numeric glyph changes in comparison values.
                component.comparison1.as_mut().unwrap().time =
                    format!("5:{:02}.00", (seconds as u32) % 60);
                component.comparison2.as_mut().unwrap().time =
                    format!("4:{:02}.00", (seconds as u32) % 60);
                manager.update_scene(&mut allocator, dims, &state, &image_cache);
                let timers = placed_labels(&manager, true);
                let text = placed_labels(&manager, false);
                let positions: Vec<_> = timers
                    .iter()
                    .chain(&text[text.len() - 4..])
                    .map(|label| (label.transform.y, label.transform.scale_y))
                    .collect();
                assert_eq!(
                    *expected.get_or_insert(positions.clone()),
                    positions,
                    "digits moved the layout at {seconds}s"
                );
            }
        }
    }
}

#[test]
fn detailed_timer_uses_three_equal_gaps_around_fixed_timer_boxes() {
    let mut timer = detailed_timer_with_times(&["Start"]);
    timer
        .set_game_time(TimeSpan::from_seconds(1124.85))
        .unwrap();
    let mut layout = detailed_layout();
    let mut image_cache = ImageCache::new();
    let mut state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
        unreachable!()
    };
    remove_comparison(component, 1);
    remove_comparison(component, 0);
    let mut allocator = Allocator(TextEngine::new(), 0);
    let reference = timer_reference(&mut allocator, None);
    let mut manager = SceneManager::new(&mut allocator);
    for dims in [
        [280.0, 120.0],
        [560.0, 120.0],
        [280.0, 260.0],
        [1000.0, 260.0],
    ] {
        manager.update_scene(&mut allocator, dims, &state, &image_cache);
        let timers = placed_labels(&manager, true);
        let bound = |label: &PlacedText, index: usize| {
            label.transform.y + reference[index] * label.transform.scale_y
        };
        let gaps = [
            bound(&timers[1], 0),
            bound(&timers[3], 0) - bound(&timers[1], 1),
            dims[1] - bound(&timers[3], 1),
        ];
        assert!((gaps[0] - gaps[1]).abs() < 0.001, "{gaps:?}");
        assert!((gaps[1] - gaps[2]).abs() < 0.001, "{gaps:?}");
    }
}

#[test]
fn detailed_timer_centers_a_single_comparison_for_either_slot() {
    let mut timer = detailed_timer_with_times(&["Start"]);
    timer.set_game_time(TimeSpan::from_seconds(123.45)).unwrap();
    let mut layout = detailed_layout();
    let mut image_cache = ImageCache::new();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let reference = timer_reference(&mut allocator, None);
    let mut manager = SceneManager::new(&mut allocator);
    for second_slot in [false, true] {
        let mut state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
        let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
            unreachable!()
        };
        if second_slot {
            remove_comparison(component, 0);
        } else {
            remove_comparison(component, 1);
        }
        for dims in [[280.0, 120.0], [420.0, 260.0]] {
            manager.update_scene(&mut allocator, dims, &state, &image_cache);
            assert_detailed_timer_fits(&manager, dims, 1);
            let timers = placed_labels(&manager, true);
            let text = placed_labels(&manager, false);
            let comparisons = &text[text.len() - 2..];
            let top = comparisons[0].bounds[0].min(comparisons[1].bounds[0]);
            let bottom = comparisons[0].bounds[1].max(comparisons[1].bounds[1]);
            let center = timers[3].transform.y
                + 0.5 * (reference[0] + reference[1]) * timers[3].transform.scale_y;
            assert!((0.5 * (top + bottom) - center).abs() < 1.0);
        }
    }
}

#[test]
fn detailed_main_timer_keeps_its_size_across_splits_with_different_names() {
    let mut run = tests_helper::create_run(&[
        "A",
        "Final Boss Battle",
        "Ancient Underground Laboratory Escape and Final Boss Battle",
    ]);
    for (segment, seconds) in run.segments_mut().iter_mut().zip([300.0, 1200.0, 3600.0]) {
        segment.personal_best_split_time_mut().game_time = Some(TimeSpan::from_seconds(seconds));
    }
    let mut timer = Timer::new(run).unwrap();
    tests_helper::start_run(&mut timer);
    let mut layout = Layout::new();
    layout.push(Box::new(detailed_timer::Component::with_settings(
        detailed_timer::Settings {
            show_segment_name: true,
            ..Default::default()
        },
    )));
    let mut image_cache = ImageCache::new();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    let dims = [280.0, 120.0];
    let mut state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
    manager.update_scene(&mut allocator, dims, &state, &image_cache);
    let reserved_scale = detailed_main_timer_scale(&manager);
    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
        unreachable!();
    };
    assert_eq!(component.segment_name_size_hints.len(), 3);
    component.segment_name_size_hints.clear();
    manager.update_scene(&mut allocator, dims, &state, &image_cache);
    let normal_scale = detailed_main_timer_scale(&manager);
    assert!((reserved_scale - 0.75 * normal_scale).abs() < 0.001);
    let mut expected_scale = None;
    for (index, seconds) in [1.23, 600.12, 1800.23].into_iter().enumerate() {
        timer
            .set_game_time(TimeSpan::from_seconds(seconds))
            .unwrap();
        let state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
        manager.update_scene(&mut allocator, dims, &state, &image_cache);
        let scale = detailed_main_timer_scale(&manager);
        assert_eq!(*expected_scale.get_or_insert(scale), scale);
        if index < 2 {
            timer.split().unwrap();
            let state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
            manager.update_scene(&mut allocator, dims, &state, &image_cache);
            assert_eq!(detailed_main_timer_scale(&manager), scale);
        }
    }
    timer
        .set_game_time(TimeSpan::from_seconds(36_000.0))
        .unwrap();
    let state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
    manager.update_scene(&mut allocator, dims, &state, &image_cache);
    assert!(detailed_main_timer_scale(&manager) < expected_scale.unwrap());
}

#[test]
fn detailed_timer_sizes_for_the_widest_name_rather_than_the_most_characters() {
    let mut timer = tests_helper::create_timer(&[
        "A",
        "WWWWWWWWWWWWWWWWWWWW",
        "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
    ]);
    tests_helper::start_run(&mut timer);
    let mut layout = Layout::new();
    layout.push(Box::new(detailed_timer::Component::with_settings(
        detailed_timer::Settings {
            show_segment_name: true,
            ..Default::default()
        },
    )));
    let mut image_cache = ImageCache::new();
    let mut state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    let dims = [800.0, 120.0];
    manager.update_scene(&mut allocator, dims, &state, &image_cache);
    let wide_name_scale = detailed_main_timer_scale(&manager);

    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
        unreachable!();
    };
    // Change only a future name. The longer name made of narrow glyphs remains.
    component.segment_name_size_hints[1].name = "A".into();
    manager.update_scene(&mut allocator, dims, &state, &image_cache);
    let narrow_name_scale = detailed_main_timer_scale(&manager);
    assert!(wide_name_scale < narrow_name_scale);
    assert!((narrow_name_scale - dims[1] * 40.0 / 65.0).abs() < 0.001);
    assert!(wide_name_scale > 0.75 * narrow_name_scale);
}

#[test]
fn detailed_timer_reserves_future_names_even_when_current_name_is_empty() {
    let mut timer = detailed_timer_with_times(&[
        "Start",
        "Ancient Underground Laboratory Escape and Final Boss Battle",
        "",
    ]);
    timer.set_game_time(TimeSpan::from_seconds(123.45)).unwrap();
    let mut layout = detailed_layout();
    let mut image_cache = ImageCache::new();
    let mut state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    for dims in [
        [160.0, 120.0],
        [280.0, 120.0],
        [420.0, 260.0],
        [1000.0, 120.0],
    ] {
        let mut expected = None;
        for name in [
            Some("Start"),
            Some("Ancient Underground Laboratory Escape and Final Boss Battle"),
            Some(""),
            None,
        ] {
            let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
                unreachable!()
            };
            component.segment_name = name.map(Into::into);
            manager.update_scene(&mut allocator, dims, &state, &image_cache);
            let timers = placed_labels(&manager, true);
            let positions: Vec<_> = timers
                .iter()
                .map(|label| (label.transform.y, label.transform.scale_y))
                .collect();
            assert_eq!(*expected.get_or_insert(positions.clone()), positions);
        }
    }
}

#[test]
fn detailed_timer_preserves_bounds_with_configured_heights_and_accuracies() {
    let mut timer = detailed_timer_with_times(&["Áverage (gyp)", "Start"]);
    let mut image_cache = ImageCache::new();
    let mut allocator = Allocator(TextEngine::new(), 0);
    let mut manager = SceneManager::new(&mut allocator);
    for (top, bottom) in [(10, 55), (40, 25), (55, 10)] {
        for accuracy in [
            Accuracy::Seconds,
            Accuracy::Tenths,
            Accuracy::Hundredths,
            Accuracy::Milliseconds,
        ] {
            let mut settings = detailed_timer::Settings {
                show_segment_name: true,
                ..Default::default()
            };
            settings.timer.height = top;
            settings.segment_timer.height = bottom;
            settings.timer.accuracy = accuracy;
            settings.segment_timer.accuracy = accuracy;
            settings.comparison_times_accuracy = accuracy;
            let mut layout = Layout::new();
            layout.push(Box::new(detailed_timer::Component::with_settings(settings)));
            for seconds in [1.11, 1124.85, -360_000.12, 3_600_000.99] {
                timer
                    .set_game_time(TimeSpan::from_seconds(seconds))
                    .unwrap();
                for count in 0..=2 {
                    let mut state =
                        layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
                    let ComponentState::DetailedTimer(component) = &mut state.components[0] else {
                        unreachable!()
                    };
                    if count < 2 {
                        remove_comparison(component, 1);
                    }
                    if count == 0 {
                        remove_comparison(component, 0);
                    }
                    for dims in [
                        [160.0, 90.0],
                        [420.0, 90.0],
                        [280.0, 260.0],
                        [1000.0, 260.0],
                    ] {
                        manager.update_scene(&mut allocator, dims, &state, &image_cache);
                        let timers = placed_labels(&manager, true);
                        let text = placed_labels(&manager, false);
                        for label in timers.iter().chain(&text) {
                            assert!(label.transform.scale_y.is_finite());
                            assert!(
                                label.bounds[0] >= -0.001,
                                "text above component: {:?}",
                                label.bounds
                            );
                            assert!(
                                label.bounds[1] <= dims[1] + 0.001,
                                "text below component: {:?}",
                                label.bounds
                            );
                            assert!(label.transform.x >= -0.001);
                            assert!(label.transform.x + label.width <= dims[0] + 0.001);
                        }
                        if text.len() > count * 2 && count > 0 {
                            let name = text[0];
                            let comparisons = &text[text.len() - count * 2..];
                            for comparison in comparisons {
                                assert!(
                                    comparison.bounds[0] > name.bounds[1],
                                    "comparisons enter first row"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn text_bounds_follow_outlines_and_preserve_the_full_untruncated_text() {
    let mut allocator = Allocator(TextEngine::new(), 0);
    for kind in [FontKind::Text, FontKind::Times, FontKind::Timer] {
        let mut font = allocator.create_font(None, kind);
        let empty = allocator.create_label("", &mut font, None);
        assert_eq!(empty.vertical_bounds(12.0), [0.0; 2]);
        allocator.update_label(&mut empty.share(), "   ", &mut font, None);
        assert_eq!(empty.vertical_bounds(12.0), [0.0; 2]);
        let dash = allocator.create_label("—", &mut font, None);
        let bounds = dash.vertical_bounds(1.0);
        assert!(
            bounds[0] < bounds[1] && bounds[1] < 0.0,
            "do not force baseline into dash bounds"
        );
        let label = allocator.create_label("Áverage (gyp)", &mut font, None);
        let original = label.vertical_bounds(1.0);
        assert!(original[0] < 0.0 && original[1] > 0.0);
        assert_eq!(label.vertical_bounds(12.0), original.map(|v| v * 12.0));
        allocator.update_label(&mut label.share(), "Áverage (gyp)", &mut font, Some(1.0));
        assert!(label.width(1.0) < label.width_without_max_width(1.0));
        assert_eq!(
            label.vertical_bounds(1.0),
            original,
            "ellipsis must not change full bounds"
        );
    }
}

#[cfg(feature = "software-rendering")]
#[test]
fn comparison_pixels_remain_identical_while_the_timer_ticks() {
    use livesplit_core::rendering::software::Renderer;
    let mut timer = detailed_timer_with_times(&["Start"]);
    let mut layout = detailed_layout();
    let mut images = ImageCache::new();
    let mut renderer = Renderer::new();
    let mut expected = None;
    for seconds in [1.11, 2.22, 8.88, 9.99, 10.0, 59.99, 60.0, 1124.85] {
        timer
            .set_game_time(TimeSpan::from_seconds(seconds))
            .unwrap();
        let state = layout.state(&mut images, &timer.snapshot(), Lang::English);
        renderer.render(&state, &images, [420, 120]);
        // In this fixed fixture, the comparison columns occupy the lower-left
        // 180 × 60 pixels. Both changing timers are outside this rectangle.
        let pixels: Vec<_> = (60..120)
            .flat_map(|y| {
                renderer.image_data()[4 * (y * 420)..4 * (y * 420 + 180)]
                    .iter()
                    .copied()
            })
            .collect();
        assert!(
            pixels
                .as_chunks::<4>()
                .0
                .iter()
                .any(|p| p[0] > 100 && p[1] > 100 && p[2] > 100),
            "comparison crop must contain text"
        );
        assert_eq!(
            *expected.get_or_insert(pixels.clone()),
            pixels,
            "comparison pixels changed at {seconds}s"
        );
    }
}
