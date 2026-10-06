use super::*;

pub(super) const LIST: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<AutoSplitters>
    <!-- Unknown elements must not contribute games or URLs. -->
    <Future><AutoSplitter><Games><Game>Hidden</Game></Games></AutoSplitter></Future>
    <AutoSplitter>
        <Games><Game>Game &amp; One</Game><Future/><Game>Alias &#x32;</Game></Games>
        <URLs><URL>https://example.com/legacy.asl</URL></URLs>
        <Type>Script</Type>
        <Description>Legacy description</Description>
        <Website>https://example.com/legacy</Website>
        <AutoSplittingRuntime>
            <URL>https://example.com/game.wasm?x=1&amp;y=2</URL>
            <Description>Runtime &lt;description&gt;</Description>
            <Website>https://example.com/runtime</Website>
            <Future><URL>https://example.com/ignored.wasm</URL></Future>
        </AutoSplittingRuntime>
    </AutoSplitter>
    <AutoSplitter>
        <Games><Game>WASM only</Game></Games>
        <URLs><URL>https://example.com/only.wasm</URL><URL>https://example.com/data.bin</URL></URLs>
        <ScriptType>AutoSplittingRuntime</ScriptType>
        <Description>WASM description</Description>
    </AutoSplitter>
    <AutoSplitter>
        <Games><Game>Fallback</Game></Games>
        <URLs><URL>https://example.com/fallback.asl</URL></URLs>
        <Description>Fallback description</Description>
        <Website>https://example.com/fallback</Website>
        <AutoSplittingRuntime><URL>https://example.com/fallback.wasm</URL></AutoSplittingRuntime>
    </AutoSplitter>
    <AutoSplitter>
        <Games><Game>Legacy</Game></Games>
        <URLs><URL>https://example.com/legacy.asl</URL></URLs>
        <Description/>
    </AutoSplitter>
</AutoSplitters>"#;

#[test]
fn parses_both_runtime_formats_and_metadata_overrides() {
    let list = List::new(LIST);
    let splitter = list.get_for_game("Game & One").unwrap().unwrap();

    assert!(splitter.is_using_auto_splitting_runtime());
    assert_eq!(splitter, list.get_for_game("Alias 2").unwrap().unwrap());
    assert_eq!(splitter.urls(), ["https://example.com/game.wasm?x=1&y=2"]);
    assert_eq!(splitter.description, "Runtime <description>");

    assert_eq!(
        splitter.website.as_deref(),
        Some("https://example.com/runtime")
    );

    assert_eq!(
        list.get_description_for_game("Game & One")
            .unwrap()
            .as_deref(),
        Some("Runtime <description>")
    );

    assert_eq!(
        list.get_website_for_game("Game & One").unwrap().as_deref(),
        Some("https://example.com/runtime")
    );

    let splitter = list.get_for_game("WASM only").unwrap().unwrap();

    assert!(splitter.is_using_auto_splitting_runtime());

    assert_eq!(
        splitter.urls(),
        [
            "https://example.com/only.wasm",
            "https://example.com/data.bin"
        ]
    );

    assert_eq!(splitter.website, None);
    assert_eq!(splitter.description, "WASM description");

    let splitter = list.get_for_game("Fallback").unwrap().unwrap();

    assert!(splitter.is_using_auto_splitting_runtime());
    assert_eq!(splitter.urls(), ["https://example.com/fallback.wasm"]);
    assert_eq!(splitter.description, "Fallback description");

    assert_eq!(
        splitter.website.as_deref(),
        Some("https://example.com/fallback")
    );

    let splitter = list.get_for_game("Legacy").unwrap().unwrap();
    assert!(!splitter.is_using_auto_splitting_runtime());
    assert_eq!(splitter.description, "");

    for name in ["game & one", "Hidden", "Unknown"] {
        assert!(list.get_for_game(name).unwrap().is_none());
        assert_eq!(list.get_description_for_game(name).unwrap(), None);
        assert_eq!(list.get_website_for_game(name).unwrap(), None);
    }
}

