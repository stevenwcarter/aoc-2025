use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Debug, Default, Clone)]
pub struct Maxer {
    value: Arc<AtomicUsize>,
}

impl Maxer {
    pub fn max(&mut self, value: usize) {
        let mut current = self.value.load(Ordering::Relaxed);
        while value > current {
            match self
                .value
                .compare_exchange(current, value, Ordering::Relaxed, Ordering::Relaxed)
            {
                Ok(_) => return,
                Err(prev) => current = prev,
            }
        }
    }

    pub fn get(&self) -> usize {
        self.value.load(Ordering::Relaxed)
    }
}
