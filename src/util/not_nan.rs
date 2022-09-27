use core::cmp::Ordering;

#[repr(transparent)]
pub struct NotNaN(pub f64);

impl PartialEq for NotNaN {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for NotNaN {}

impl PartialOrd for NotNaN {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NotNaN {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}
