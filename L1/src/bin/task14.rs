// Подготовьте пример программы, которая в рантайме способна определить тип переменной, используйте std::any::*.
use std::any::*;

fn main() {
    let value = "Hello, world!";
    let type_id = value.type_id();

    println!("Type ID of value: {:?}", type_id);

    let is_str = type_id == std::any::TypeId::of::<str>();
    println!("{:?}", TypeId::of::<&str>());
    println!("{:?}", TypeId::of::<str>());
    println!("Is value a string? {:?}", is_str);
}
