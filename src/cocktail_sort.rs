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
mod tests {
    use super::*;
    #[test]
    fn tester() {
        let mut t = vec ! [2, 4, 3, 1, 5, 7, 6, 8];
        assert_eq ! ([1, 2, 3, 4, 5, 6, 7, 8], cocktail_sort(&mut t));
    }
}
