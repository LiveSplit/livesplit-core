extern crate livesplit_core;

mod parse {
    use std::fs::File;
    use std::io::BufReader;
    use livesplit_core::run::parser::{livesplit, llanfair, llanfair_gered, quick_livesplit,
                                      quick_llanfair_gered, time_split_tracker, urn, wsplit,
                                      llanfair2, quick_llanfair2};

    fn file(path: &str) -> BufReader<File> {
        BufReader::new(File::open(path).unwrap())
    }

    fn livesplit(path: &str) {
        let old = livesplit::parse(file(path), None).unwrap();
        let new = quick_livesplit::parse(file(path), None).unwrap();
        assert_eq!(old, new);
    }

    fn parse_llanfair_gered(path: &str) {
        let old = llanfair_gered::parse(file(path)).unwrap();
        let new = quick_llanfair_gered::parse(file(path)).unwrap();
        assert_eq!(old, new);
    }

    fn parse_llanfair2(path: &str) {
        let old = llanfair2::parse(file(path)).unwrap();
        let new = quick_llanfair2::parse(file(path)).unwrap();
        assert_eq!(old, new);
    }

    #[test]
    fn livesplit_fuzz_crash() {
        let path = "tests/run_files/quick_livesplit_fuzz_crash.lss";
        quick_livesplit::parse(file(path), None).ok();
    }

    #[test]
    fn livesplit_1_4() {
        livesplit("tests/run_files/livesplit1.4");
    }

    #[test]
    fn livesplit_1_5() {
        livesplit("tests/run_files/livesplit1.5.lss");
    }

    #[test]
    fn livesplit_1_6() {
        livesplit("tests/run_files/livesplit1.6");
    }

    #[test]
    fn livesplit_1_6_gametime() {
        livesplit("tests/run_files/livesplit1.6_gametime.lss");
    }

    #[test]
    fn llanfair() {
        llanfair::parse(file("tests/run_files/llanfair")).unwrap();
    }

    #[test]
    fn llanfair_gered_with_refs() {
        parse_llanfair_gered("tests/run_files/llanfair_gered_with_refs.lfs");
    }

    #[test]
    fn llanfair_gered_icons() {
        parse_llanfair_gered("tests/run_files/llanfair_gered_icons.lfs");
    }

    #[test]
    fn llanfair2() {
        parse_llanfair2("tests/run_files/llanfair2.xml")
    }

    #[test]
    fn llanfair2_empty() {
        parse_llanfair2("tests/run_files/llanfair2_empty.xml")
    }

    #[test]
    fn time_split_tracker() {
        time_split_tracker::parse(file("tests/run_files/timesplittracker.txt"), None).unwrap();
    }

    #[test]
    fn wsplit() {
        wsplit::parse(file("tests/run_files/wsplit"), false).unwrap();
    }

    #[test]
    fn urn() {
        urn::parse(file("tests/run_files/urn.json")).unwrap();
    }
}
