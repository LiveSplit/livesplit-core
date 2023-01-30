use crate::{run::LinkedLayout, Run};

#[test]
fn changing_does_nothing_when_run_doesnt_have_a_linked_layout() {
    let mut run = Run::new();

    assert_eq!(run.linked_layout(), None);
    assert!(!run.layout_path_changed(None::<&str>));
    assert_eq!(run.linked_layout(), None);
    assert!(!run.layout_path_changed(Some("foo")));
    assert_eq!(run.linked_layout(), None);
}

#[test]
fn changes_properly() {
    let mut run = Run::new();

    run.set_linked_layout(Some(LinkedLayout::Default));
    assert_eq!(run.linked_layout(), Some(&LinkedLayout::Default));

    assert!(!run.layout_path_changed(None::<&str>));
    assert_eq!(run.linked_layout(), Some(&LinkedLayout::Default));

    assert!(run.layout_path_changed(Some("foo")));
    assert_eq!(run.linked_layout(), Some(&LinkedLayout::Path("foo".into())));

    assert!(run.layout_path_changed(Some("bar")));
    assert_eq!(run.linked_layout(), Some(&LinkedLayout::Path("bar".into())));

    assert!(!run.layout_path_changed(Some("bar")));
    assert_eq!(run.linked_layout(), Some(&LinkedLayout::Path("bar".into())));

    assert!(run.layout_path_changed(None::<&str>));
    assert_eq!(run.linked_layout(), Some(&LinkedLayout::Default));
}
