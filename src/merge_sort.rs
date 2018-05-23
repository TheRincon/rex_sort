use std::cmp::Ord;
use std::cmp::min;

pub fn merge<T>(a: &mut [T], b: &mut [T], start: usize, mid: usize, end: usize) where T: PartialOrd + Clone {
    let mut one = start;
    let mut two = mid;
    for i in start..end {
        if (one < mid) && (two >= end || a[one] <= a[two]) {
            b[i] = a[one].clone();
            one += 1;
        } else {
            b[i] = a[two].clone();
            two += 1;
        }
    }
}

pub fn copy_vec<T>(a: &mut [T], b: &mut [T], start: usize, end: usize) where T: PartialOrd + Clone {
    for i in start..end { a[i] = b[i].clone() }
}

pub fn merge_split<T>(a: &mut [T], b: &mut [T], start: usize, end: usize) where T: PartialOrd + Clone {
    let e = end as i64;
    let s = start as i64;
    if (e - s) <= 1 {
        return;
    } else {
        let mid = ((e + s) / 2 ) as usize;
        merge_split(a, b, start, mid);
        merge_split(a, b, mid, end);
        merge(a, b, start, mid, end);
        copy_vec(a, b, start, end);
    }
}

pub fn merge_sort<T>(a: &mut [T])  where T: PartialOrd + Clone {

    let size = a.len();
    let mut b: Vec<T> = Vec::with_capacity(size);
    unsafe { b.set_len(size); }
    merge_split(a, &mut b, 0, size);
}
