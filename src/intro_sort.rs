use std::cmp::Ord;

use heap_sort;
use quick_sort;

fn sort<T>(a: &mut [T], maxdepth: usize) where T: PartialOrd {
    let n = a.len();
    if n <= 1 {
        return;
    } else if maxdepth == 0 {
        heap_sort::heap_sort(a);
    } else {
        let mut p = quick_sort::partition(a, 0, n-1);
        sort(& mut a[0..p], maxdepth - 1);
        sort(&mut a[p+1..n], maxdepth - 1);
    }

}

pub fn simple_log_ten(num: usize) -> usize {
    let mut n = num;
    let mut i = 0;
    while n > 0 {
        n /= 10;
        i += 1;
    }
    return i;
}

pub fn intro_sort<T>(a: &mut [T]) where T: PartialOrd {
    let maxdepth: usize = a.len() * 2;
    sort(a, maxdepth);
}