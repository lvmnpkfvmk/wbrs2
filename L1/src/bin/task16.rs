fn main() {
    let numbers = vec![1, 3, 4, 6, 8, 9, 11, 13, 14, 15, 17, 18, 19];

    for target in [13, 16] {
        match numbers.binary_search(&target) {
            Ok(index) => println!("Found {} at index {}", target, index),
            Err(index) => println!("{} not found, but could be at {}", target, index),
        }
    }
}
