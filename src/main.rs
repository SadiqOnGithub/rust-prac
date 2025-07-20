mod task;

use prac_rs::sum;
use task::my_task;

fn main() {
    let result = sum(2, 2);
    println!("The sum is: {}", result);
    // task::my_task();
    my_task();
}
