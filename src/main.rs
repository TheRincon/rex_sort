extern crate rand;

use rand::Rng;
use std::ops::Range;
use rand::thread_rng;

mod merge_sort;
mod tim_sort;
mod quick_sort;
mod bubble_sort;
mod insertion_sort;
mod heap_sort;
mod intro_sort;
mod shell_sort;
mod selection_sort;
mod odd_even_sort;
mod dutch_flag_sort;
mod flash_sort;
mod comb_sort;
mod cycle_sort;
mod bitonic_sort;
mod cocktail_sort;
mod min_max_sort;


fn main() {

    let mut y: Vec<u64> = vec![];

     for i in 0..1_000 {
        let j = rand::thread_rng().gen_range(0, 1_000);
        y.push(j);
    }
    let mut non: Vec<u32> = vec![1,2,3,4,3,31,2,4,3,2,6,45,4,4,4,3];
    // non.iter().map(|x| x.count());
    let mut f = vec![4.3434, 23.32323, 99.3243, 0.84728934, 00.78787, 0.0];
    let mut k = vec!["dfsdfi", "gdfgdfga", "ayrtrtr", "fsdd", "fdsfsdf", "dghfgd8787dfjskdfg", "5345345ghjsdfg", "\t"];
    merge_sort::merge_sort(&mut y);
    for x in y.iter() {
        print!("{} \n", x)
    }
}
