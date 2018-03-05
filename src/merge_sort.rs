use std::cmp::Ord;
use std::cmp::min;

fn top_down_split_merge<T>(b: &mut [T], beg: usize, end: usize, a: &mut [T]) where T: PartialOrd + Clone {
    if end - beg < 2 {
        return
    }
    let mid = (end + beg) / 2;
    top_down_split_merge(b, beg, mid, a);
    top_down_split_merge(b, mid + 1, end, a);

    top_down_merge(a, b, beg, mid, end);

}

fn top_down_merge<T>(a: &mut [T], b: &mut [T], l: usize, r: usize, e: usize) where T: PartialOrd + Clone {

    let mut i = l;
    let mut j = r;
    for k in l..e {
        if i < r && (j >= e || a[i] <= a[j]) {
            b[k] = a[i].clone();
            i += 1;
        } else {
            b[i] = a[j].clone();
            j += 1;
        }
    }
}

pub fn merge_sort2<T>(a: &mut [T])  where T: PartialOrd + Clone {

    let size = a.len();
    let mut b: Vec<T> = Vec::with_capacity(size);
    unsafe { b.set_len(size); }
    copy_vec2(a, &mut b, 0, size);
    top_down_split_merge(&mut b, 0, size, a);
}

fn copy_vec2<T>(a: &mut [T], b: &mut [T], c: usize, d: usize) where T: PartialOrd + Clone {
    for i in c..d {
        b[i] = a[i].clone();
    }
}



pub fn bottom_Up_merge_sort<T>(a: &mut [T], b: &mut [T]) where T: PartialOrd + Clone {
    let mut width: usize = 1;
    let mut i: usize = 0;
    let n = a.len();
    while width < n {
        while i < n {
            bottom_up_merge(a, b, i, min(i+width, n), min(i+2*width, n));
            i = i + (2 * width);
        }
        copy_vec2(a, b, 0, n);
        width = 2 * width;
    }
}

fn bottom_up_merge<T>(a: &mut [T], b: &mut [T], i: usize , j: usize, k: usize) where T: PartialOrd + Clone {
    let mut left = i;
    let mut right = j;
    for m in i..k {
        if left < j && (right >= k || (a[left] <= a[right])) {
            b[m] = a[left].clone();
            left += 1;
        } else {
            b[m] = a[right].clone();
            right += 1;
        }
    }
}

pub fn merge_sort<T>(a: &mut [T])  where T: PartialOrd + Clone {

    let size = a.len();
    let mut b: Vec<T> = Vec::with_capacity(size);
    unsafe { b.set_len(size); }
    bottom_Up_merge_sort(a, &mut b);
}