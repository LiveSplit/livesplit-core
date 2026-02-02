use super::Component;
use crate::{
    GeneralLayoutSettings, Run, Segment, Timer, comparison,
    localization::{Lang, Text},
};

#[test]
fn comparison_text() {
    let mut run = Run::new();
    run.push_segment(Segment::new("Ok"));

    let timer = Timer::new(run).unwrap();
    let mut delta_comp = Component::new();
    let settings = GeneralLayoutSettings::default();

    // No Override
    delta_comp.settings_mut().comparison_override = None;
    assert_eq!(
        delta_comp.name(Lang::English),
        Text::ComponentDelta.resolve(Lang::English)
    ); // Displayed in Layout Editor
    assert_eq!(
        &*delta_comp
            .state(&timer.snapshot(), &settings, Lang::English)
            .key,
        timer.current_comparison()
    ); // Displayed in Layout

    // Good Override
    let mut comp = "Personal Best";
    delta_comp.settings_mut().comparison_override = Some(comp.to_owned());
    assert_eq!(
        delta_comp.name(Lang::English),
        format!(
            "{} ({})",
            Text::ComponentDelta.resolve(Lang::English),
            comparison::shorten(comp)
        )
    );
    assert_eq!(
        &*delta_comp
            .state(&timer.snapshot(), &settings, Lang::English)
            .key,
        comp
    );

    // Good Override
    comp = "Best Segments";
    delta_comp.settings_mut().comparison_override = Some(comp.to_owned());
    assert_eq!(
        delta_comp.name(Lang::English),
        format!(
            "{} ({})",
            Text::ComponentDelta.resolve(Lang::English),
            comparison::shorten(comp)
        )
    );
    assert_eq!(
        &*delta_comp
            .state(&timer.snapshot(), &settings, Lang::English)
            .key,
        comp
    );

    // Good Override
    comp = "None";
    delta_comp.settings_mut().comparison_override = Some(comp.to_owned());
    assert_eq!(
        delta_comp.name(Lang::English),
        format!(
            "{} ({})",
            Text::ComponentDelta.resolve(Lang::English),
            comparison::shorten(comp)
        )
    );
    assert_eq!(
        &*delta_comp
            .state(&timer.snapshot(), &settings, Lang::English)
            .key,
        comp
    );

    // Bad Override
    comp = "Fake Comparison";
    delta_comp.settings_mut().comparison_override = Some(comp.to_owned());
    assert_eq!(
        delta_comp.name(Lang::English),
        format!(
            "{} ({})",
            Text::ComponentDelta.resolve(Lang::English),
            comparison::shorten(comp)
        )
    );
    assert_eq!(
        &*delta_comp
            .state(&timer.snapshot(), &settings, Lang::English)
            .key,
        timer.current_comparison()
    );
}
