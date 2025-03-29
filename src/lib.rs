use std::sync::atomic::{AtomicU64, AtomicUsize};

pub struct PackedLeafNodes {
    data: AtomicU64
}

impl PackedLeafNodes {
    pub fn new() -> Self {
        Self { data: AtomicU64::new(0) }
    }

    pub fn select(&self) {
        
    }

    pub fn set(&self, idx: u8) {
        
    }
}


pub struct PackedNodes {
    data: AtomicU64
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