#[test]
fn borrows_source_and_unescaped_metadata() {
    let source = LIST.to_owned();
    let list = List::new(&source);

    assert_eq!(list.source().as_ptr(), source.as_ptr());
    assert_eq!(list.source(), source);

    let splitter = list.get_for_game("WASM only").unwrap().unwrap();

    assert!(matches!(
        splitter.description,
        Cow::Borrowed("WASM description")
    ));

    assert!(
        splitter
            .urls()
            .iter()
            .all(|url| matches!(url, Cow::Borrowed(_)))
    );

    let splitter = list.get_for_game("Game & One").unwrap().unwrap();
    assert!(matches!(splitter.description, Cow::Owned(_)));
    assert!(matches!(splitter.urls()[0], Cow::Owned(_)));
    assert!(matches!(splitter.website, Some(Cow::Borrowed(_))));
}

#[test]
fn skips_unrelated_metadata_and_stops_after_first_match() {
    // An unrelated entry's missing fields do not affect the requested entry,
    // and nothing past the first match is parsed, even a truncated tail.
    let source = r#"<AutoSplitters>
        <AutoSplitter><Games><Game>Other</Game></Games><AutoSplittingRuntime/></AutoSplitter>
        <AutoSplitter>
            <Description>First</Description><URLs><URL>first.wasm</URL></URLs>
            <Games><Game>Target</Game></Games>
        </AutoSplitter>
        <AutoSplitter><Games>"#;

    let list = List::new(source);
    let splitter = list.get_for_game("Target").unwrap().unwrap();

    assert_eq!(splitter.description, "First");
    assert_eq!(splitter.urls(), ["first.wasm"]);
    assert!(list.get_for_game("Other").is_err());
    assert!(list.get_for_game("Missing").is_err());
}

#[test]
fn uses_first_entry_when_games_are_duplicated() {
    let source = r#"<AutoSplitters>
        <AutoSplitter><Games><Game>Test</Game></Games><URLs/><Description>First</Description></AutoSplitter>
        <AutoSplitter><Games><Game>Test</Game></Games><URLs/><Description>Second</Description></AutoSplitter>
    </AutoSplitters>"#;

    assert_eq!(
        List::new(source)
            .get_description_for_game("Test")
            .unwrap()
            .as_deref(),
        Some("First")
    );
}

#[test]
fn empty_lists_have_no_matches() {
    for list in [
        List::empty(),
        List::default(),
        List::new("<AutoSplitters></AutoSplitters>"),
    ] {
        assert!(list.get_for_game("Unknown").unwrap().is_none());

        assert!(
            List::new(list.source())
                .get_for_game("Unknown")
                .unwrap()
                .is_none()
        );
    }
}

#[test]
fn queries_report_incomplete_xml() {
    for source in [
        "",
        "<WrongRoot/>",
        "<AutoSplitters>",
        "<AutoSplitters><AutoSplitter>",
        "<AutoSplitters><AutoSplitter/></AutoSplitters>",
        "<AutoSplitters><Future>",
    ] {
        assert!(List::new(source).get_for_game("Test").is_err(), "{source}");
    }
}

#[test]
fn queries_report_missing_required_fields() {
    let entry = "<Games><Game>Test</Game></Games><URLs><URL>https://example.com/test.wasm</URL></URLs><Description>Test</Description>";

    for tag in ["Games", "URLs", "Description"] {
        let start = entry.find(&format!("<{tag}>")).unwrap();
        let end = entry.find(&format!("</{tag}>")).unwrap() + tag.len() + 3;

        let source = format!(
            "<AutoSplitters><AutoSplitter>{}{}</AutoSplitter></AutoSplitters>",
            &entry[..start],
            &entry[end..]
        );

        assert!(
            matches!(List::new(&source).get_for_game("Test"), Err(Error::MissingElement { element }) if element == tag)
        );
    }

    let source = LIST.replace("<URL>https://example.com/fallback.wasm</URL>", "<Future/>");
    let list = List::new(&source);
    assert!(list.get_for_game("WASM only").unwrap().is_some());

    assert!(matches!(
        list.get_for_game("Fallback"),
        Err(Error::MissingElement { element: "URL" })
    ));
}
