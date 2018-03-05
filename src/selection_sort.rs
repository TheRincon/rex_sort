use std::ops::{Add, Sub};
use std::cmp::PartialOrd;

pub fn selection_sort<T>(a: &mut [T]) where T: PartialOrd {
    let n = a.len();
    for j in 0..n {
        let mut min = j;
        for i in j+1..n {
            if a[i] < a[min] {
                min = i;
            }
        }

        if min != j {
            a.swap(j, min);
        }

    }
}