// Дана переменная int64. Разработать программу которая устанавливает i-й бит в 1 или 0.

fn set_bit(num: i64, i: u32, bit: bool) -> i64 {
    if bit {
        num | (1i64).checked_shl(i).unwrap_or(0)
    } else {
        num & !(1i64).checked_shl(i).unwrap_or(0)
    }
}
fn main() {
    let mut num = 0b10101010;
    println!("{:b}", set_bit(num, 2, true));
    println!("{:b}", set_bit(num, 1, false));
    println!("{:b}", set_bit(num, 64, true));
    println!("{:b}", set_bit(num, 64, false));
}
