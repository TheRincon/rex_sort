// Not my code, credit to shepmaster from stack overflow

use std::cmp::Ordering;

#[derive(PartialEq,PartialOrd)]
struct NonNan(f64);

impl NonNan {
    fn new(val: f64) -> Option<NonNan> {
        if val.is_nan() {
            None
        } else {
            Some(NonNan(val))
        }
    }
}

impl Eq for NonNan {}

impl Ord for NonNan {
    fn cmp(&self, other: &NonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let mut v: Vec<_> = [2.0, 1.0, 3.0].iter().map(|v| NonNan::new(*v).unwrap()).collect();
    v.sort();
    let r = v.binary_search(&NonNan::new(2.0).unwrap());
    println!("{:?}", r);
}