fn main() {
    // let mystery = vec![1, 2, 3].iter().map(|x| x * 2);
    // let _: () = mystery; // Compiler will tell you the real type!

    // Can't print iterators directly
    let iter = vec![1, 2, 3]
        .into_iter()
        .map(|x| x * 2)
        .filter(|x| *x > 3)
        .skip(1)
        // .next()
        // .unwrap()
        ;
    println!("{:?}", iter); // ✗ Error!

    // Collect first
    // let collected: Vec<i32> = iter.collect();
    // println!("{:?}", collected); // ✓ Works! [2, 4, 6]
}
