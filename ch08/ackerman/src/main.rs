//! アッカーマン関数

use num::{BigUint, FromPrimitive, One, Zero};

const M: usize = 4;
const N: usize = 4;

fn main() {
    let m = M;
    let n = BigUint::from_usize(N).unwrap();
    let a = ackerman(m, n.clone());
    println!("ackerman({M}, {N}) = {a}");
}

fn ackerman(m: usize, n: BigUint) -> BigUint {
    let one: BigUint = One::one();
    let zero: BigUint = Zero::zero();
    if m == 0 {
        n + one
    } else if n == zero {
        ackerman(m - 1, one)
    } else {
        ackerman(m - 1, ackerman(m, n - one))
    }
}

#[cfg(test)]
mod tests {
    use num::{BigUint, FromPrimitive, ToPrimitive};

    #[test]
    fn test_ackerman() {
        let a = crate::ackerman(3, BigUint::from_usize(3).unwrap());
        assert_eq!(a.to_usize().unwrap(), 61);
    }
}
