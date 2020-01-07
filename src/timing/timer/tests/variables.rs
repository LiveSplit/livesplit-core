use crate::tests_helper::create_timer;

#[test]
fn can_set_variable() {
    let mut timer = create_timer(&["A"]);

    assert!(timer.run().metadata().custom_variable("Points").is_none());

    timer.set_custom_variable("Points", "10");
    assert_eq!(
        timer
            .run()
            .metadata()
            .custom_variable_value("Points")
            .unwrap(),
        "10"
    );

    timer.start();

    assert_eq!(
        timer
            .run()
            .metadata()
            .custom_variable_value("Points")
            .unwrap(),
        "10"
    );

    timer.set_custom_variable("Points", "20");

    assert_eq!(
        timer
            .run()
            .metadata()
            .custom_variable_value("Points")
            .unwrap(),
        "20"
    );

    timer.set_custom_variable("Points", "30");

    assert_eq!(
        timer
            .run()
            .metadata()
            .custom_variable_value("Points")
            .unwrap(),
        "30"
    );

    timer.split();

    assert_eq!(
        timer
            .run()
            .metadata()
            .custom_variable_value("Points")
            .unwrap(),
        "30"
    );

    timer.set_custom_variable("Points", "40");

    assert_eq!(
        timer
            .run()
            .metadata()
            .custom_variable_value("Points")
            .unwrap(),
        "40"
    );

    timer.set_custom_variable("Points", "50");

    assert_eq!(
        timer
            .run()
            .metadata()
            .custom_variable_value("Points")
            .unwrap(),
        "50"
    );

    timer.reset(true);

    assert_eq!(
        timer
            .run()
            .metadata()
            .custom_variable_value("Points")
            .unwrap(),
        "50"
    );

    timer.set_custom_variable("Points", "60");

    assert_eq!(
        timer
            .run()
            .metadata()
            .custom_variable_value("Points")
            .unwrap(),
        "60"
    );

    assert!(
        !timer
            .run()
            .metadata()
            .custom_variable("Points")
            .unwrap()
            .is_permanent,
    );
}
