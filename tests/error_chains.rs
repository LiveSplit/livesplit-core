use core::{error::Error, num::ParseIntError};
use livesplit_core::{
    Lang, TimeSpan,
    run::parser::{
        face_split, flitter, libresplit, opensplit, portal2_live_timer, source_live_timer,
        speedrun_igt, splitterino, splitty,
    },
    timing::ParseError,
};

#[test]
fn time_parse_errors_keep_their_messages_and_sources() {
    for (text, message, has_source) in [
        ("1.x", "Couldn't parse the fractional part.", true),
        ("x", "Couldn't parse the seconds, minutes, or hours.", true),
        ("1:60", "A piece separate by a colon is too large.", false),
        ("1:2:3:4", "There are too many colons in the string.", false),
        (
            "18446744073709551615:00",
            "The time is too large to be represented.",
            false,
        ),
        (
            "0.12345678€",
            "The fractional part contains characters that are not digits.",
            false,
        ),
    ] {
        let error = TimeSpan::parse(text, Lang::English).unwrap_err();
        assert_eq!(error.to_string(), message, "{text}");
        if has_source {
            let source = error.source().unwrap();
            assert!(source.is::<ParseIntError>());
            assert!(source.source().is_none());
        } else {
            assert!(error.source().is_none());
        }
    }
}

#[test]
fn parser_errors_preserve_the_entire_source_chain() {
    let error = face_split::parse("Category\nGoal\n1\n\nSegment-invalid-0\n", None).unwrap_err();
    assert_eq!(
        error.to_string(),
        "Failed to parse the split time of a segment."
    );

    let time_error = error.source().unwrap();
    assert!(time_error.is::<ParseError>());
    assert_eq!(
        time_error.to_string(),
        "Couldn't parse the seconds, minutes, or hours."
    );

    let int_error = time_error.source().unwrap();
    assert!(int_error.is::<ParseIntError>());
    assert!(int_error.source().is_none());

    let error = face_split::parse("Category\n", None).unwrap_err();
    assert_eq!(error.to_string(), "Expected the goal, but didn't find it.");
    assert!(error.source().is_none());
}

#[test]
fn parser_messages_include_the_original_field_values() {
    let error = portal2_live_timer::parse("Header\nwrong_map,0,1\n").unwrap_err();
    assert_eq!(
        error.to_string(),
        "Expect the map `sp_a1_intro1` but found `wrong_map`."
    );
    assert!(error.source().is_none());
}

#[test]
fn json_parsers_keep_their_sources_with_std() {
    fn check(error: &(dyn Error + 'static)) {
        assert_eq!(error.to_string(), "Failed to parse JSON.");
        #[cfg(feature = "std")]
        {
            let source = error.source().unwrap();
            assert!(source.is::<serde_json::Error>());
            assert!(source.source().is_none());
        }
        #[cfg(not(feature = "std"))]
        assert!(error.source().is_none());
    }

    check(&flitter::parse("{").unwrap_err());
    check(&libresplit::parse("{", None).unwrap_err());
    check(&opensplit::parse("{").unwrap_err());
    check(&source_live_timer::parse("{").unwrap_err());
    check(&speedrun_igt::parse("{").unwrap_err());
    check(&splitterino::parse("{").unwrap_err());
    check(&splitty::parse("{").unwrap_err());
}

#[test]
fn flitter_validation_keeps_its_errors_and_order() {
    for (names, golds, splits, message) in [
        ("[]", "[]", "[]", "The split names can't be empty."),
        (r#"["a"]"#, "[]", "[]", "The golds can't be empty."),
        (
            r#"["a"]"#,
            "[null]",
            "[]",
            "The personal best can't be empty.",
        ),
        (
            r#"["a","b"]"#,
            "[null]",
            "[null]",
            "The split name count does not match the gold count.",
        ),
        (
            r#"["a"]"#,
            "[null]",
            "[null,null]",
            "The split name count does not match the personal best split count.",
        ),
        (
            r#"["a"]"#,
            "[null]",
            "[null]",
            "The last split of the personal best can't be null.",
        ),
    ] {
        let source = format!(
            r#"{{"title":"Game","category":"Any%","attempts":0,"split_names":{names},"golds":{golds},"personal_best":{{"splits":{splits}}}}}"#
        );
        let error = flitter::parse(&source).unwrap_err();
        assert_eq!(error.to_string(), message);
        assert!(error.source().is_none());
    }
}
