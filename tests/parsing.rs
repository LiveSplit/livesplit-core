extern crate livesplit_core;

mod parse {
    use std::fs::File;
    use std::io::{BufReader, Cursor};
    use livesplit_core::Run;
    use livesplit_core::run::parser::{composite, livesplit, llanfair, llanfair_gered,
                                      source_live_timer, splitterz, time_split_tracker, urn,
                                      worstrun, wsplit, TimerKind, llanfair2};

    fn file(path: &str) -> BufReader<File> {
        BufReader::new(File::open(path).unwrap())
    }

    fn livesplit(path: &str) -> Run {
        livesplit::parse(file(path), None).unwrap()
    }

    fn parse_llanfair_gered(path: &str) {
        llanfair_gered::parse(file(path)).unwrap();
    }

    fn parse_llanfair2(path: &str) {
        llanfair2::parse(file(path)).unwrap();
    }

    #[test]
    fn livesplit_fuzz_crash() {
        let path = "tests/run_files/livesplit_fuzz_crash.lss";
        livesplit::parse(file(path), None).unwrap_err();
    }

    #[test]
    fn livesplit_fuzz_crash_utf8() {
        let path = "tests/run_files/livesplit_fuzz_crash_utf8.lss";
        livesplit::parse(file(path), None).unwrap_err();
    }

    #[test]
    fn livesplit_1_0() {
        livesplit("tests/run_files/livesplit1.0.lss");
    }

    #[test]
    fn livesplit_1_4() {
        livesplit("tests/run_files/livesplit1.4.lss");
    }

    #[test]
    fn livesplit_1_5() {
        livesplit("tests/run_files/livesplit1.5.lss");
    }

    #[test]
    fn livesplit_1_6() {
        livesplit("tests/run_files/livesplit1.6.lss");
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
    fn zeroed_out_doesnt_parse_as_llanfair() {
        llanfair::parse(Cursor::new(&mut [0u8; 64][..])).unwrap_err();
    }

    #[test]
    fn llanfair_gered_doesnt_parse_as_livesplit() {
        livesplit::parse(file("tests/run_files/llanfair_gered.lfs"), None).unwrap_err();
    }

    #[test]
    fn llanfair_gered() {
        parse_llanfair_gered("tests/run_files/llanfair_gered.lfs");
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
        let run =
            time_split_tracker::parse(file("tests/run_files/timesplittracker.txt"), None).unwrap();
        assert_eq!(
            run.custom_comparisons(),
            [
                "Personal Best",
                "Time",
                "Custom Comparison",
                "Race",
                "Race2"
            ]
        );
    }

    #[test]
    fn splitterz() {
        splitterz::parse(file("tests/run_files/splitterz"), false).unwrap();
    }

    #[test]
    fn wsplit() {
        wsplit::parse(file("tests/run_files/wsplit"), false).unwrap();
    }

    #[test]
    fn urn() {
        urn::parse(file("tests/run_files/urn.json")).unwrap();
    }

    #[test]
    fn source_live_timer() {
        source_live_timer::parse(file("tests/run_files/source_live_timer.json")).unwrap();
    }

    #[test]
    fn source_live_timer2() {
        source_live_timer::parse(file("tests/run_files/source_live_timer2.json")).unwrap();
    }

    #[test]
    fn worstrun() {
        worstrun::parse(file("tests/run_files/worstrun.json")).unwrap();
    }

    #[test]
    fn worstrun_prefers_parsing_as_itself() {
        let run = composite::parse(file("tests/run_files/worstrun.json"), None, false).unwrap();
        assert_eq!(run.kind, TimerKind::Worstrun);
    }

    #[test]
    fn urn_prefers_parsing_as_itself() {
        let run = composite::parse(file("tests/run_files/urn.json"), None, false).unwrap();
        assert_eq!(run.kind, TimerKind::Urn);
    }

    #[test]
    fn source_live_time_prefers_parsing_as_itself() {
        let run =
            composite::parse(file("tests/run_files/source_live_timer.json"), None, false).unwrap();
        assert_eq!(run.kind, TimerKind::SourceLiveTimer);
    }
}
