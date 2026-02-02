cfg_if::cfg_if! {
    if #[cfg(feature = "svg-rendering")] {
        use {
            criterion::{criterion_group, criterion_main, Criterion},
            livesplit_core::{
                layout::{self, Layout},
                rendering::svg::Renderer,
                run::parser::livesplit,
                settings::ImageCache,
                Lang, Run, Segment, TimeSpan, Timer, TimingMethod,
            },
            std::fs,
        };

        criterion_main!(benches);
        criterion_group!(benches, default, subsplits_layout);

        fn default(c: &mut Criterion) {
            let mut run = create_run(&["A", "B", "C", "D"]);
            run.set_game_name("Some Game Name");
            run.set_category_name("Some Category Name");
            run.set_attempt_count(1337);
            let mut timer = Timer::new(run).unwrap();
            let mut layout = Layout::default_layout();
            let mut image_cache = ImageCache::new();

            start_run(&mut timer);
            make_progress_run_with_splits_opt(&mut timer, &[Some(5.0), None, Some(10.0)]);

            let state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
            let mut renderer = Renderer::new();
            let mut buf = String::new();

            c.bench_function("SVG Rendering (Default)", move |b| {
                b.iter(|| {
                    buf.clear();
                    renderer.render(&mut buf, &state, &image_cache, [300.0, 500.0]).unwrap();
                })
            });
        }

        fn subsplits_layout(c: &mut Criterion) {
            let run = lss("tests/run_files/Celeste - Any% (1.2.1.5).lss");
            let mut timer = Timer::new(run).unwrap();
            let mut layout = lsl("tests/layout_files/subsplits.lsl");
            let mut image_cache = ImageCache::new();

            start_run(&mut timer);
            make_progress_run_with_splits_opt(&mut timer, &[Some(10.0), None, Some(20.0), Some(55.0)]);

            let state = layout.state(&mut image_cache, &timer.snapshot(), Lang::English);
            let mut renderer = Renderer::new();
            let mut buf = String::new();

            c.bench_function("SVG Rendering (Subsplits Layout)", move |b| {
                b.iter(|| {
                    buf.clear();
                    renderer.render(&mut buf, &state, &image_cache, [300.0, 800.0]).unwrap();
                })
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
            timer.start().unwrap();
            timer.initialize_game_time().unwrap();
            timer.pause_game_time().unwrap();
            timer.set_game_time(TimeSpan::zero()).unwrap();
        }

        fn make_progress_run_with_splits_opt(timer: &mut Timer, splits: &[Option<f64>]) {
            for &split in splits {
                if let Some(split) = split {
                    timer.set_game_time(TimeSpan::from_seconds(split)).unwrap();
                    timer.split().unwrap();
                } else {
                    timer.skip_split().unwrap();
                }
            }
        }
    } else {
        fn main() {}
    }
}
