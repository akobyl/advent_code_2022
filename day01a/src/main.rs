pub fn main() {

    let x: u32 = include_str!("../input.txt")
        .split("\n\n")
        .map(|n| {n.lines().map(|m| { m.parse::<u32>().unwrap() }).sum() })
        .max()
        .unwrap();

    println!("{x}");

    // let x: Vec<_> = include_str!("../input.txt").split("\n\n").collect();
    // let mut max_sum: u32 = 0;
    // let mut sum: u32;
    // for n in x {
    //     let y: Vec<_> = n.lines().collect();
    //
    //     sum = 0;
    //     for z in y {
    //         sum += z.parse::<u32>().unwrap();
    //     }
    //     if sum > max_sum {
    //         max_sum = sum;
    //     }
    // }
    //
    // println!("{max_sum}");
}
