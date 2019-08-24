
pub fn heapify<T>(a: &mut [T], count: usize) where T: PartialOrd {
    let mut start: usize = i_parent(count - 1);
    loop {
        if start == 0 { return; }
        sift_down(a, start, count - 1);
        start -= 1;
    }
}

fn i_child_left(i: usize) -> usize {
    2 * i + 1
}

fn i_parent(i: usize) -> usize {
    (i - 1) / 2
}

fn i_child_right(i: usize) -> usize {
    2 * i + 2
}


pub fn sift_down<T>(a: & mut [T], start: usize, end: usize) where T: PartialOrd {
    let mut root = start;
    loop {
        let mut child = i_child_left(root);
        if child > end {
            break;
        }
        if child < end && a[child] < a[child + 1] {
            child += 1;
        }
        if a[root] < a[child] {
            a.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}

pub fn heap_sort<T>(a: &mut [T]) where T: PartialOrd {
    let end: usize = a.len();
    for start in (0..end / 2).rev() {
        sift_down(a, start, end - 1);
    }
    for item in (1..a.len()).rev() {
        a.swap(item, 0);
        sift_down(a, 0, item - 1);
    }
}

#[cfg(test)]
mod test_heap {

    use super::*;
    #[test]
    fn test_heap_int() {
        let mut int_vec = vec![2, 4, 3, 1, 5, 7, 6, 8];
        heap_sort(&mut int_vec);
        assert_eq!(int_vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_heap_usize() {
        let mut usize_vec: Vec<usize> = vec![2, 4, 3, 1, 5, 7, 6, 8, 88238];
        heap_sort(&mut usize_vec);
        assert_eq!(usize_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 88238]);
    }

    #[test]
    fn test_heap_very_large() {
        let mut large_vec:  Vec<i64> = vec![28467298374870234, 4462896788749623746, 3, 1, 5, 7, 6, 8, 846287687263478238];
        heap_sort(&mut large_vec);
        assert_eq!(large_vec, vec![1, 3, 5, 6, 7, 8, 28467298374870234, 846287687263478238, 4462896788749623746]);
    }

    #[test]
    fn test_heap_very_small() {
        let mut small_vec:  Vec<f64> = vec![0.28467298374870234, -4462896788749623746.0, 0.0000000000000003, 1.0, 5.4234, 0.07, 6.993423, 0.0000367468, 846287687263478238.0];
        heap_sort(&mut small_vec);
        assert_eq!(small_vec, vec![-4462896788749624000.0, 0.0000000000000003, 0.0000367468, 0.07, 0.2846729837487023, 1.0, 5.4234, 6.993423, 846287687263478300.0]);
    }

    #[test]
    fn test_heap_float() {
        let mut float_vec = vec![2.0, 4.0, 3.0, 1.0, 5.0, 7.0, 6.0, 8.0, 0.0];
        heap_sort(&mut float_vec);
        assert_eq!(float_vec, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    }

    #[test]
    fn test_heap_string() {
        let mut string_vec = vec!["Help", "'Twas the night B4 Xmas", "fbajsb", "99999", "!@#!$@!783479823", "/??<.,.sd", ""];
        heap_sort(&mut string_vec);
        assert_eq!(string_vec, vec!["", "!@#!$@!783479823", "'Twas the night B4 Xmas", "/??<.,.sd", "99999", "Help", "fbajsb"]);
    }
}

