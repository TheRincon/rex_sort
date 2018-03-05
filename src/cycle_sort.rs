use std::ops::{Add, Sub};
use std::cmp::{PartialOrd, PartialEq};

// Possible improvement.

// When the array contains only duplicates of a relatively small number of items,
// a constant-time perfect hash function can greatly speed up finding where to put an item1,
// turning the sort from Θ(n2) time to Θ(n + k) time, where k is the total number of hashes.
// The array ends up sorted in the order of the hashes, so choosing a hash function that gives
// you the right ordering is important.

pub fn cycle_sort<T>(a: &mut [T]) where T: PartialOrd + Clone {
    let mut writes = 0;
    let n = a.len();

    for i in 0..n {
        let mut itemm: T = a[i].clone();

        let mut pos = i;
        for k in i + 1..n {
            if a[k] < itemm {
                pos += 1;
            }
        }
        if pos == i {
            continue;
        }
        while itemm == a[pos] {
            pos += 1;
        }
        let mut tmp = a[pos].clone();
        a[pos] = itemm;
        itemm = tmp;
        writes += 1;

        while pos != i {
            pos = i;
            for l in i + 1..a.len() {
                if a[l] < itemm {
                    pos += 1;
                }
            }
            while itemm == a[pos] {
                pos += 1;
            }
            tmp = a[pos].clone();
            a[pos] = itemm;
            itemm = tmp;
            writes += 1;
        }
    }
}