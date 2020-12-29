mod layout_files;

mod parse {
    use crate::layout_files;
    use livesplit_core::layout::{parser::parse, Layout};
    use std::io::Cursor;

    fn file(data: &[u8]) -> Cursor<&[u8]> {
        Cursor::new(data)
    }

    fn livesplit(data: &[u8]) -> Layout {
        parse(file(data)).unwrap()
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
        livesplit(layout_files::WITH_TIMER_GRADIENT_BACKGROUND);
        // FIXME: Add a rendering test to render out the gradient once we have
        // support for this.
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
