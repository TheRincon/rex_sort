use std::mem;
use std::ops::{Add, Sub};
use std::cmp::Ord;

pub fn quick_sort<T>(a: &mut [T]) where T: PartialOrd {
    let hi = a.len() - 1;
    quick_sort_real(a, 0, hi);
}

fn quick_sort_real<T>(a: &mut [T], lo: usize, hi: usize) where T: PartialOrd {
    if lo < hi {
        let p = partition(a, lo, hi);
        // u32 triggers a panic when p == 0, i.e. 0usize - 1; this modification stops it.
        if p == 0 {
            quick_sort_real(a, lo, p);
        } else {
            quick_sort_real(a, lo, p - 1);
        }
        quick_sort_real(a, p + 1, hi);
    }
}

pub fn partition<T>(a: &mut [T], lo: usize, hi: usize) -> usize where T: PartialOrd {
    let mut i = lo;
    let mut k = hi;
    for j in lo..hi {
        if a[j] <= a[hi] {
            a.swap(i, j);
            i += 1;
        }
    }
    if a[hi] <= a[i] {
        a.swap(k, i);
    }
    return i;
}

#[cfg(test)]
mod test_quick {

    use super::*;
    #[test]
    fn test_quick_int() {
        let mut int_vec = vec![2, 4, 3, 9, 1, 5, 7, 6, 8];
        quick_sort(&mut int_vec);
        assert_eq!(int_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_quick_usize() {
        let mut usize_vec: Vec<usize> = vec![2, 4, 3, 1, 5, 7, 6, 8, 88238];
        quick_sort(&mut usize_vec);
        assert_eq!(usize_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 88238]);
    }

    #[test]
    fn test_quick_very_large() {
        let mut large_vec:  Vec<i64> = vec![28467298374870234, 4462896788749623746, 3, 1, 5, 7, 6, 8, 846287687263478238];
        quick_sort(&mut large_vec);
        assert_eq!(large_vec, vec![1, 3, 5, 6, 7, 8, 28467298374870234, 846287687263478238, 4462896788749623746]);
    }

    #[test]
    fn test_quick_very_small() {
        let mut small_vec:  Vec<f64> = vec![0.28467298374870234, -4462896788749623746.0, 0.0000000000000003, 1.0, 5.4234, 0.07, 6.993423, 0.0000367468, 846287687263478238.0];
        quick_sort(&mut small_vec);
        assert_eq!(small_vec, vec![-4462896788749624000.0, 0.0000000000000003, 0.0000367468, 0.07, 0.2846729837487023, 1.0, 5.4234, 6.993423, 846287687263478300.0]);
    }

    #[test]
    fn test_quick_float() {
        let mut float_vec = vec![2.0, 4.0, 3.0, 1.0, 5.0, 7.0, 6.0, 8.0, 0.0];
        quick_sort(&mut float_vec);
        assert_eq!(float_vec, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn test_quick_string() {
        let mut string_vec = vec!["Help", "'Twas the night B4 Xmas", "fbajsb", "99999", "!@#!$@!783479823", "/??<.,.sd", ""];
        quick_sort(&mut string_vec);
        assert_eq!(string_vec, vec!["", "!@#!$@!783479823", "'Twas the night B4 Xmas", "/??<.,.sd", "99999", "Help", "fbajsb"]);
    }

    #[test]
    fn test_quick_many_ints() {
        let mut int_vec = vec![2, 4, 3, 9, 1, 5, 7, 6, 8, 0, 11, 12, 14, 19, 18, 67, 90, 743, 43, 12, 1100, 6464, 1, 3, 4, 5, 6, 9, 1, 3, 2, 77];
        quick_sort(&mut int_vec);
        assert_eq!(int_vec, vec![0, 1, 1, 1, 2, 2, 3, 3, 3, 4, 4, 5, 5, 6, 6, 7, 8, 9, 9, 11, 12, 12, 14, 18, 19, 43, 67, 77, 90, 743, 1100, 6464]);
    }
}
