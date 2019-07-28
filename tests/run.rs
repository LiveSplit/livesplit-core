mod run {
    use livesplit_core::run::ComparisonError;
    use livesplit_core::Run;

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
        assert_eq!(c, Err(ComparisonError::DuplicateName));
    }
}

mod editor {
    use livesplit_core::run::{ComparisonError, Editor, RenameError};
    use livesplit_core::{Run, Segment};

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
        assert_eq!(c, Err(ComparisonError::DuplicateName));
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

    #[test]
    fn rename_comparison_bad_name() {
        let mut run = Run::new();
        run.push_segment(Segment::new("s"));
        run.add_custom_comparison("Custom").ok();
        let mut editor = Editor::new(run).unwrap();
        let c = editor.rename_comparison("Bad", "My Comparison");
        assert_eq!(c, Err(RenameError::OldNameNotFound));
    }

    #[test]
    fn rename_comparison_name_starts_with_race() {
        let mut run = Run::new();
        run.push_segment(Segment::new("s"));
        run.add_custom_comparison("Custom").ok();
        let mut editor = Editor::new(run).unwrap();
        let c = editor.rename_comparison("Custom", "[Race]Custom");
        assert_eq!(
            c,
            Err(RenameError::InvalidName {
                source: ComparisonError::NameStartsWithRace
            })
        );
    }

    #[test]
    fn rename_comparison_duplicate_name() {
        let mut run = Run::new();
        run.push_segment(Segment::new("s"));
        run.add_custom_comparison("Custom").ok();
        run.add_custom_comparison("Custom2").ok();
        let mut editor = Editor::new(run).unwrap();
        let c = editor.rename_comparison("Custom2", "Custom");
        assert_eq!(
            c,
            Err(RenameError::InvalidName {
                source: ComparisonError::DuplicateName
            })
        );
    }
}
