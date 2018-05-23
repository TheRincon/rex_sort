use std::mem;
use std::ops::{Add, Sub};
use std::cmp::Ord;

pub fn quick_sort<T>(a: &mut [T]) where T: PartialOrd {
    let hi = a.len()-1;
    quick_sort_real(a, 0, hi);
}

fn quick_sort_real<T>(a: &mut [T], lo: usize, hi: usize) where T: PartialOrd {
    if lo < hi {
        let p = partition(a, lo, hi);
        // u32 triggers a panic when p == 0, i.e. 0usize - 1; this modification stops it.
        if p == 0 {
            quick_sort_real(a, lo, p);
        } else { quick_sort_real(a, lo, p - 1); }
        quick_sort_real(a, p + 1, hi);
    }
}

pub fn partition<T>(a: &mut [T], lo: usize, hi: usize) -> usize where T: PartialOrd {
    let mut i = lo;
    let mut k = hi;
    for j in lo..hi {
        if a[j] <= a[hi] {
            a.swap(i, j);
            i += 1;
        }
    }
    if a[hi] <= a[i] {
        a.swap(k, i);
    }
    return i;
}
