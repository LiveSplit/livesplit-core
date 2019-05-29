mod parse {
    use livesplit_core::run::parser::{
        composite, flitter, livesplit, llanfair, llanfair2, llanfair_gered, portal2_live_timer,
        source_live_timer, splits_io, splitterino, splitterz, time_split_tracker, urn, worstrun,
        wsplit, TimerKind,
    };
    use livesplit_core::{analysis::total_playtime, Run, TimeSpan};
    use std::fs::File;
    use std::io::{BufReader, Cursor};

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
    fn livesplit_celeste() {
        livesplit("tests/run_files/Celeste - Any% (1.2.1.5).lss");
    }

    #[test]
    fn livesplit_attempt_ended_bug() {
        let run = livesplit("tests/run_files/livesplit_attempt_ended_bug.lss");
        let playtime = total_playtime::calculate(run);
        assert!(playtime >= TimeSpan::zero());
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
    fn time_split_tracker_without_attempt_count() {
        time_split_tracker::parse(file("tests/run_files/1734.timesplittracker"), None).unwrap();
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
    fn splitterino() {
        splitterino::parse(file("tests/run_files/splitterino.splits")).unwrap();
    }

    #[test]
    fn urn() {
        urn::parse(file("tests/run_files/urn.json")).unwrap();
    }

    #[test]
    fn flitter() {
        flitter::parse(file("tests/run_files/flitter.scm")).unwrap();
    }

    #[test]
    fn flitter_small() {
        flitter::parse(file("tests/run_files/flitter-small.scm")).unwrap();
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
    fn portal2_live_timer() {
        portal2_live_timer::parse(file("tests/run_files/portal2_live_timer1.csv")).unwrap();
    }

    #[test]
    fn portal2_live_timer2() {
        portal2_live_timer::parse(file("tests/run_files/portal2_live_timer2.csv")).unwrap();
    }

    #[test]
    fn worstrun() {
        worstrun::parse(file("tests/run_files/worstrun.json")).unwrap();
    }

    #[test]
    fn splits_io() {
        splits_io::parse(file("tests/run_files/generic_splits_io.json")).unwrap();
    }

    #[test]
    fn splits_io_prefers_parsing_as_itself() {
        let run =
            composite::parse(file("tests/run_files/generic_splits_io.json"), None, false).unwrap();
        assert!(if let TimerKind::Generic(_) = run.kind {
            true
        } else {
            false
        });
    }

    #[test]
    fn portal2_live_timer_prefers_parsing_as_itself() {
        let run =
            composite::parse(file("tests/run_files/portal2_live_timer1.csv"), None, false).unwrap();
        assert_eq!(run.kind, TimerKind::Portal2LiveTimer);
    }

    #[test]
    fn worstrun_prefers_parsing_as_itself() {
        let run = composite::parse(file("tests/run_files/worstrun.json"), None, false).unwrap();
        assert_eq!(run.kind, TimerKind::Worstrun);
    }

    #[test]
    fn splitterino_prefers_parsing_as_itself() {
        let run =
            composite::parse(file("tests/run_files/splitterino.splits"), None, false).unwrap();
        assert_eq!(run.kind, TimerKind::Splitterino);
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

    #[test]
    fn flitter_prefers_parsing_as_itself() {
        let run = composite::parse(file("tests/run_files/flitter.scm"), None, false).unwrap();
        assert_eq!(run.kind, TimerKind::Flitter);

        let run = composite::parse(file("tests/run_files/flitter-small.scm"), None, false).unwrap();
        assert_eq!(run.kind, TimerKind::Flitter);
    }
}
