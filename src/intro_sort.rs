use std::cmp::Ord;

use heap_sort;
use quick_sort;

fn sort<T>(a: &mut [T], max_depth: usize) where T: PartialOrd {
    let n = a.len();
    if n <= 1 {
        return;
    } else if max_depth == 0 {
        heap_sort::heap_sort(a);
    } else {
        let mut p = quick_sort::partition(a, 0, n-1);
        sort(& mut a[0..p], max_depth - 1);
        sort(&mut a[p+1..n], max_depth - 1);
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
    let max_depth: usize = a.len() * 2;
    sort(a, max_depth);
}

#[cfg(test)]
mod test_intro {

    use super::*;
    #[test]
    fn test_intro_int() {
        let mut int_vec = vec![2, 4, 3, 1, 5, 7, 6, 8];
        intro_sort(&mut int_vec);
        assert_eq!(int_vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_intro_usize() {
        let mut usize_vec: Vec<usize> = vec![2, 4, 3, 1, 5, 7, 6, 8, 88238];
        intro_sort(&mut usize_vec);
        assert_eq!(usize_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 88238]);
    }

    #[test]
    fn test_intro_very_large() {
        let mut large_vec:  Vec<i64> = vec![28467298374870234, 4462896788749623746, 3, 1, 5, 7, 6, 8, 846287687263478238];
        intro_sort(&mut large_vec);
        assert_eq!(large_vec, vec![1, 3, 5, 6, 7, 8, 28467298374870234, 846287687263478238, 4462896788749623746]);
    }

    #[test]
    fn test_intro_very_small() {
        let mut small_vec:  Vec<f64> = vec![0.28467298374870234, -4462896788749623746.0, 0.0000000000000003, 1.0, 5.4234, 0.07, 6.993423, 0.0000367468, 846287687263478238.0];
        intro_sort(&mut small_vec);
        assert_eq!(small_vec, vec![-4462896788749624000.0, 0.0000000000000003, 0.0000367468, 0.07, 0.2846729837487023, 1.0, 5.4234, 6.993423, 846287687263478300.0]);
    }

    #[test]
    fn test_intro_float() {
        let mut float_vec = vec![2.0, 4.0, 3.0, 1.0, 5.0, 7.0, 6.0, 8.0, 0.0];
        intro_sort(&mut float_vec);
        assert_eq!(float_vec, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn test_intro_string() {
        let mut string_vec = vec!["Help", "'Twas the night B4 Xmas", "fbajsb", "99999", "!@#!$@!783479823", "/??<.,.sd", ""];
        intro_sort(&mut string_vec);
        assert_eq!(string_vec, vec!["", "!@#!$@!783479823", "'Twas the night B4 Xmas", "/??<.,.sd", "99999", "Help", "fbajsb"]);
    }
}