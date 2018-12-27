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
}
