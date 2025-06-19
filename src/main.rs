fn main() {
    let mut i = 7;
    dbg!(change(&mut i));
    println!("{}", i);
}

fn change(i: &mut i32) -> i32 {
    *i = *i + 5;
    *i
}
