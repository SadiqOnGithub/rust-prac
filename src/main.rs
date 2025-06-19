use std::collections::HashMap;

fn main() {
    let mut key_value = HashMap::new();
    key_value.insert("key1", 10);
    key_value.insert("key3", 34);

    key_value.entry("key1").or_insert(1);
    key_value.entry("key2").or_insert(1);

    let result = key_value.entry("key3").or_insert(1);
    dbg!(*result += 1);
    dbg!(result);
    println!("{:?}", key_value);
}
