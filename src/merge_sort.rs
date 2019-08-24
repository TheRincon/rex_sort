
pub fn merge<T>(a: &mut [T], b: &mut [T], start: usize, mid: usize, end: usize) where T: PartialOrd + Clone {
    let mut one = start;
    let mut two = mid;
    for i in start..end {
        if (one < mid) && (two >= end || a[one] <= a[two]) {
            b[i] = a[one].clone();
            one += 1;
        } else {
            b[i] = a[two].clone();
            two += 1;
        }
    }
}

pub fn copy_vec<T>(a: &mut [T], b: &mut [T], start: usize, end: usize) where T: PartialOrd + Clone {
    for i in start..end { a[i] = b[i].clone() }
}

pub fn merge_split<T>(a: &mut [T], b: &mut [T], start: usize, end: usize) where T: PartialOrd + Clone {
    let e = end as i64;
    let s = start as i64;
    if (e - s) <= 1 {
        return;
    } else {
        let mid = ((e + s) / 2 ) as usize;
        merge_split(a, b, start, mid);
        merge_split(a, b, mid, end);
        merge(a, b, start, mid, end);
        copy_vec(a, b, start, end);
    }
}

pub fn merge_sort<T>(a: &mut [T])  where T: PartialOrd + Clone {

    let size = a.len();
    let mut b: Vec<T> = Vec::with_capacity(size);
    unsafe { b.set_len(size); }
    merge_split(a, &mut b, 0, size);
}

#[cfg(test)]
mod test_merge {

    use super::*;
    use merge_sort::merge_sort;

    #[test]
    fn test_merge_int() {
        let mut int_vec = vec![2, 4, 3, 1, 5, 7, 6, 8];
        merge_sort(&mut int_vec);
        assert_eq!(int_vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_merge_usize() {
        let mut usize_vec: Vec<usize> = vec![2, 4, 3, 1, 5, 7, 6, 8, 88238];
        merge_sort(&mut usize_vec);
        assert_eq!(usize_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 88238]);
    }

    #[test]
    fn test_merge_very_large() {
        let mut large_vec:  Vec<i64> = vec![28467298374870234, 4462896788749623746, 3, 1, 5, 7, 6, 8, 846287687263478238];
        merge_sort(&mut large_vec);
        assert_eq!(large_vec, vec![1, 3, 5, 6, 7, 8, 28467298374870234, 846287687263478238, 4462896788749623746]);
    }

    #[test]
    fn test_merge_very_small() {
        let mut small_vec:  Vec<f64> = vec![0.28467298374870234, -4462896788749623746.0, 0.0000000000000003, 1.0, 5.4234, 0.07, 6.993423, 0.0000367468, 846287687263478238.0];
        merge_sort(&mut small_vec);
        assert_eq!(small_vec, vec![-4462896788749624000.0, 0.0000000000000003, 0.0000367468, 0.07, 0.2846729837487023, 1.0, 5.4234, 6.993423, 846287687263478300.0]);
    }

    #[test]
    fn test_merge_float() {
        let mut float_vec = vec![2.0, 4.0, 3.0, 1.0, 5.0, 7.0, 6.0, 8.0, 0.0];
        merge_sort(&mut float_vec);
        assert_eq!(float_vec, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn test_merge_string() {
        let mut string_vec = vec!["Help", "'Twas the night B4 Xmas", "fbajsb", "99999", "!@#!$@!783479823", "/??<.,.sd", ""];
        merge_sort(&mut string_vec);
        assert_eq!(string_vec, vec!["", "!@#!$@!783479823", "'Twas the night B4 Xmas", "/??<.,.sd", "99999", "Help", "fbajsb"]);
    }
}
