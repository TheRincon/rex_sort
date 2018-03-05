pub fn min_max_sort<T>(a: &mut [T]) where T: Ord {
    let mut last: usize = a.len() - 1;
    let mut first: usize = 0;
    while first < last {
        let mut started = false;
        let mut min: usize = 0;
        let mut max: usize = 0;
        for i in 0..(a.len()) {
            if !started {
                min = i;
                max = i;
                started = true;
            } else {
                if a[i] < a[min] {
                    min = i;
                } else if a[i] > a[max] {
                    max = i;
                }
            }
        }
        if (a[max] == a[first]) && (a[min] == a[last]) {
            a.swap(max, min);
        } else if a[max] == a[first] {
            a.swap(max, last);
            a.swap(min, first);
        } else {
            a.swap(min, first);
            a.swap(max, last);
        }
        println!("{:?} first here", first);
        println!("{:?} last here", last);
        first += 1;
        last -= 1;
    }
}