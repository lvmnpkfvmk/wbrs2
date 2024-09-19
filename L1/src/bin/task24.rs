// Разработать программу, которая проверяет, что все символы в строке уникальные (true — если уникальные, false etc).
// Функция проверки должна быть регистронезависимой.
// Например:
// abcd — true
// abCdefAaf — false
// aabcd — false

fn main() {
    let str1 = "abcd";
    let str2 = "abCdefAaf";
    let str3 = "aabcd";

    println!("{}", is_unique(str1));
    println!("{}", is_unique(str2));
    println!("{}", is_unique(str3));
}

fn is_unique(s: &str) -> bool {
    let mut seen = std::collections::HashSet::new();
    s.to_lowercase().chars().all(|c| seen.insert(c))
}