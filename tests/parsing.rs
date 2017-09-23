extern crate livesplit_core;

mod parse {
    use std::fs::File;
    use std::io::BufReader;
    use livesplit_core::run::parser::{livesplit, llanfair, llanfair_gered, quick_livesplit,
                                      time_split_tracker, urn, wsplit};

    fn file(path: &str) -> BufReader<File> {
        BufReader::new(File::open(path).unwrap())
    }

    fn livesplit(path: &str) {
        let old = livesplit::parse(file(path), None).unwrap();
        let new = quick_livesplit::parse(file(path), None).unwrap();
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
        llanfair_gered::parse(file("tests/run_files/llanfair_gered_with_refs.lfs")).unwrap();
    }

    #[test]
    fn llanfair_gered() {
        llanfair_gered::parse(file("tests/run_files/llanfair_gered.lfs")).unwrap();
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
