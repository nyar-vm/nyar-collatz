use crate::CollatzType;
use num::BigInt;

pub type Collatz = CollatzType<BigInt>;

impl From<i64> for Collatz {
    fn from(i: i64) -> Collatz {
        if i <= 0 {
            panic!("Input must be a positive integer!")
        }
        let n = BigInt::from(i);
        Collatz { start: n }
    }
}

impl From<BigInt> for Collatz {
    fn from(n: BigInt) -> Collatz {
        if n <= BigInt::from(0) {
            panic!("Input must be a positive integer!")
        }
        Collatz { start: n }
    }
}

impl Iterator for Collatz {
    type Item = BigInt;
    fn next(&mut self) -> Option<Self::Item> {
        let n = self.start.clone();
        if n.clone() % 2 == BigInt::from(0) {
            self.start = n / 2;
            return Some(self.start.clone());
        } else if n.clone() == BigInt::from(1) {
            return None;
        } else {
            self.start = n * 3 + 1;
            return Some(self.start.clone());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn collatz_0() {
        let mut collatz = Collatz::from(0);
        assert_eq!(collatz.next(), None);
    }

    #[test]
    fn collatz_13() {
        let mut collatz = Collatz::from(13);
        assert_eq!(collatz.next(), Some(BigInt::from(40)));
        assert_eq!(collatz.next(), Some(BigInt::from(20)));
        assert_eq!(collatz.next(), Some(BigInt::from(10)));
        assert_eq!(collatz.next(), Some(BigInt::from(5)));
        assert_eq!(collatz.next(), Some(BigInt::from(16)));
        assert_eq!(collatz.next(), Some(BigInt::from(8)));
        assert_eq!(collatz.next(), Some(BigInt::from(4)));
        assert_eq!(collatz.next(), Some(BigInt::from(2)));
        assert_eq!(collatz.next(), Some(BigInt::from(1)));
        assert_eq!(collatz.next(), None);
    }
}
