cfg_if::cfg_if! {
    if #[cfg(feature = "rendering")] {
        use criterion::{criterion_group, criterion_main, Criterion};
        use livesplit_core::{
            layout::{self, Layout},
            rendering::{
                PathBuilder, ResourceAllocator, SceneManager, Label, FontKind, SharedOwnership,
            },
            run::parser::livesplit,
            settings::Font,
            Run, Segment, TimeSpan, Timer, TimingMethod,
        };
        use std::fs;

        criterion_main!(benches);
        criterion_group!(benches, default, subsplits_layout);

        struct Dummy;

        impl PathBuilder for Dummy {
            type Path = ();

            fn move_to(&mut self, _: f32, _: f32) {}
            fn line_to(&mut self, _: f32, _: f32) {}
            fn quad_to(&mut self, _: f32, _: f32, _: f32, _: f32) {}
            fn curve_to(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: f32) {}
            fn close(&mut self) {}
            fn finish(self) -> Self::Path {}
        }

        impl ResourceAllocator for Dummy {
            type PathBuilder = Dummy;
            type Path = ();
            type Image = ();
            type Font = ();
            type Label = Dummy;

            fn path_builder(&mut self) -> Self::PathBuilder {
                Dummy
            }
            fn create_image(&mut self, _: &[u8]) -> Option<(Self::Image, f32)> {
                Some(((), 1.0))
            }
            fn create_font(&mut self, _: Option<&Font>, _: FontKind) -> Self::Font {}
            fn create_label(
                &mut self,
                _: &str,
                _: &mut Self::Font,
                _: Option<f32>,
            ) -> Self::Label {
                Dummy
            }
            fn update_label(
                &mut self,
                _: &mut Self::Label,
                _: &str,
                _: &mut Self::Font,
                _: Option<f32>,
            ) {}
        }

        impl Label for Dummy {
            fn width(&self, _: f32) -> f32 {
                0.0
            }
            fn width_without_max_width(&self, _: f32) -> f32 {
                0.0
            }
        }

        impl SharedOwnership for Dummy {
            fn share(&self) -> Self {
                Dummy
            }
        }

        fn default(c: &mut Criterion) {
            let mut run = create_run(&["A", "B", "C", "D"]);
            run.set_game_name("Some Game Name");
            run.set_category_name("Some Category Name");
            run.set_attempt_count(1337);
            let mut timer = Timer::new(run).unwrap();
            let mut layout = Layout::default_layout();

            start_run(&mut timer);
            make_progress_run_with_splits_opt(&mut timer, &[Some(5.0), None, Some(10.0)]);

            let state = layout.state(&timer.snapshot());

            let mut manager = SceneManager::new(Dummy);

            c.bench_function("Scene Management (Default)", move |b| {
                b.iter(|| manager.update_scene(Dummy, (300.0, 500.0), &state))
            });
        }

        fn subsplits_layout(c: &mut Criterion) {
            let run = lss("tests/run_files/Celeste - Any% (1.2.1.5).lss");
            let mut timer = Timer::new(run).unwrap();
            let mut layout = lsl("tests/layout_files/subsplits.lsl");

            start_run(&mut timer);
            make_progress_run_with_splits_opt(
                &mut timer,
                &[Some(10.0), None, Some(20.0), Some(55.0)],
            );

            let snapshot = timer.snapshot();
            let mut state = layout.state(&snapshot);
            layout.update_state(&mut state, &snapshot);

            let mut manager = SceneManager::new(Dummy);

            c.bench_function("Scene Management (Subsplits Layout)", move |b| {
                b.iter(|| manager.update_scene(Dummy, (300.0, 800.0), &state))
            });
        }

        fn file(path: &str) -> String {
            fs::read_to_string(path).unwrap()
        }

        fn lss(path: &str) -> Run {
            livesplit::parse(&file(path)).unwrap()
        }

        fn lsl(path: &str) -> Layout {
            layout::parser::parse(&file(path)).unwrap()
        }

        fn create_run(names: &[&str]) -> Run {
            let mut run = Run::new();
            for &name in names {
                run.push_segment(Segment::new(name));
            }
            run
        }

        fn start_run(timer: &mut Timer) {
            timer.set_current_timing_method(TimingMethod::GameTime);
            timer.start();
            timer.initialize_game_time();
            timer.pause_game_time();
            timer.set_game_time(TimeSpan::zero());
        }

        fn make_progress_run_with_splits_opt(timer: &mut Timer, splits: &[Option<f64>]) {
            for &split in splits {
                if let Some(split) = split {
                    timer.set_game_time(TimeSpan::from_seconds(split));
                    timer.split();
                } else {
                    timer.skip_split();
                }
            }
        }
    } else {
        fn main() {}
    }
}
