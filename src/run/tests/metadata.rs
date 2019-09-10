use super::super::RunMetadata;

#[test]
fn removing_shifts_the_variables() {
    let mut data = RunMetadata::new();
    data.set_variable("A", "A");
    data.set_variable("B", "B");
    data.set_variable("C", "C");
    data.set_variable("D", "D");
    assert_eq!(
        data.variables().map(|(k, _)| k).collect::<Vec<_>>(),
        ["A", "B", "C", "D"]
    );
    data.remove_variable("B");
    assert_eq!(
        data.variables().map(|(k, _)| k).collect::<Vec<_>>(),
        ["A", "C", "D"]
    );
}
