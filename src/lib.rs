extern crate num;

pub mod collatz;
pub mod collatz64;
pub use collatz::Collatz;
pub use collatz64::Collatz64;

pub struct CollatzType<T> {
    start: T,
}

/*
impl<T: PartialOrd> CollatzType<T> {
    fn new(n: T) -> CollatzType<T> {
        if n <= 0 { panic!("Input must be a positive integer!") }
        CollatzType { start: T }
    }
}
*/
