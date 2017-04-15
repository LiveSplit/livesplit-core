use super::Cow;
use parking_lot::RwLock;

#[derive(Debug, Default)]
pub struct History<T: ?Sized + Clone> {
    inner: RwLock<(usize, Vec<Cow<T>>)>,
}

impl<T: ?Sized + Clone> History<T> {
    pub fn new(value: Cow<T>) -> Self {
        Self { inner: RwLock::new((0, vec![value])) }
    }

    pub fn get(&self) -> Cow<T> {
        let lock = self.inner.read();
        lock.1[lock.0].clone()
    }

    pub fn commit(&self, value: Cow<T>) {
        let mut lock = self.inner.write();
        let index = lock.0;
        lock.1.truncate(index + 1);
        lock.1.push(value);
        lock.0 += 1;
    }

    pub fn compare_and_commit(&self, current: Cow<T>, new: Cow<T>) -> Result<Cow<T>, Cow<T>> {
        let mut lock = self.inner.write();
        let index = lock.0;
        let old = lock.1[index].clone();
        if Cow::ptr_eq(&old, &current) {
            lock.1.truncate(index + 1);
            lock.1.push(new.clone());
            lock.0 += 1;
            Ok(new)
        } else {
            Err(old)
        }
    }

    pub fn commit_with<F>(&self, mut f: F)
        where F: FnMut(&mut T)
    {
        let mut old = self.get();
        loop {
            let mut new = old.clone();
            f(&mut new);
            if let Err(e) = self.compare_and_commit(old, new) {
                old = e;
            } else {
                break;
            }
        }
    }

    pub fn undo(&self) {
        let mut lock = self.inner.write();
        if lock.0 > 0 {
            lock.0 -= 1;
        }
    }

    pub fn redo(&self) {
        let mut lock = self.inner.write();
        if lock.0 + 1 < lock.1.len() {
            lock.0 += 1;
        }
    }
}

#[test]
fn test() {
    let mut value = Cow::new(1);
    let history = History::new(value.clone());
    *value = 2;
    history.commit(value.clone());
    *value = 3;
    history.commit(value.clone());
    assert_eq!(*history.get(), 3);
    history.undo();
    assert_eq!(*history.get(), 2);
    history.undo();
    assert_eq!(*history.get(), 1);
    history.undo();
    assert_eq!(*history.get(), 1);
    history.redo();
    assert_eq!(*history.get(), 2);
    history.redo();
    assert_eq!(*history.get(), 3);
    history.redo();
    assert_eq!(*history.get(), 3);
    history.undo();
    assert_eq!(*history.get(), 2);
    *value = 4;
    history.commit(value.clone());
    assert_eq!(*history.get(), 4);
}
