// Реализовать быструю сортировку массива (quicksort) встроенными методами языка.

fn main() {
    // Создаем вектор с несортированными числами
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];

    println!("Input {:?}", numbers);

    numbers.sort();

    println!("Output {:?}", numbers);
}
