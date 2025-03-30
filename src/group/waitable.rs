use std::{error::Error, sync::{atomic::Ordering, Condvar, Mutex}, time::Duration};

use super::WorkContractGroup;

// This is the default Atomic access ording if nothing else is specified
const ORDER: Ordering = Ordering::SeqCst;

pub struct WaitableState {
    mutex: Mutex<bool>,
    condvar: Condvar
}

impl WaitableState {
    fn notify_all(&self) {
        let lock = self.mutex.lock();
        self.condvar.notify_all();
    }

    fn wait<T: Error>(&self, owner: &WorkContractGroup<T>) -> bool {
        /*
        takes pointer to owner

        if (owner->nonZeroCounter_ == 0)
                {
                    std::unique_lock uniqueLock(mutex_);
                    conditionVariable_.wait(uniqueLock, [owner](){return ((owner->nonZeroCounter_ != 0) || (owner->stopped_));});
                    return (!owner->stopped_);
                }
                return true;
         */
        if owner.non_zero_counter.load(ORDER) == 0 {
            let guard = self.mutex.lock().unwrap();
            let _unused = self.condvar.wait(guard);
            return !owner.stopped.load(ORDER);
        }
        true
        
    }

    fn wait_for<T: Error>(&self, owner: &WorkContractGroup<T>, nanos: u64) -> bool{
        /*
        takes pointer to owner and duration in NS

        if (owner->nonZeroCounter_ == 0)
                {
                    std::unique_lock uniqueLock(mutex_);
                    auto waitSuccess = conditionVariable_.wait_for(uniqueLock, duration, [owner]() mutable{return ((owner->nonZeroCounter_ != 0) || (owner->stopped_));});
                    return ((!owner->stopped_) && (waitSuccess));
                }
                return true;
         */
        if owner.non_zero_counter.load(ORDER) == 0 {
            let guard = self.mutex.lock().unwrap();
            let dur = Duration::from_nanos(nanos);
            
            let (_unused, wait_success) = self.condvar.wait_timeout_while(guard, dur, |_x| (owner.non_zero_counter.load(ORDER) != 0 || owner.stopped.load(ORDER))).unwrap();
            return !owner.stopped.load(ORDER) && wait_success.timed_out();
            
        }
        true
        
    }
}