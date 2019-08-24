
struct Node {

}

pub fn burst_sort<T>(a: &mut [T]) where T: PartialOrd {
    let threshold = 8192;
    let alphabet = 256;
    let sub_bucket_start_size = 16;
    let sub_bucket_growth_factor = 2;
    let sub_bucket_threshold = 256;

}

