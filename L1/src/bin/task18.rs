// Разработать программу, которая переворачивает подаваемую на ход строку (например: «главрыба — абырвалг»).
//  Символы могут быть unicode.

use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error read line");

    let output = input.chars().rev().collect::<String>();

    println!("Output: {}", output);
}
