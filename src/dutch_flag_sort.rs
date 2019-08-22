use std::ops::{Add, Sub};
use std::cmp::PartialOrd;
use std::fmt::Debug;

use quick_sort;

fn dutch_flag_partition<T>(a: &mut [T], mid: usize) -> (usize, usize)  where T: Ord + Debug {
    let mut n = a.len() - 1;
    let mut i = 0;
    let mut j = 0;
    while j <= n {
        if a[j] < a[mid] {
            println!("a[j] is {:?}", a[j]);
            println!("a[mid] is {:?}", a[mid]);
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

pub fn dutch_flag_sort<T>(a: &mut [T]) where T: Ord + Debug {
    let d: usize = a.len() / 3;
    println!("d is {:?}", d);
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
        let mut t = vec ! [2, 4, 3, 1, 9, 5, 6, 8, 7];
        dutch_flag_sort(&mut t);
        assert_eq!(t, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
