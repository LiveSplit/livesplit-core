mod parse {
    use livesplit_core::layout::{parser::parse, Layout};
    use std::{fs::File, io::BufReader};

    fn file(path: &str) -> BufReader<File> {
        BufReader::new(File::open(path).unwrap())
    }

    fn livesplit(path: &str) -> Layout {
        parse(file(path)).unwrap()
    }

    #[test]
    fn all() {
        livesplit("tests/layout_files/All.lsl");
    }

    #[test]
    fn dark() {
        livesplit("tests/layout_files/dark.lsl");
    }

    #[test]
    fn subsplits() {
        livesplit("tests/layout_files/subsplits.lsl");
    }

    #[test]
    fn wsplit() {
        livesplit("tests/layout_files/WSplit.lsl");
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
