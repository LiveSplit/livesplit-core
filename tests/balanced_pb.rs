extern crate livesplit_core;

use livesplit_core::comparison::balanced_pb::{BalancedPB, NAME};
use livesplit_core::run::parser::livesplit;
use livesplit_core::{Time, TimeSpan};
use std::fs::File;
use std::io::BufReader;

fn r(t: Time) -> Time {
    Time::new()
        .with_real_time(
            t.real_time
                .map(|t| TimeSpan::from_seconds(t.total_seconds().floor())),
        )
        .with_game_time(
            t.game_time
                .map(|t| TimeSpan::from_seconds(t.total_seconds().floor())),
        )
}

fn t(r: &str, g: &str) -> Time {
    Time::new()
        .with_real_time(r.parse().ok())
        .with_game_time(g.parse().ok())
}

#[test]
fn balanced_pb() {
    let reader = BufReader::new(File::open("tests/run_files/livesplit1.6_gametime.lss").unwrap());
    let mut run = livesplit::parse(reader, None).unwrap();
    run.comparison_generators_mut().clear();
    run.comparison_generators_mut().push(Box::new(BalancedPB));
    run.regenerate_comparisons();
    let s = run.segments();

    assert_eq!(r(s[0].comparison(NAME)), t("3:11", "3:11"));
    assert_eq!(r(s[1].comparison(NAME)), t("4:24", "4:21"));
    assert_eq!(r(s[2].comparison(NAME)), t("6:38", "6:31"));
    assert_eq!(r(s[3].comparison(NAME)), t("10:34", "10:13"));
    assert_eq!(r(s[4].comparison(NAME)), t("13:05", "12:34"));
    assert_eq!(r(s[5].comparison(NAME)), t("15:02", "14:21"));
    assert_eq!(r(s[6].comparison(NAME)), t("17:50", "16:56"));
    assert_eq!(r(s[7].comparison(NAME)), t("22:42", "21:30"));
    assert_eq!(r(s[8].comparison(NAME)), t("26:20", "24:47"));
    assert_eq!(r(s[9].comparison(NAME)), t("30:24", "28:37"));
    assert_eq!(r(s[10].comparison(NAME)), t("36:51", "34:39"));
    assert_eq!(r(s[11].comparison(NAME)), t("37:56", "35:39"));
    assert_eq!(r(s[12].comparison(NAME)), t("40:01", "37:37"));
}
