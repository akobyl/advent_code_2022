use std::collections::BinaryHeap;

pub fn main() {
    // let k_count = 3;
    // let x: Vec<_> = include_str!("../../day01a/input.txt").split("\n\n").collect();
    //
    // let mut heap = BinaryHeap::new();
    // let mut sum: u32;
    // for n in x {
    //     let y: Vec<_> = n.lines().collect();
    //
    //     sum = 0;
    //     for z in y {
    //         sum += z.parse::<u32>().unwrap();
    //     }
    //     heap.push(sum);
    // }
    //
    // let mut top_k_sum = 0;
    // for _ in 0..k_count {
    //     top_k_sum += heap.pop().unwrap();
    // }
    // println!("{top_k_sum}");

    //-------------------------

    let x: Vec<u32> = include_str!("../../day01a/input.txt")
        .split("\n\n")
        .map(|n| {n.lines().map(|m| { m.parse::<u32>().unwrap() }).sum() })
        .collect();

    let mut heap = BinaryHeap::new();
    for n in x {
        heap.push(n);
    }
    let k_count = 3;
    let mut top_k_sum = 0;
    for _ in 0..k_count {
        top_k_sum += heap.pop().unwrap();
    }
    println!("{top_k_sum}");
}
