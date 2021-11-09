cfg_if::cfg_if! {
    if #[cfg(feature = "software-rendering")] {
        use {
            criterion::{criterion_group, criterion_main, Criterion},
            livesplit_core::{
                layout::{self, Layout},
                rendering::software::Renderer,
                run::parser::livesplit,
                Run, Segment, TimeSpan, Timer, TimingMethod,
            },
            std::{fs::File, io::BufReader},
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

            start_run(&mut timer);
            make_progress_run_with_splits_opt(&mut timer, &[Some(5.0), None, Some(10.0)]);

            let snapshot = timer.snapshot();
            let mut state = layout.state(&snapshot);
            let mut renderer = Renderer::new();

            // Do a single frame beforehand as otherwise the layout state will
            // keep saying that the icons changed.
            renderer.render(&state, [300, 500]);
            layout.update_state(&mut state, &snapshot);

            c.bench_function("Software Rendering (Default)", move |b| {
                b.iter(|| renderer.render(&state, [300, 500]))
            });
        }

        fn subsplits_layout(c: &mut Criterion) {
            let run = lss("tests/run_files/Celeste - Any% (1.2.1.5).lss");
            let mut timer = Timer::new(run).unwrap();
            let mut layout = lsl("tests/layout_files/subsplits.lsl");

            start_run(&mut timer);
            make_progress_run_with_splits_opt(&mut timer, &[Some(10.0), None, Some(20.0), Some(55.0)]);

            let snapshot = timer.snapshot();
            let mut state = layout.state(&snapshot);
            let mut renderer = Renderer::new();

            // Do a single frame beforehand as otherwise the layout state will
            // keep saying that the icons changed.
            renderer.render(&state, [300, 800]);
            layout.update_state(&mut state, &snapshot);

            c.bench_function("Software Rendering (Subsplits Layout)", move |b| {
                b.iter(|| renderer.render(&state, [300, 800]))
            });
        }

        fn file(path: &str) -> BufReader<File> {
            BufReader::new(File::open(path).unwrap())
        }

        fn lss(path: &str) -> Run {
            livesplit::parse(file(path), None).unwrap()
        }

        fn lsl(path: &str) -> Layout {
            layout::parser::parse(file(path)).unwrap()
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
