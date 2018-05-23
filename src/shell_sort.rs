use std::cmp::Ord;
use quick_sort;

pub fn shell_sort2<T>(a: &mut [T]) where T: Ord {
    if a.len() > 900 {
        let gaps: Vec<usize> = vec![701, 301, 132, 57, 23, 10, 4, 1];
        for gap in &gaps {
            for i in *gap..a.len() {
                let tmp = i;
                let mut j = i;
                while (j > *gap - 1) && (a[j - gap] >= a[i]) {
                    a.swap(j, j - gap);
                    j -= *gap;
                }
                a.swap(j, tmp);
            }
        }
    } else {
        let n = a.len();
        quick_sort::quick_sort(a);
    }
}

pub fn shell_sort<T>(a: &mut [T]) where T: PartialOrd + Clone {
    let mut gap = a.len() / 2;
    while gap > 0 {
        for i in gap..a.len() {
            let mut tmp: T = a[i].clone();
            let mut j = i;
            while j >= gap && a[j - gap] > tmp {
                let y:T = a[j - gap].clone();
                a[j] = y;
                j = j - gap;
            }
            a[j] = tmp;
        }
        gap = gap / 2;
    }
}