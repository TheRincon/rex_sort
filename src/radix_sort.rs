use std::cmp::PartialOrd;

pub fn radix_sort<T: Ord>(a: &mut [T]) where T: PartialOrd {
    if a.len() < 1000 {
        let base = 10;
    } else { let base = 1000; }

    let mut buc = vec![0; base];

    for k in buc {
        v = *k;
    buc[(val & 0xFF) as usize] += 1;

    }
}