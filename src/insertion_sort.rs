use std::ops::{Add, Sub};
use std::cmp::PartialOrd;

pub fn recursive_insertion_sort<T>(a: & mut [T], n: usize) where T: PartialOrd {
    if n > 0 {
        recursive_insertion_sort(a, n - 1);
        for i in 0..a.len() {
            for j in (0..i).rev() {
                if a[j] >= a[j + 1] {
                    a.swap(j + 1, j);
                } else {
                    break;
                }
            }
        }
    }
}

pub fn insertion_sort<T>(a: &mut [T]) where T: PartialOrd {
    for i in 0..a.len() {
        for j in (0..i).rev() {
            if a[j] >= a[j + 1] {
                a.swap(j + 1, j);
            } else {
                break;
            }
        }
    }
}