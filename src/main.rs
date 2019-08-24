
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
mod flash_sort2;
mod spread_sort;


fn main() {

//    let mut a: Vec<u64> = vec![];
//     for i in 0..1_000 {
//        let j = rand::thread_rng().gen_range(0, 1_000);
//        a.push(j);
//    }
//    let mut b = a.clone();
//    let mut c = a.clone();
//    let mut d = a.clone();
//    let mut e = a.clone();
//    let mut f = a.clone();
//    let mut g = a.clone();
//    let mut h = a.clone();
//    let mut j = a.clone();
//    let mut k = a.clone();
//    let mut l = a.clone();
//    let mut m = a.clone();
//    let mut n = a.clone();
//    let mut o = a.clone();
//    let mut p = a.clone();
//    let mut q = a.clone();
//    let mut r = a.clone();
//    let mut s = a.clone();
//    let mut t = a.clone();
//    let mut u = a.clone();
//    let mut v = a.clone();
//    let mut w = a.clone();
//    let mut x = a.clone();
//    let mut y = a.clone();
//    let mut z = a.clone();
//    let mut intss: Vec<i32> = vec![1,2,3,4,3,31,2,4,3,2,6,45,4,4,4,3];
//    // non.iter().map(|x| x.count());
//    let mut flo = vec![4.3434, 23.32323, 99.3243, 0.84728934, 00.78787, 0.0];
//    let mut strr = vec!["dfsdfi", "gdfgdfga", "ayrtrtr", "fsdd", "fdsfsdf", "dghfgd8787dfjskdfg", "5345345ghjsdfg", "\tkk"];
//    merge_sort::merge_sort(&mut a);
//    quick_sort::quick_sort(&mut b);
//    cycle_sort::cycle_sort(&mut c);
//    intro_sort::intro_sort(&mut e);
//    selection_sort::selection_sort(&mut f);
//    heap_sort::heap_sort(&mut g);
//    insertion_sort::insertion_sort(&mut h);
//    comb_sort::comb_sort(&mut j);
//    odd_even_sort::odd_even_sort(&mut k);
//    bubble_sort::bubble_sort(&mut l);
//    shell_sort::shell_sort(&mut m);
//    assert_eq!(a,b);
//    assert_eq!(c,b);
//    assert_eq!(e,f);
//    assert_eq!(g,h);
//    assert_eq!(j,k);
//    assert_eq!(l,m);
//    assert_eq!(a,m);
//    for x in c.iter() {
//       print!("{} \n", x)
//    }
    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
    struct Person {
        name: String,
        age: u32
    }

    impl Person {
        pub fn new(name: String, age: u32) -> Self {
            Person {
                name,
                age
            }
        }
    }

    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    merge_sort::merge_sort(&mut people);

    println!("{:?}", people);

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]);
}
