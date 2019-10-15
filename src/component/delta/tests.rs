use super::Component;
use crate::{GeneralLayoutSettings, Run, Segment, Timer};

#[test]
fn comparison_text() {
    let mut run = Run::new();
    run.push_segment(Segment::new("Ok"));

    let timer = Timer::new(run).unwrap();
    let mut delta_comp = Component::new();
    let settings = GeneralLayoutSettings::default();

    // No Override
    delta_comp.settings_mut().comparison_override = None;
    assert_eq!(delta_comp.name(), "Delta"); // Displayed in Layout Editor
    assert_eq!(
        &*delta_comp.state(&timer, &settings).key,
        timer.current_comparison()
    ); // Displayed in Layout

    // Good Override
    let mut comp = "Personal Best";
    delta_comp.settings_mut().comparison_override = Some(comp.to_owned());
    assert_eq!(delta_comp.name(), format!("Delta ({})", comp));
    assert_eq!(&*delta_comp.state(&timer, &settings).key, comp);

    // Good Override
    comp = "Best Segments";
    delta_comp.settings_mut().comparison_override = Some(comp.to_owned());
    assert_eq!(delta_comp.name(), format!("Delta ({})", comp));
    assert_eq!(&*delta_comp.state(&timer, &settings).key, comp);

    // Good Override
    comp = "None";
    delta_comp.settings_mut().comparison_override = Some(comp.to_owned());
    assert_eq!(delta_comp.name(), format!("Delta ({})", comp));
    assert_eq!(&*delta_comp.state(&timer, &settings).key, comp);

    // Bad Override
    comp = "Fake Comparison";
    delta_comp.settings_mut().comparison_override = Some(comp.to_owned());
    assert_eq!(delta_comp.name(), format!("Delta ({})", comp));
    assert_eq!(
        &*delta_comp.state(&timer, &settings).key,
        timer.current_comparison()
    );
}
