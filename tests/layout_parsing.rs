mod layout_files;

mod parse {
    use crate::layout_files;
    use livesplit_core::{
        Component,
        component::{splits, text},
        layout::{Layout, parser::parse},
    };

    #[track_caller]
    fn livesplit(data: &str) -> Layout {
        parse(data).unwrap()
    }

    #[track_caller]
    fn ls1l(data: &str) -> Layout {
        Layout::from_settings(serde_json::from_str(data).unwrap())
    }

    #[test]
    fn all() {
        livesplit(layout_files::ALL);
    }

    #[test]
    fn dark() {
        livesplit(layout_files::DARK);
    }

    #[test]
    fn subsplits() {
        livesplit(layout_files::SUBSPLITS);
    }

    #[test]
    fn wsplit() {
        livesplit(layout_files::WSPLIT);
    }

    #[test]
    fn with_timer_delta_background() {
        livesplit(layout_files::WITH_TIMER_DELTA_BACKGROUND);
    }

    #[test]
    fn custom_variable_splits() {
        let l = livesplit(layout_files::CUSTOM_VARIABLE_SPLITS);
        let Some(splits) = l.components.iter().find_map(|c| match c {
            Component::Splits(s) => Some(s),
            _ => None,
        }) else {
            panic!("Splits component not found");
        };
        let texts: Vec<_> = l
            .components
            .iter()
            .filter_map(|c| match c {
                Component::Text(t) => Some(t),
                _ => None,
            })
            .collect();
        {
            let splits::ColumnKind::Variable(splits::VariableColumn { ref variable_name }) =
                splits.settings().columns[2].kind
            else {
                panic!("expected ColumnKind::Variable");
            };
            assert_eq!(variable_name, "delta hits");
        }
        {
            let splits::ColumnKind::Variable(splits::VariableColumn { ref variable_name }) =
                splits.settings().columns[3].kind
            else {
                panic!("expected ColumnKind::Variable");
            };
            assert_eq!(variable_name, "segment hits");
        }
        {
            let text::Text::Variable(ref variable_name, is_split) = texts[0].settings().text else {
                panic!("expected Text::Variable");
            };
            assert_eq!(variable_name, "hits");
            assert!(is_split);
        }
        {
            let text::Text::Variable(ref variable_name, is_split) = texts[1].settings().text else {
                panic!("expected Text::Variable");
            };
            assert_eq!(variable_name, "pb hits");
            assert!(is_split);
        }
        let l1 = ls1l(layout_files::CUSTOM_VARIABLE_LS1L);
        assert_eq!(
            serde_json::to_string(&l.settings()).ok(),
            serde_json::to_string(&l1.settings()).ok()
        );
    }

    #[test]
    fn custom_variable_subsplits() {
        let l = livesplit(layout_files::CUSTOM_VARIABLE_SUBSPLITS);
        let Some(splits) = l.components.iter().find_map(|c| match c {
            Component::Splits(s) => Some(s),
            _ => None,
        }) else {
            panic!("Splits component not found");
        };
        let texts: Vec<_> = l
            .components
            .iter()
            .filter_map(|c| match c {
                Component::Text(t) => Some(t),
                _ => None,
            })
            .collect();
        {
            let splits::ColumnKind::Variable(splits::VariableColumn { ref variable_name }) =
                splits.settings().columns[2].kind
            else {
                panic!("expected ColumnKind::Variable");
            };
            assert_eq!(variable_name, "delta hits");
        }
        {
            let splits::ColumnKind::Variable(splits::VariableColumn { ref variable_name }) =
                splits.settings().columns[3].kind
            else {
                panic!("expected ColumnKind::Variable");
            };
            assert_eq!(variable_name, "segment hits");
        }
        {
            let text::Text::Variable(ref variable_name, is_split) = texts[0].settings().text else {
                panic!("expected Text::Variable");
            };
            assert_eq!(variable_name, "hits");
            assert!(is_split);
        }
        {
            let text::Text::Variable(ref variable_name, is_split) = texts[1].settings().text else {
                panic!("expected Text::Variable");
            };
            assert_eq!(variable_name, "pb hits");
            assert!(is_split);
        }
        let l1 = ls1l(layout_files::CUSTOM_VARIABLE_LS1L);
        assert_eq!(
            serde_json::to_string(&l.settings()).ok(),
            serde_json::to_string(&l1.settings()).ok()
        );
    }

    #[test]
    fn assert_order_of_default_columns() {
        use livesplit_core::component::splits;

        // The layout parser assumes that the order is from right to left. If it
        // changes, the layout parser needs to be adjusted as well.
        let component = splits::Component::default();
        let columns = &component.settings().columns;
        assert_eq!(columns[0].name, "Time");
        assert_eq!(columns[1].name, "+/−");
    }
}
