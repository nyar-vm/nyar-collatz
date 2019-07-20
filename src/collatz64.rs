use crate::CollatzType;

pub type Collatz64 = CollatzType<u64>;

impl From<u64> for Collatz64 {
    fn from(n: u64) -> Collatz64 {
        if n == 0 {
            panic!("Input must be a positive integer!")
        }
        Collatz64 { start: n }
    }
}

impl Iterator for Collatz64 {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start % 2 == 0 {
            self.start = self.start / 2;
            return Some(self.start);
        } else if self.start == 1 {
            return None;
        } else {
            self.start = self.start * 3 + 1;
            Some(self.start)
        }
    }
}

pub fn count_collatz(n: u32, lengths: &mut [u16; 500_000]) -> u16 {
    if n < 500_000 && lengths[n as usize] > 0 {
        lengths[n as usize]
    } else {
        if n == 1 {
            return 1;
        } else {
            let val = 1 + match n % 2 {
                0 => count_collatz(n / 2, lengths),
                _ => count_collatz(n * 3 + 1, lengths),
            };
            if n < 500_000 {
                lengths[n as usize] = val;
            }
            val
        }
    }
}

pub fn longest_collatz_memo(highest: u32) -> u32 {
    let mut max_length = 0;
    let mut lengths: [u16; 500_000] = [0; 500_000];
    let mut cause = 0;
    for i in 1..highest + 1 {
        let length = count_collatz(i, &mut lengths);
        if length > max_length {
            cause = i;
            max_length = length;
        }
    }
    cause
}

pub fn longest_collatz(highest: usize) -> usize {
    let mut max_length = 0;
    let mut cause = 0;
    for i in 1..highest + 1 {
        let mut length = 1;
        let mut n = i;
        loop {
            if n == 1 {
                break;
            }
            n = match n % 2 {
                0 => n / 2,
                _ => n * 3 + 1,
            };
            length += 1;
        }
        if length > max_length {
            max_length = length;
            cause = i;
        }
    }
    cause
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn collatz_0() {
        let mut collatz = Collatz64::from(0u64);
        assert_eq!(collatz.next(), None);
    }

    #[test]
    fn collatz_13() {
        let mut collatz = Collatz64::from(13u64);
        assert_eq!(collatz.next(), Some(40));
        assert_eq!(collatz.next(), Some(20));
        assert_eq!(collatz.next(), Some(10));
        assert_eq!(collatz.next(), Some(5));
        assert_eq!(collatz.next(), Some(16));
        assert_eq!(collatz.next(), Some(8));
        assert_eq!(collatz.next(), Some(4));
        assert_eq!(collatz.next(), Some(2));
        assert_eq!(collatz.next(), Some(1));
        assert_eq!(collatz.next(), None);
    }

    #[test]
    fn test_longest_collatz() {
        assert_eq!(longest_collatz(1), 1);
        assert_eq!(longest_collatz(2), 2);
        assert_eq!(longest_collatz(3), 3)
    }

    #[test]
    fn test_longest_collatz_memo() {
        assert_eq!(longest_collatz_memo(1), 1);
    }
}
