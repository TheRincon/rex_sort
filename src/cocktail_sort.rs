pub fn cocktail_sort<T>(a: &mut [T]) where T: PartialOrd {
    let mut sorted: bool = false;
    let mut first: usize = 0;
    let mut last: usize = a.len() - 1;
    while !sorted {
        let mut changed: bool = false;
        for i in 0..(a.len() - 1) {
            if a[i] > a[i + 1] {
                a.swap(i, i + 1);
                changed = true;
            }
        }
        if changed {
            last -= 1;
        } else {
            sorted = true;
        }
        if !sorted {
            changed = false;
            for k in (1..(a.len())).rev() {
                if a[k] < a[k - 1] {
                    a.swap(k, k - 1);
                    changed = true;
                }
            }
            if changed {
                first += 1;
            } else {
                sorted = true;
            }
        }
    }
}

#[cfg(test)]
mod test_cocktail {

    use super::*;
    #[test]
    fn test_cocktail_int() {
        let mut int_vec = vec![2, 4, 3, 1, 5, 7, 6, 8];
        cocktail_sort(&mut int_vec);
        assert_eq!(int_vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_cocktail_usize() {
        let mut usize_vec: Vec<usize> = vec![2, 4, 3, 1, 5, 7, 6, 8, 88238];
        cocktail_sort(&mut usize_vec);
        assert_eq!(usize_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 88238]);
    }

    #[test]
    fn test_cocktail_very_large() {
        let mut large_vec:  Vec<i64> = vec![28467298374870234, 4462896788749623746, 3, 1, 5, 7, 6, 8, 846287687263478238];
        cocktail_sort(&mut large_vec);
        assert_eq!(large_vec, vec![1, 3, 5, 6, 7, 8, 28467298374870234, 846287687263478238, 4462896788749623746]);
    }

    #[test]
    fn test_cocktail_very_small() {
        let mut small_vec:  Vec<f64> = vec![0.28467298374870234, -4462896788749623746.0, 0.0000000000000003, 1.0, 5.4234, 0.07, 6.993423, 0.0000367468, 846287687263478238.0];
        cocktail_sort(&mut small_vec);
        assert_eq!(small_vec, vec![-4462896788749624000.0, 0.0000000000000003, 0.0000367468, 0.07, 0.2846729837487023, 1.0, 5.4234, 6.993423, 846287687263478300.0]);
    }

    #[test]
    fn test_cocktail_float() {
        let mut float_vec = vec![2.0, 4.0, 3.0, 1.0, 5.0, 7.0, 6.0, 8.0, 0.0];
        cocktail_sort(&mut float_vec);
        assert_eq!(float_vec, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn test_cocktail_string() {
        let mut string_vec = vec!["Help", "'Twas the night B4 Xmas", "fbajsb", "!@#!$@!783479823", "/??<.,.sd", ""];
        cocktail_sort(&mut string_vec);
        assert_eq!(string_vec, vec!["", "!@#!$@!783479823", "'Twas the night B4 Xmas", "/??<.,.sd", "Help", "fbajsb"]);
    }
}
