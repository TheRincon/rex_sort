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

#[cfg(test)]
mod test_shell {
    use super::*;

    #[test]
    fn test_shell_int() {
        let mut int_vec = vec![2, 4, 3, 1, 5, 7, 6, 8];
        shell_sort(&mut int_vec);
        assert_eq!(int_vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_shell_usize() {
        let mut usize_vec: Vec<usize> = vec![2, 4, 3, 1, 5, 7, 6, 8, 88238];
        shell_sort(&mut usize_vec);
        assert_eq!(usize_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 88238]);
    }

    #[test]
    fn test_shell_very_large() {
        let mut large_vec: Vec<i64> = vec![28467298374870234, 4462896788749623746, 3, 1, 5, 7, 6, 8, 846287687263478238];
        shell_sort(&mut large_vec);
        assert_eq!(large_vec, vec![1, 3, 5, 6, 7, 8, 28467298374870234, 846287687263478238, 4462896788749623746]);
    }

    #[test]
    fn test_shell_very_small() {
        let mut small_vec: Vec<f64> = vec![0.28467298374870234, -4462896788749623746.0, 0.0000000000000003, 1.0, 5.4234, 0.07, 6.993423, 0.0000367468, 846287687263478238.0];
        shell_sort(&mut small_vec);
        assert_eq!(small_vec, vec![-4462896788749624000.0, 0.0000000000000003, 0.0000367468, 0.07, 0.2846729837487023, 1.0, 5.4234, 6.993423, 846287687263478300.0]);
    }

    #[test]
    fn test_shell_float() {
        let mut float_vec = vec![2.0, 4.0, 3.0, 1.0, 5.0, 7.0, 6.0, 8.0, 0.0];
        shell_sort(&mut float_vec);
        assert_eq!(float_vec, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn test_shell_string() {
        let mut string_vec = vec!["Help", "'Twas the night B4 Xmas", "fbajsb", "99999", "!@#!$@!783479823", "/??<.,.sd", ""];
        shell_sort(&mut string_vec);
        assert_eq!(string_vec, vec!["", "!@#!$@!783479823", "'Twas the night B4 Xmas", "/??<.,.sd", "99999", "Help", "fbajsb"]);
    }
}