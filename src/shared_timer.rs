use Timer;
use clone_on_write::{History, Cow};
use std::sync::Arc;

#[derive(Clone)]
pub struct SharedTimer {
    history: Arc<History<Timer>>,
}

impl SharedTimer {
    pub fn new(timer: Cow<Timer>) -> Self {
        Self { history: Arc::new(History::new(timer)) }
    }

    pub fn get(&self) -> Cow<Timer> {
        self.history.get()
    }

    pub fn commit(&self, timer: Cow<Timer>) {
        self.history.commit(timer);
    }

    pub fn commit_with<F>(&self, f: F)
        where F: FnMut(&mut Timer)
    {
        self.history.commit_with(f);
    }

    pub fn split(&self) {
        self.commit_with(|t| { t.split(); });
    }

    pub fn skip_split(&self) {
        self.commit_with(|t| { t.skip_split(); });
    }

    pub fn reset(&self, update_splits: bool) {
        self.commit_with(|t| { t.reset(update_splits); });
    }

    pub fn pause(&self) {
        self.commit_with(|t| { t.pause(); });
    }

    pub fn switch_to_next_comparison(&self) {
        self.commit_with(|t| { t.switch_to_next_comparison(); })
    }

    pub fn switch_to_previous_comparison(&self) {
        self.commit_with(|t| { t.switch_to_previous_comparison(); })
    }

    pub fn undo(&self) {
        self.history.undo();
    }

    pub fn redo(&self) {
        self.history.redo();
    }
}
