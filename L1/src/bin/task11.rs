// Дана последовательность температурных колебаний (для примера: -25.4, -27.0 13.0, 19.0, 15.5, 24.5, -21.0, 32.5).
//  Объединить данные значения в интервалы с шагом в 10 градусов. Последовательность в подмножноствах не важна.
// Пример: [-30,-20):{-25.0, -27.0, -21.0}, [10, 20):{13.0, 19.0, 15.5}, [20, 30): {24.5}, etc.

use std::collections::HashMap;

fn main() {
    let temperatures: Vec<f64> = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
    let mut temperature_map = HashMap::new();

    for temp in temperatures {
        let key = (temp as i32 / 10) * 10;
        temperature_map.entry(key).or_insert(Vec::new()).push(temp);
    }

    for (key, value) in temperature_map {
        if key < 0 {
            println!("[{}, {}): {:?}", key - 10, key, value);
        } else {
            println!("[{}, {}): {:?}", key, key + 10, value);
        }
    }
}
