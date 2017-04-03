use KeyEvent;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct KeyCode;

pub struct Hook;

pub fn register_hook<F>(callback: F) -> Result<Hook, ()>
    where F: FnMut(KeyEvent) + Send + 'static
{
    drop(callback);
    Ok(Hook)
}
