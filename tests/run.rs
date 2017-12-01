extern crate livesplit_core;

mod run {
    use livesplit_core::{Run, Segment};
    use livesplit_core::run::Editor;

    #[test]
    fn add_comparison() {
        let mut run = Run::new();
        let c = run.add_custom_comparison("My Comparison");
        assert_eq!(c, Ok(()));
    }

    #[test]
    fn add_duplicate_comparison() {
        let mut run = Run::new();
        let c = run.add_custom_comparison("Best Segments");
        assert_eq!(c, Err(()));
    }

    #[test]
    fn add_comparison_editor() {
        let mut run = Run::new();
        run.push_segment(Segment::new("s"));
        let mut editor = Editor::new(run).unwrap();
        let c = editor.add_comparison("My Comparison");
        assert_eq!(c, Ok(()));
    }

    #[test]
    fn add_duplicate_comparison_editor() {
        let mut run = Run::new();
        run.push_segment(Segment::new("s"));
        let mut editor = Editor::new(run).unwrap();
        let c = editor.add_comparison("Best Segments");
        assert_eq!(c, Err(()));
    }

    #[test]
    fn rename_comparison_editor() {
        let mut run = Run::new();
        run.push_segment(Segment::new("s"));
        run.add_custom_comparison("Custom").ok();
        let mut editor = Editor::new(run).unwrap();
        let c = editor.rename_comparison("Custom", "My Comparison");
        assert_eq!(c, Ok(()));
    }
}
