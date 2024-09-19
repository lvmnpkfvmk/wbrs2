// Реализовать пересечение двух неупорядоченных множеств.
fn main() {
    let set_1 = vec![1, 2, 3, 4];
    let set_2 = vec![2, 4, 6, 8];
    let intersection: Vec<i32> = set_1
        .iter()
        .filter(|&x| set_2.contains(x))
        .cloned()
        .collect();

    println!("{:?}", intersection); // [2, 4]
}
