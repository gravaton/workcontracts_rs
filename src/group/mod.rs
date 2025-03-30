use std::{error::Error, sync::{atomic::{AtomicBool, AtomicU64, AtomicU8}, Condvar, Mutex}};

use token::ReleaseToken;
use waitable::WaitableState;

use crate::{get_enum, WorkContractId};

mod waitable;
mod token;

type WorkFunction<'a> = dyn Fn(&ReleaseToken) -> () + 'a;
type ReleaseFunction<'a> = dyn Fn() -> () + 'a;
type ExceptionFunction<'a, T: Error> = dyn Fn(T) -> () + 'a;

struct Contract<'a> {
    flags: AtomicU8,
    work: Box<WorkFunction<'a>>
}

pub struct WorkContractGroup<'a, T: Error> {
    stopped: AtomicBool,
    non_zero_counter: AtomicU64,
    contracts: Vec<Contract<'a>>,
    release: Vec<Box<ReleaseFunction<'a>>>,
    exception: Vec<Box<ExceptionFunction<'a, T>>>,
    waitable_state: WaitableState
}

impl<'a, T: Error> WorkContractGroup<'a, T> {
    fn get_available_contract(&self) -> Option<WorkContractId> {
        /*
        for (auto i = 0ull; i < available_.size(); ++i) {
            auto subTreeIndex (nextAvailableTreeIndex_++ & subTreeMask_);
            if (!available_[subTreeIndex].empty()) {
                if (auto [signalIndex, _] = available_[subTreeIndex].select<largest_child_selector>(0); signalIndex != ~0ull) {
                    work_contract_id workContractId(subTreeIndex * signal_tree_capacity);
                    workContractId += signalIndex;
                    return workContractId;
                }
            }
        }
        return ~0ull; 
        
         */
        None
    }

    pub fn create_contract(&mut self, work: &'a WorkFunction<'a>, release: &'a ReleaseFunction<'a>, exception: &'a ExceptionFunction<'a, T>) {
        /*

        if (auto workContractId = get_available_contract(); workContractId != ~0ull) {
            auto & contract = contracts_[workContractId];
            contract.flags_ = 0;
            if constexpr (work_contract_token_callable<std::decay_t<decltype(workFunction)>>)
                contract.work_ = std::forward<std::decay_t<decltype(workFunction)>>(workFunction); 
            if constexpr (work_contract_no_token_callable<std::decay_t<decltype(workFunction)>>)
                contract.work_ = [work = std::forward<std::decay_t<decltype(workFunction)>>(workFunction)](auto &) mutable{work();};

            release_[workContractId] = std::forward<decltype(releaseFunction)>(releaseFunction); 
            exception_[workContractId] = std::forward<decltype(exceptionFunction)>(exceptionFunction);
            return {this, releaseToken_[workContractId] = std::make_shared<release_token>(this), workContractId, initialState};
        }
        return {};
         */
        self.get_available_contract().map(|id| {
            let contract = &mut self.contracts[id as usize];
            contract.flags.store(0, std::sync::atomic::Ordering::SeqCst);
            contract.work = Box::new(work);
            self.release[id as usize] = Box::new(release);
            self.exception[id as usize] = Box::new(exception);
        });
        todo!()
    }

    fn incr_non_zero(&self) {
        let order = const { get_enum(0) };
        self.non_zero_counter.fetch_add(1, order);
    }

    fn decr_non_sero(&self) {
        self.non_zero_counter.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}