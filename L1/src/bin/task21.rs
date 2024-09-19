// Разработать программу, которая перемножает, делит, складывает, вычитает две числовых переменных a,b, значение которых > 2^20.

fn main() {
    use std::ops::{Add, Sub, Mul, Div};
    let a: i128 = 2_i128.pow(30) + 1;
    let b: i128 = 2_i128.pow(31) - 1;

    println!("a = {}", a);
    println!("b = {}", b);

    // Addition
    let sum = a.add(b);
    println!("a + b = {}", sum);

    // Subtraction
    let difference = a.sub(b);
    println!("a - b = {}", difference);

    // Multiplication
    let product = a.mul(b);
    println!("a * b = {}", product);

    // Division
    let quotient = b.div(a);
    println!("a / b = {}", quotient);
}