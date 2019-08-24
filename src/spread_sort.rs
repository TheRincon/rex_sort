//use std::cmp::PartialOrd;
//use std::mem;
//
//use merge_sort;
//
//#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
//struct Bin {
//    position: usize,
//    count: usize
//}
//
//#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
//struct BinArray<'a, T> {
//    array: &'a [T],
//    index: usize,
//    length: usize,
//}
//
//fn spread_sort_core<T>(a: &mut [T], ct: usize, bc: usize, max: &usize, min: &usize) -> Vec<Bin>  where T: PartialOrd {
//
//}
//
//fn log_two_rough(i: usize) -> usize {
//    let mut res = 0;
//    if i >= 0 {
//        while (i >> res) && (res < mem::size_of::<usize>()) {
//            res += 1;
//        }
//    } else {
//        while (i >> res as usize) < -1 && (res < mem::size_of::<usize>()) {
//            res += 1;
//        }
//    }
//    return res;
//}
//
//fn find_extremes<T>(a: &mut [T], count: usize) where T: PartialOrd + Clone {
//    let mut min = a[0].clone();
//    let mut max = a[0].clone();
//    for u in 0..count {
//        if a[u] > max {
//            max = a[u];
//        }
//        else if a[u] < min {
//            max = a[u];
//        }
//    }
//}
//
//fn get_max_count(lr: usize, count: usize) -> usize {
//
//    let max_splits = 11;
//    let log_mean_bin_size = 2;
//    let log_min_split_count = 9;
//    let log_const = 4;
//    let log_size = log_two_rough(count);
//    let mut relative_width = if (log_const * lr) / (log_size) > max_splits { max_splits } else { log_size };
//
//    if mem::size_of::<usize>() <= relative_width {
//        relative_width = mem::size_of::<usize>() - 1;
//    }
//
//    let shifter = if relative_width < (log_mean_bin_size + log_min_split_count) {
//        log_mean_bin_size + log_min_split_count
//    } else {
//        relative_width
//    };
//
//    return 1 << shifter;
//}
//
//pub fn spread_sort_bins<T>(ba: &mut [T], ct: usize, bin_count: usize, max: &usize, min: &usize, bin_array: &mut Vec<Bin>, max_count: usize) where T: PartialOrd {
//    for i in 0..bin_count {
//        let mut c = (bin_array[i].position - ba.index) - bin_array[i].count;
//        if c < 2 {
//            continue;
//        }
//        if c < max_count {
//            merge_sort::merge_sort(a);
//        } else {
//            spread_sort(a);
//        }
//    }
//}
//
////pub fn spread_sort<T>(a: &mut [T]) where T: PartialOrd {
////    let ct = a.len();
////    if ct < 2 {
////        return
////    }
////    let mut max: usize = 0;
////    let mut min: usize = 0;
////    let bc = 10;
////    let bin_arraya = spread_sort_core();
////    if(!ba) {
////        return;
////    }
////    spread_sort_bins(a, ct, bc, &max, &min, ba, mc);
////}
//
//fn spread_sort<T>(a: &mut [T]) where T: PartialOrd + Clone {
//
//    let count: usize = a.len();
//    if count < 2 {
//        return;
//    }
//    let mut max: usize = 0;
//    let mut min: usize = 0;
//    let bin_counts: usize = 0;
//    let mut ba = BinArray { array: a, index: 0, length: a.len() };
//    let mut bins_array = spread_sort_core(&mut ba, count, bin_counts, &max, &min);
//    // if !bins_array { return };
//    spread_sort_bins(ba, count, bin_counts, &max, &min, &mut bins_array, get_max_count(log_two_rough(&max - &min) as usize, count))
//}
//
//#[cfg(test)]
//mod test_spread {
//    use super::*;
//
//    #[test]
//    fn test_shell_int() {
//        let mut int_vec = vec![2, 4, 3, 1, 5, 7, 6, 8];
//        spread_sort(&mut int_vec);
//        assert_eq!(int_vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
//    }
//
//    #[test]
//    fn test_spread_usize() {
//        let mut usize_vec: Vec<usize> = vec![2, 4, 3, 1, 5, 7, 6, 8, 88238];
//        spread_sort(&mut usize_vec);
//        assert_eq!(usize_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 88238]);
//    }
//
//    #[test]
//    fn test_spread_very_large() {
//        let mut large_vec: Vec<i64> = vec![28467298374870234, 4462896788749623746, 3, 1, 5, 7, 6, 8, 846287687263478238];
//        spread_sort(&mut large_vec);
//        assert_eq!(large_vec, vec![1, 3, 5, 6, 7, 8, 28467298374870234, 846287687263478238, 4462896788749623746]);
//    }
//
//    #[test]
//    fn test_spread_very_small() {
//        let mut small_vec: Vec<f64> = vec![0.28467298374870234, -4462896788749623746.0, 0.0000000000000003, 1.0, 5.4234, 0.07, 6.993423, 0.0000367468, 846287687263478238.0];
//        spread_sort(&mut small_vec);
//        assert_eq!(small_vec, vec![-4462896788749624000.0, 0.0000000000000003, 0.0000367468, 0.07, 0.2846729837487023, 1.0, 5.4234, 6.993423, 846287687263478300.0]);
//    }
//
//    #[test]
//    fn test_spread_float() {
//        let mut float_vec = vec![2.0, 4.0, 3.0, 1.0, 5.0, 7.0, 6.0, 8.0, 0.0];
//        spread_sort(&mut float_vec);
//        assert_eq!(float_vec, vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
//    }
//
//    #[test]
//    fn test_spread_string() {
//        let mut string_vec = vec!["Help", "'Twas the night B4 Xmas", "fbajsb", "99999", "!@#!$@!783479823", "/??<.,.sd", ""];
//        spread_sort(&mut string_vec);
//        assert_eq!(string_vec, vec!["", "!@#!$@!783479823", "'Twas the night B4 Xmas", "/??<.,.sd", "99999", "Help", "fbajsb"]);
//    }
//}
