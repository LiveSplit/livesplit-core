use super::super::RunMetadata;

#[test]
fn removing_shifts_the_speedrun_com_variables() {
    let mut data = RunMetadata::new();
    data.set_speedrun_com_variable("A", "A");
    data.set_speedrun_com_variable("B", "B");
    data.set_speedrun_com_variable("C", "C");
    data.set_speedrun_com_variable("D", "D");
    assert_eq!(
        data.speedrun_com_variables()
            .map(|(k, _)| k)
            .collect::<Vec<_>>(),
        ["A", "B", "C", "D"]
    );
    data.remove_speedrun_com_variable("B");
    assert_eq!(
        data.speedrun_com_variables()
            .map(|(k, _)| k)
            .collect::<Vec<_>>(),
        ["A", "C", "D"]
    );
}
