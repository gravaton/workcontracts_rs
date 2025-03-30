//! Signal Tree is the signaling mechanism used in work contracts


type SignalIndex = usize;

pub struct SignalTree {}

trait TestTrait<const T: usize> {
    const D: usize = T / 2;
    type Next;
}

const fn half<const N: usize>() -> usize { N / 2 }

struct DefaultSelector<const C: usize, const B: usize, const BIAS: usize> {}

impl<const C: usize, const B: usize, const S: usize> DefaultSelector<C, B, S> {
    const CPH: usize = C / 2;
    const BPH: usize = Self::CPH * B;
    const RBM: usize = 1_usize << Self::BPH;
    const LBM: usize = Self::RBM << Self::BPH;

    const fn bph() -> usize { Self::CPH * B }

    pub fn operator(bias_flags: usize, counters: usize, next_bias: usize) -> SignalIndex {
        if C == 1 {
            return 0;
        }

        let right_counters = counters & Self::RBM;
        let left_counters = counters & Self::LBM;
        let bias_right = bias_flags & S;
        let mut choose_right = ((bias_right && right_counters) || (left_counters == 0));
        next_bias <<= 1;
        next_bias |= (right_counters != 0);
        counters >>= if choose_right { 0 } else { Self::BPH };
        let tim = half::<S>();
        let jim: [usize; Self::CPH];
        return 0 + DefaultSelector::<1, 2, {Self::CPH}>::operator(bias_flags, counters & r, next_bias);


        return 0;
    }
}

impl<const C: usize, const B: usize, const S: usize> TestTrait<C> for DefaultSelector<C, B, S> {
    type Next = [usize; DefaultSelector::<C, B, S>::D];
}