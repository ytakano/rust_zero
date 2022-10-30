//! 末尾呼び出し版のアッカーマン関数実装

use num::{BigUint, One, Zero};

#[derive(Debug, Clone)]
pub enum N {
    VAL(BigUint),
    A(usize, BigUint),
}

impl N {
    fn get(self) -> BigUint {
        match self {
            N::VAL(n) => n,
            N::A(m, n) => ackerman_tail(m, N::VAL(n)),
        }
    }
}

pub fn ackerman(m: usize, n: BigUint) -> BigUint {
    ackerman_tail(m, N::VAL(n))
}

fn ackerman_tail(m: usize, n: N) -> BigUint {
    let one: BigUint = One::one();
    let zero: BigUint = Zero::zero();
    if m == 0 {
        n.get() + one // n + 1
    } else if n.clone().get() == zero {
        ackerman_tail(m - 1, N::VAL(one))
    } else {
        let n_dec = n.get() - one; // n - 1
        ackerman_tail(m - 1, N::A(m, n_dec))
    }
}
