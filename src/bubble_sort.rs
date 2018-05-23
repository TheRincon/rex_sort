use std::cmp::PartialOrd;

pub fn bubble_sort<T>(a: & mut [T]) where T: PartialOrd {
    let mut m = a.len();
    loop {
        let mut new = 0;
        for i in 1..m {
            if a[i-1] > a[i] {
                a.swap(i-1, i);
                new = i;
            }
        }
        m = new;
        if m == 0 { break; }
    }
}
