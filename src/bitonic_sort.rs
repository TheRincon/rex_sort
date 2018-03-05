use std::ops::{Add, Sub};
use std::cmp::{PartialOrd, PartialEq};

/* pub fn bitonic_sort<T: Add>(a: &mut [T]) -> &mut [T] where T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd {
    let n = a.len();
    if a.len() <= 1 {
        return a;
    } else {
        let first = bitonic_sort(&mut a[0..n / 2]);
        let second = bitonic_sort(& mut a[(n / 2)..n - 1]);
        // let g = first.append(second.iter().clone().as_slice());
        // return bitonic_merge(g)
        return first;
    }
}

fn bitonic_merge<T>(a: &mut [T]) -> &mut [T] where T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd {
    bitonic_compare(a);
    let f = bitonic_merge(a);
    // let s = bitonic_merge(a);
    // let g: Vec<T> = f.append(s.iter().clone());
    // g
    return f;
}

fn bitonic_compare<T>(a: &mut [T]) where T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd {
    let distance = a.len() / 2;
    for i in 0..distance {
        if a[i] > a[i + distance]  {
            a.swap(i, i + distance);
        }
    }
}

*/