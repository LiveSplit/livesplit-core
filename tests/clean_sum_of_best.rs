use livesplit_core::{
    Lang,
    run::{editor::SumOfBestCleaner, parser::livesplit},
};

mod run_files;

#[test]
fn dont_panic_if_attempt_doesnt_exist() {
    // There's an attempt that does not exist that it wants to clean up. We
    // shouldn't panic then and it should just ignore it.
    let mut run = livesplit::parse(run_files::CLEAN_SUM_OF_BEST).unwrap();
    let mut cleaner = SumOfBestCleaner::new(&mut run, Lang::English);
    let cleanup = cleaner.next_potential_clean_up().unwrap().into();
    cleaner.apply(cleanup);
    assert!(cleaner.next_potential_clean_up().is_none());
}
