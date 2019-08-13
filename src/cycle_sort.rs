use std::cmp::PartialOrd;

// Possible improvement.

// When the array contains only duplicates of a relatively small number of items,
// a constant-time perfect hash function can greatly speed up finding where to put an item1,
// turning the sort from Θ(n2) time to Θ(n + k) time, where k is the total number of hashes.
// The array ends up sorted in the order of the hashes, so choosing a hash function that gives
// you the right ordering is important.

pub fn cycle_sort<T>(a: &mut [T]) where T: PartialOrd + Clone {
    let mut writes = 0;
    let n = a.len();

    for i in 0..n {
        let mut itemm: T = a[i].clone();

        let mut pos = i;
        for k in i + 1..n {
            if a[k] < itemm {
                pos += 1;
            }
        }
        if pos == i {
            continue;
        }
        while itemm == a[pos] {
            pos += 1;
        }
        let mut tmp = a[pos].clone();
        a[pos] = itemm;
        itemm = tmp;
        writes += 1;

        while pos != i {
            pos = i;
            for l in i + 1..a.len() {
                if a[l] < itemm {
                    pos += 1;
                }
            }
            while itemm == a[pos] {
                pos += 1;
            }
            tmp = a[pos].clone();
            a[pos] = itemm;
            itemm = tmp;
            writes += 1;
        }
    }
}

#[cfg(test)]
mod test_cycle {

    use super::*;
    #[test]
    fn test_cycle_int() {
        let mut int_vec = vec![2, 4, 3, 1, 5, 7, 6, 8];
        cycle_sort(&mut int_vec);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], int_vec);
    }

    #[test]
    fn test_cycle_usize() {
        let mut usize_vec: Vec<usize> = vec![2, 4, 3, 1, 5, 7, 6, 8, 88238];
        cycle_sort(&mut usize_vec);
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 88238], usize_vec);
    }

    #[test]
    fn test_cycle_very_large() {
        let mut large_vec:  Vec<i64> = vec![28467298374870234, 4462896788749623746, 3, 1, 5, 7, 6, 8, 846287687263478238];
        cycle_sort(&mut large_vec);
        assert_eq!(vec![1, 3, 5, 6, 7, 8, 28467298374870234, 846287687263478238, 4462896788749623746], large_vec);
    }

    #[test]
    fn test_cycle_very_small() {
        let mut small_vec:  Vec<f64> = vec![0.28467298374870234, -4462896788749623746.0, 0.0000000000000003, 1.0, 5.4234, 0.07, 6.993423, 0.0000367468, 846287687263478238.0];
        cycle_sort(&mut small_vec);
        assert_eq!(vec![-4462896788749624000.0, 0.0000000000000003, 0.0000367468, 0.07, 0.2846729837487023, 1.0, 5.4234, 6.993423, 846287687263478300.0], small_vec);
    }

    #[test]
    fn test_cycle_float() {
        let mut float_vec = vec![2.0, 4.0, 3.0, 1.0, 5.0, 7.0, 6.0, 8.0, 0.0];
        cycle_sort(&mut float_vec);
        assert_eq!(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], float_vec);
    }

    #[test]
    fn test_cycle_string() {
        let mut string_vec = vec!["Help", "'Twas the night B4 Xmas", "fbajsb", "!@#!$@!783479823", "/??<.,.sd", ""];
        cycle_sort(&mut string_vec);
        assert_eq!(vec!["", "!@#!$@!783479823", "'Twas the night B4 Xmas", "/??<.,.sd", "Help", "fbajsb"], string_vec);
    }
}