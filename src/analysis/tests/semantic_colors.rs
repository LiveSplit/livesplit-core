use crate::{
    Timer, TimingMethod,
    analysis::split_color,
    comparison,
    settings::SemanticColor,
    util::tests_helper::{
        create_timer, make_progress_run_with_splits_opt, run_with_splits, span, start_run,
    },
};

#[test]
fn segment_colors_are_correct() {
    let mut timer = create_timer(&["A", "B"]);

    run_with_splits(&mut timer, &[10.0, 20.0]);

    start_run(&mut timer);
    make_progress_run_with_splits_opt(&mut timer, &[Some(7.0)]);

    assert_eq!(color(&timer, -2.5), SemanticColor::AheadLosingTime);
    assert_eq!(color(&timer, -5.0), SemanticColor::AheadGainingTime);

    assert_eq!(color(&timer, 0.0), SemanticColor::Default);

    timer.reset(false).unwrap();

    start_run(&mut timer);
    make_progress_run_with_splits_opt(&mut timer, &[Some(15.0)]);

    assert_eq!(color(&timer, 2.5), SemanticColor::BehindGainingTime);
    assert_eq!(color(&timer, 8.0), SemanticColor::BehindLosingTime);

    assert_eq!(color(&timer, 0.0), SemanticColor::Default);
}

fn color(timer: &Timer, delta: f64) -> SemanticColor {
    split_color(
        timer,
        Some(span(delta)),
        1,
        true,
        false,
        comparison::personal_best::NAME,
        TimingMethod::GameTime,
    )
}
