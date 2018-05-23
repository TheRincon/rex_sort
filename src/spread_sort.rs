use std::cmp::PartialOrd;
use std::mem;

use merge_sort;

pub fn spread_sort_core() {

}

fn log_two_rough(i: usize) -> u8 {
    let mut res = 0u8;
    if i >= 0 {
        while (input >> res) && (res < mem::size_of::<usize>()) {
            res += 1;
        }
    } else {
        while (i >> res) < -1 && (res < mem::sizo_of::<usize>()) {
            res += 1;
        }
    }
    return res;
}

fn get_max_count(lr: usize, count: usize) -> usize {
    let log_size: usize = log_two_rough(count);
    let mut relative_width = ();
    if(mem::size_of::<usize>() <= relative_width) {
        relative_width = mem::size_of::<usize>() - 1;
    }
    return 1 << ((relative_width < ()))
}

fn spread_sort_bins<T>(a: &mut [T], ct: usize, bc: usize, max: &usize, min: &usize, ba: &mut [T], mc: usize) where T: PartialOrd {
    for u in 0..bc {
        let mut c = 0;
        if c < 2 {
            continue;
        }
        if c < mc {
            merge_sort::merge_sort(a);
        } else {
            spread_sort(a, ct);
        }
    }
}

fn spread_sort<T>(a: &mut [T], ct: usize) where T: PartialOrd {
    if ct < 2 {
        return
    }

    let mut max: usize = 0;
    let mut min: usize = 0;
    let bc = 10;
    let ba = spread_sort_core();
    if(!ba) {
        return;
    }
    spread_sort_bins(a, ct, bc, max, min, ba, mc);
}

fn spreadsort_rec<T>(a: *mut [T], count: usize) where T: PartialOrd {
    if(count < 2) { return };
    let mut max: usize, min: usize = 0;
    let bc: usize = 0;

    if(!ba) { return };
    spread_sort_bins(a, count, bc, max, min, ba, get_max_count())
}
