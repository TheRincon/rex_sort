//use std::cmp::Ord;
//use std::cmp::min;
//use std::ops::{Add, Sub, Mul};
//use std::convert::TryFrom;
//
//pub fn flash_sort<T>(a: &mut [T]) where T: PartialOrd + Clone + Add + Sub {
//
//	let mut k = 0;
//	let mut a_min = a[0].clone();
//	let mut n_max = 0;
//	let n = a.len();
//	let m = (0.42 * n as f64) as usize;
//    let mut b = vec![0; m];
//	for i in 1..n {
//		if a[i] < a_min {
//			a_min = a[i].clone();
//		}
//		if a[i] > a[n_max] {
//			n_max = i;
//		}
//	}
//	if a_min == a[n_max].clone() {
//		return;
//	}
//	let c = (m - 1) / (usize::try_from(a[n_max].clone()).unwrap() - usize::try_from(a_min).unwrap());
//	for j in 0..n {
//		k = (c * (a[j] - anmin) ) as usize;
//        b[k] += 1;
//	}
//    for j in 1..m {
//        b[j] += b[j-1];
//    }
//
//    let mut hold = a[n_max].clone();
//    a[n_max] = a[0].clone();
//    let n_move = 0;
//    k = (m - 1);
//    let mut j = 0;
//    let mut flash = a[0].clone();
//
//    while n_move < (n - 1) {
//        while j > (b[k] - 1) {
//            j += 1;
//            k = (c * usize::try_from(a[j] - a_min).unwrap()) ;
//        }
//        flash = a[j].clone();
//        while j != b[k] {
//            k = (c * (flash - a_min)) as usize;
//            hold = a[b[k]-1].clone();
//            a[b[k]-1] = flash;
//            flash = hold;
//            b[k] -= 1;
//            n_move += 1;
//        }
//    }
//}
//
//#[cfg(test)]
//mod test_flash {
//
//    use super::*;
//    #[test]
//    fn test_flash_int() {
//        let mut int_vec = vec![2, 4, 3, 1, 5, 7, 6, 8];
//        flash_sort(&mut int_vec);
//        assert_eq!(int_vec, vec![1, 2, 3, 4, 5, 6, 7, 8]);
//    }
//
//    #[test]
//    fn test_flash_usize() {
//        let mut usize_vec: Vec<usize> = vec![2, 4, 3, 1, 5, 7, 6, 8, 88238];
//        flash_sort(&mut usize_vec);
//        assert_eq!(usize_vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 88238]);
//    }
//
//    #[test]
//    fn test_flash_very_large() {
//        let mut large_vec:  Vec<i64> = vec![28467298374870234, 4462896788749623746, 3, 1, 5, 7, 6, 8, 846287687263478238];
//        flash_sort(&mut large_vec);
//        assert_eq!(large_vec, vec![1, 3, 5, 6, 7, 8, 28467298374870234, 846287687263478238, 4462896788749623746]);
//    }
//
//    #[test]
//    fn test_flash_very_small() {
//        let mut small_vec:  Vec<f64> = vec![0.28467298374870234, -4462896788749623746.0, 0.0000000000000003, 1.0, 5.4234, 0.07, 6.993423, 0.0000367468, 846287687263478238.0];
//        flash_sort(&mut small_vec);
//        assert_eq!(vec![-4462896788749624000.0, 0.0000000000000003, 0.0000367468, 0.07, 0.2846729837487023, 1.0, 5.4234, 6.993423, 846287687263478300.0], small_vec);
//    }
//
//    #[test]
//    fn test_flash_float() {
//        let mut float_vec = vec![2.0, 4.0, 3.0, 1.0, 5.0, 7.0, 6.0, 8.0, 0.0];
//        flash_sort(&mut float_vec);
//        assert_eq!(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], float_vec);
//    }
//}
