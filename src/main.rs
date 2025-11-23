fn main() {
    // immutable reference
    let x = 5;
    let r = &x;
    assert_eq!(5, *r);

    // mutable reference
    let mut y = 10;
    // {
    let mut_ref = &mut y;
    *mut_ref += 5;
    assert_eq!(&mut 15, mut_ref);
    println!("mut_ref: {}", mut_ref);
    println!("y: {}", y); // Now we can access y
    // }
    assert_eq!(15, y);
}
