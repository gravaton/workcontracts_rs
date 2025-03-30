use std::{error::Error, sync::atomic::AtomicU64};

mod group;
mod signal_tree;


// WorkContractId is hard defined as a u64 in the original C++
type WorkContractId = u64;

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

const fn get_enum(x: u8) -> std::sync::atomic::Ordering {
    match x {
        0 => std::sync::atomic::Ordering::Relaxed,
        _ => std::sync::atomic::Ordering::AcqRel
    }
}

// work_contract_group.cpp/h

struct Contract {}



pub struct ReleaseToken {
    // pointer to the group
}

impl ReleaseToken {

}


// Very basic skeleton of the contract itself from the header file
pub struct WorkContract {
    owner: usize,
    release_token: Option<usize>,
    id: usize
}

impl WorkContract {
    pub fn id(&self) -> usize {
        // equivalent to get_id in the C++
        self.id
    }

    pub fn schedule(&self) {
        // owner_->schedule(id_);
        todo!();
    }

    pub fn release(&mut self) -> bool {
        if let Some(t) = self.release_token.take() {
            /*
             releaseToken->schedule(*this);
            owner_ = {};
            return true;
             */
            true
        } else {
            false
        }
    }

    pub fn deschedule(&self) -> bool {
        todo!()
    }

    pub fn is_valid(&self) -> bool {
        if let Some(t) = self.release_token {
            // return ((releaseToken_) && (releaseToken_->is_valid()));
            true
        } else {
            false
        }
    }
}

impl Drop for WorkContract {
    fn drop(&mut self) {
        self.release();
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
