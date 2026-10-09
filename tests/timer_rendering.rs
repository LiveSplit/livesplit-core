#![cfg(feature = "default-text-engine")]

#[path = "../src/util/tests_helper.rs"]
mod tests_helper;

use livesplit_core::{
    Lang, Run, Segment, TimeSpan, Timer, TimingMethod,
    component::timer,
    layout::{ComponentState, Layout},
    rendering::{
        Entity, FontKind, Image, Label, PathBuilder, ResourceAllocator, SceneManager,
        SharedOwnership, Transform,
        default_text_engine::{self, TextEngine},
    },
    settings::{Font, ImageCache},
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
    fn create_image(&mut self, _: &[u8]) -> Option<EmptyPath> {
        None
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
