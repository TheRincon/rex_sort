
pub fn heapify<T>(a: &mut [T], count: usize) where T: PartialOrd {
    let mut start: usize = i_parent(count);
    while start > 0 {
        start -= 1;
        sift_down(a, start, count);
    }
}

fn i_child_left(i: usize) -> usize {
    2*i + 1
}

fn i_parent(i: usize) -> usize {
    (i-1) / 2
}

fn i_child_right(i: usize) -> usize {
    2*i + 2
}


pub fn sift_down<T>(a: & mut [T], start: usize, end: usize) where T: PartialOrd {
    let mut root = start;
    while i_child_left(root) <= end {
        let mut child = i_child_left(root);
        let mut swap = root;
        if a[swap] < a[child] {
            swap = child;
        }
        if child + 1 <= end && a[swap] < a[child + 1] {
            swap = child + 1;
        }
        if swap == root {
            return
        } else {
            a.swap(root, swap);
            root = swap
        }

    }
}

pub fn heap_sort<T>(a: &mut [T]) where T: PartialOrd {
    let count: usize = a.len() -1;
    heapify(a, count);
    let mut end = count;
    while end > 0 {
        a.swap(end, 0);
        end -= 1;
        sift_down(a, 0, end);
    }
}

