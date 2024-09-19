//Разработать программу, которая переворачивает слова в строке.
// Пример: «snow dog sun — sun dog snow»..

fn main() {
    let input = "snow dog sun";
    let words = input.split_whitespace().collect::<Vec<&str>>();
    let reversed_words = words.into_iter().rev().collect::<Vec<&str>>();
    let output = reversed_words.join(" ");
    assert_eq!(output, "sun dog snow");
    println!("Output: {}", output);
}
