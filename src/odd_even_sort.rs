use std::ops::{Add, Sub};
use std::cmp::PartialOrd;

pub fn odd_even_sort<T>(a: &mut [T]) where T: PartialOrd {
    let n = a.len();
    for i in 0..n {
        if i % 2 == 1 {
            let mut j = 2;
            while j < n {
                if a[j] < a[j - 1] {
                    a.swap(j - 1, j);
                }
                j += 2;
            }
        } else {
            let mut k = 1;
            while k < n {
                if a[k] < a[k-1] {
                    a.swap(k - 1, k);
                }
                k += 2;
            }
        }
    }
}