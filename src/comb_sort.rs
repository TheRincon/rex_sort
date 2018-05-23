use std::cmp::Ord;

pub fn comb_sort<T>(a: &mut [T])  where T: PartialOrd {
    let mut n = a.len();

    let mut sorted = false;

    while !sorted {
        n = n / 2;
        if n > 1 {
            sorted = false;
        } else {
            n = 1;
            sorted = true;
        }
        let mut i = 0;
        while i + n < a.len() {
            if a[i] > a[i + n] {
                a.swap(i, i + n);
                sorted = false;
            }
            i += 1;
        }
    }
}

