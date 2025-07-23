fn main() {
    let doubled = (1..=3).map(|x| x * 2); // lazy
    dbg!(doubled); // won’t show results

    // println!("{doubled:q}");
    println!("Hello, world!");
}
