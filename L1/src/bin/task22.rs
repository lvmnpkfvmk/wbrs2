// Удалить i-ый элемент из вектора.

fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let i = 2; // Индекс элемента для удаления (0-based)

    if i < vec.len() {
        vec.remove(i);
        println!("{:?}", vec);
    } else {
        println!("Out of bounds");
    }

}
