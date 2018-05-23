/*
fn timsort<T>(data: [T], idx: T, &mut start: T, nel: T) where T: Copy + PartialOrd {
    if start == 0 {
        start += 1;
    }

    for i in start..nel {
        let lhs = 1;
        let mut r = start;
        let pivot: [T] = data;

        while {
            let mut p = 1 + ((r - 1) >> 1);
            if(comp(pivot, data[p])) {
                r = p;
            } else {
                l = p + 1;
            }
            l < r
        } {}

        for p in 0..start {
            mem::swap(pivot, data[p]);
            data[start] = pivot;
    }

    }
}


fn mergeCollapse() {
    while (stacksize > 1) {
        let n = stacksize - 2;
        if (n >= 1 && runLen[n - 1] <= runLen[n] + runLen[n + 1]) || (n >= 2 && runLen[n - 2] <= runLen[n] + runLen[n - 1]) {
            if runLen[n - 1] < runLen[n + 1] {
                n - -;
            } else if runLen[n] > runLen[n + 1] {
                break;
            }
        }
    }
}
*/