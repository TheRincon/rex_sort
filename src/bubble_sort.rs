

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

#[cfg(test)]
mod test_bubble {

    use super::*;
    #[test]
    fn test_bubble_int() {
        let mut int_vec = vec![2, 4, 3, 1, 5, 7, 6, 8];
        bubble_sort(&mut int_vec);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], int_vec);
    }

    #[test]
    fn test_bubble_usize() {
        let mut usize_vec: Vec<usize> = vec![2, 4, 3, 1, 5, 7, 6, 8, 88238];
        bubble_sort(&mut usize_vec);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 88238], usize_vec);
    }

    #[test]
    fn test_bubble_very_large() {
        let mut large_vec:  Vec<i64> = vec![28467298374870234, 4462896788749623746, 3, 1, 5, 7, 6, 8, 846287687263478238];
        bubble_sort(&mut large_vec);
        assert_eq!(large_vec, vec![1, 3, 5, 6, 7, 8, 28467298374870234, 846287687263478238, 4462896788749623746]);
    }

    #[test]
    fn test_bubble_very_small() {
        let mut small_vec:  Vec<f64> = vec![0.28467298374870234, -4462896788749623746.0, 0.0000000000000003, 1.0, 5.4234, 0.07, 6.993423, 0.0000367468, 846287687263478238.0];
        bubble_sort(&mut small_vec);
        assert_eq!(small_vec, vec![-4462896788749624000.0, 0.0000000000000003, 0.0000367468, 0.07, 0.2846729837487023, 1.0, 5.4234, 6.993423, 846287687263478300.0]);
    }

    #[test]
    fn test_bubble_float() {
        let mut float_vec = vec![2.0, 4.0, 3.0, 1.0, 5.0, 7.0, 6.0, 8.0, 0.0];
        bubble_sort(&mut float_vec);
        assert_eq!(float_vec, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn test_bubble_string() {
        let mut string_vec = vec!["Help", "'Twas the night B4 Xmas", "fbajsb", "!@#!$@!783479823", "/??<.,.sd", ""];
        bubble_sort(&mut string_vec);
        assert_eq!(string_vec, vec!["", "!@#!$@!783479823", "'Twas the night B4 Xmas", "/??<.,.sd", "Help", "fbajsb"]);
    }
}
