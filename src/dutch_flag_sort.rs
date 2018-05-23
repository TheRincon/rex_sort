/* use std::ops::{Add, Sub};
use std::cmp::PartialOrd;

extern crate rand;

use rand::Rng;
use quick_sort;

fn dutch_flag_partition<T>(a: &mut [T], mid: usize) -> (usize, usize)  where T: Ord {
    let mut n = a.len() - 1;
    let mut i = 0;
    let mut j = 0;
    while j <= n {
        if a[j] < a[mid] {
            a.swap(i, j);
            i += 1;
            j += 1;
        } else if a[j] > a[mid] {
            a.swap(j, n);
            n -= 1;
        } else {
            j += 1;
        }
    }
    return (i, j);
}

pub fn dutch_flag_sort<T>(a: &mut [T]) where T: Ord {
    let d: usize = a.len() / 3;
    if a.len() <= 1 {
        return
    }
    if d <= 1 {
        return;
    }
    let (i, j) = dutch_flag_partition(a, d);
    dutch_flag_sort(&mut a[0..i]);
    dutch_flag_sort(&mut a[i..j]);
    dutch_flag_sort(&mut a[j..]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tester() {
        let mut t = vec ! [2, 4, 3, 1];
        assert_eq ! ([1, 2, 3, 4], dutch_flag_sort(t));
    }
}

*/