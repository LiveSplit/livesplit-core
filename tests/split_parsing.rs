//! Integration tests for parsing representative real-world splits files and
//! detecting their timer kinds. The files in `run_files` are reserved for this
//! suite. Tests for specific fields, edge cases, and parser semantics should use
//! inline inputs in the corresponding parser's local test module.

mod run_files;

mod parse {
    use crate::run_files;
    use livesplit_core::{
        Run, TimeSpan,
        analysis::total_playtime,
        run::parser::{
            TimerKind, composite, flitter, libresplit, livesplit, llanfair, llanfair_gered,
            opensplit, portal2_live_timer, source_live_timer, speedrun_igt, splitterino, splitterz,
            time_split_tracker, wsplit,
        },
        run::saver,
    };

    #[track_caller]
    fn livesplit(data: &str) -> Run {
        livesplit::parse(data).unwrap()
    }

    fn minimal_lss(version: &str, segment_names: &[&str], extra: &str) -> String {
        let segments = segment_names
            .iter()
            .map(|name| {
                format!(
                    "<Segment><Name>{name}</Name><Icon/><SplitTimes/><BestSegmentTime/><SegmentHistory/></Segment>"
                )
            })
            .collect::<String>();
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?><Run version="{version}"><GameIcon/><GameName>Game</GameName><CategoryName>Any%</CategoryName><Offset>00:00:00</Offset><AttemptCount>0</AttemptCount><Segments>{segments}</Segments>{extra}<AutoSplitterSettings/></Run>"#
        )
    }

    #[track_caller]
    fn parse_llanfair_gered(data: &str) {
        llanfair_gered::parse(data).unwrap();
    }

    #[test]
    fn livesplit_fuzz_crash() {
        livesplit::parse(run_files::LIVESPLIT_FUZZ_CRASH).unwrap_err();
    }

    #[test]
    fn livesplit_fuzz_crash_utf8() {
        livesplit::parse(run_files::LIVESPLIT_FUZZ_CRASH_UTF8).unwrap_err();
    }

    #[test]
    fn livesplit_1_0() {
        livesplit(run_files::LIVESPLIT_1_0);
    }

    #[test]
    fn livesplit_1_4() {
        livesplit(run_files::LIVESPLIT_1_4);
    }

    #[test]
    fn livesplit_1_5() {
        livesplit(run_files::LIVESPLIT_1_5);
    }

    #[test]
    fn livesplit_1_6() {
        livesplit(run_files::LIVESPLIT_1_6);
    }

    #[test]
    fn livesplit_1_6_gametime() {
        livesplit(run_files::LIVESPLIT_1_6_GAMETIME);
    }

    #[test]
    fn livesplit_celeste() {
        livesplit(run_files::CELESTE);
    }

    #[test]
    fn livesplit_native_segment_groups() {
        // Keep a complete, real-world LSS file as the primary compatibility
        // test for the native segment-group format. In addition to exercising
        // the XML structure itself, this covers groups alongside the metadata,
        // histories, icons, comparisons, and auto splitter settings emitted by
        // an actual frontend.
        let run = livesplit(run_files::CELESTE_NATIVE_SEGMENT_GROUPS);

        assert_eq!(run.len(), 33);
        let groups = run.segment_groups().groups();
        assert_eq!(groups.len(), 7);
        assert_eq!(
            groups
                .iter()
                .map(|group| (group.start(), group.end()))
                .collect::<Vec<_>>(),
            [
                (1, 4),
                (4, 7),
                (7, 11),
                (11, 15),
                (15, 20),
                (20, 26),
                (26, 33)
            ]
        );
        assert_eq!(groups[0].name(), Some("Okay"));
        let group_icon_id = *groups[0].icon().unwrap().id();

        let mut saved = String::new();
        saver::livesplit::save_run(&run, &mut saved).unwrap();
        assert!(saved.contains(r#"<Run version="1.8.1">"#));
        assert!(saved.contains(r#"<SegmentGroup start="1" end="4">"#));
        assert!(saved.contains("<Name>Okay</Name>"));
        assert!(saved.contains("<Icon>"));

        let saved = livesplit::parse(&saved).unwrap();
        let group = &saved.segment_groups().groups()[0];
        assert_eq!(group.icon().unwrap().id(), &group_icon_id);
    }

    #[test]
    fn livesplit_1_8_1_preserves_legacy_subsplit_prefixes() {
        // Once native groups became part of the format, `-` stopped being
        // structural syntax. Keep this focused synthetic case because the
        // real-world native-group fixture has no segment with a literal prefix.
        let run = livesplit(&minimal_lss("1.8.1", &["Intro", "-Literal"], ""));

        assert!(run.segment_groups().groups().is_empty());
        assert_eq!(run.segment(1).name(), "-Literal");
    }

    #[test]
    fn livesplit_legacy_subsplits_are_upgraded() {
        let run = livesplit(&minimal_lss(
            "1.8.0",
            &["Intro", "-A1", "-A2", "{Chapter A} A End", "-Final"],
            "",
        ));

        assert_eq!(run.segment_groups().groups().len(), 1);
        let group = &run.segment_groups().groups()[0];
        assert_eq!((group.start(), group.end()), (1, 4));
        assert_eq!(group.name(), Some("Chapter A"));
        assert_eq!(run.segment(1).name(), "A1");
        assert_eq!(run.segment(3).name(), "A End");
        assert_eq!(run.segment(4).name(), "-Final");
    }

    #[test]
    fn livesplit_legacy_ungrouped_brace_prefix_is_preserved() {
        let run = livesplit(&minimal_lss(
            "1.8.0",
            &["Intro", "{Boss} Fight", "Outro"],
            "",
        ));

        assert!(run.segment_groups().groups().is_empty());
        assert_eq!(run.segment(1).name(), "{Boss} Fight");
    }

    #[test]
    fn livesplit_attempt_ended_bug() {
        let run = livesplit(run_files::LIVESPLIT_ATTEMPT_ENDED_BUG);
        let playtime = total_playtime::calculate(run);
        assert!(playtime >= TimeSpan::zero());
    }

    #[test]
    fn llanfair() {
        llanfair::parse(run_files::LLANFAIR).unwrap();
    }

    #[test]
    fn zeroed_out_doesnt_parse_as_llanfair() {
        llanfair::parse(&[0; 64]).unwrap_err();
    }

    #[test]
    fn llanfair_gered_doesnt_parse_as_livesplit() {
        livesplit::parse(run_files::LLANFAIR_GERED).unwrap_err();
    }

    #[test]
    fn llanfair_gered() {
        parse_llanfair_gered(run_files::LLANFAIR_GERED);
    }

    #[test]
    fn llanfair_gered_with_refs() {
        parse_llanfair_gered(run_files::LLANFAIR_GERED_WITH_REFS);
    }

    #[test]
    fn llanfair_gered_icons() {
        parse_llanfair_gered(run_files::LLANFAIR_GERED_ICONS);
    }

    #[test]
    fn time_split_tracker() {
        let run = time_split_tracker::parse(run_files::TIME_SPLIT_TRACKER, None).unwrap();
        assert_eq!(
            run.custom_comparisons(),
            [
                "Personal Best",
                "Time",
                "Custom Comparison",
                "Race",
                "Race 2"
            ]
        );
    }

    #[test]
    fn time_split_tracker_without_attempt_count() {
        time_split_tracker::parse(run_files::TIME_SPLIT_TRACKER_WITHOUT_ATTEMPT_COUNT, None)
            .unwrap();
    }

    #[test]
    fn splitterz() {
        splitterz::parse(run_files::SPLITTERZ, None).unwrap();
    }

    #[test]
    fn wsplit() {
        wsplit::parse(run_files::WSPLIT, None).unwrap();
    }

    #[test]
    fn splitterino() {
        splitterino::parse(run_files::SPLITTERINO).unwrap();
    }

    #[test]
    fn libresplit() {
        libresplit::parse(run_files::LIBRESPLIT, None).unwrap();
    }

    #[test]
    fn flitter() {
        flitter::parse(run_files::FLITTER).unwrap();
    }

    #[test]
    fn source_live_timer() {
        source_live_timer::parse(run_files::SOURCE_LIVE_TIMER).unwrap();
    }

    #[test]
    fn source_live_timer2() {
        source_live_timer::parse(run_files::SOURCE_LIVE_TIMER2).unwrap();
    }

    #[test]
    fn portal2_live_timer() {
        portal2_live_timer::parse(run_files::PORTAL2_LIVE_TIMER1).unwrap();
    }

    #[test]
    fn portal2_live_timer2() {
        portal2_live_timer::parse(run_files::PORTAL2_LIVE_TIMER2).unwrap();
    }

    #[test]
    fn speedrun_igt() {
        speedrun_igt::parse(run_files::SPEEDRUN_IGT).unwrap();
    }

    #[test]
    fn opensplit() {
        opensplit::parse(run_files::OPENSPLIT).unwrap();
    }

    #[test]
    fn speedrun_igt_prefers_parsing_as_itself() {
        let run = composite::parse(run_files::SPEEDRUN_IGT.as_bytes(), None).unwrap();
        assert!(matches!(run.kind, TimerKind::SpeedRunIGT));
    }

    #[test]
    fn portal2_live_timer_prefers_parsing_as_itself() {
        let run = composite::parse(run_files::PORTAL2_LIVE_TIMER1.as_bytes(), None).unwrap();
        assert_eq!(run.kind, TimerKind::Portal2LiveTimer);
    }

    #[test]
    fn splitterino_prefers_parsing_as_itself() {
        let run = composite::parse(run_files::SPLITTERINO.as_bytes(), None).unwrap();
        assert_eq!(run.kind, TimerKind::Splitterino);
    }

    #[test]
    fn libresplit_prefers_parsing_as_itself() {
        let run = composite::parse(run_files::LIBRESPLIT.as_bytes(), None).unwrap();
        assert_eq!(run.kind, TimerKind::LibreSplit);
    }

    #[test]
    fn source_live_time_prefers_parsing_as_itself() {
        let run = composite::parse(run_files::SOURCE_LIVE_TIMER.as_bytes(), None).unwrap();
        assert_eq!(run.kind, TimerKind::SourceLiveTimer);
    }

    #[test]
    fn flitter_prefers_parsing_as_itself() {
        let run = composite::parse(run_files::FLITTER.as_bytes(), None).unwrap();
        assert_eq!(run.kind, TimerKind::Flitter);
    }

    #[test]
    fn opensplit_prefers_parsing_as_itself() {
        let run = composite::parse(run_files::OPENSPLIT.as_bytes(), None).unwrap();
        assert_eq!(run.kind, TimerKind::OpenSplit);
    }
}
