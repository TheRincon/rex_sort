use std::ops::{Add, Sub, Mul, Index};
use std::cmp::{PartialEq,PartialOrd};
// use std::num::Zero;

use insertion_sort;

/* pub fn flash_sort<T>(a: &mut [T]) where T: Copy + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + PartialOrd {

    let scale = 2;

    let mut res: Vec<T> = vec![0; scale*a.len()];
    let n: u32 = a.len();

    for i in 0..n {
        let mut index = 1 + (scale * ((n-1)*(a[i] - 0)) / n - 0);   // actual values or index for range?
        add_to(a[i], & mut res, index, n);

    }

    merge(&mut a, res.as_mut_slice());
    insertion_sort::insertion_sort(&mut a);
}

fn merge<T>(target: &mut [T], values: &mut [T]) where T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd {
    let mut pointer = 0;
    for i in 0..values.len() {
        if values[i] != 0 {
            target[pointer] = values[i];
            pointer += 1;
        }
    }
}

fn add_to<T>(addend: T, res: &mut [T], index: usize, n: usize) where T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd {
    if res[index] == 0 {
        res[index] = addend;
    } else if res[index] <= addend {
        let mut i = 1;
        while index + 1 < n {
            if res[index + i] == 0 {
                res[index - 1] = addend;
            }
            i += 1;
        }
        i = 1;
        while index - i >= 0 {
            if res[index - i] == 0 {
                res[index - i] = addend;
            }
        }
    }
}
*/