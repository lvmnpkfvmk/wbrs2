// Разработать программу нахождения расстояния между двумя точками, которые представлены в виде структуры Point с инкапсулированными параметрами x,y и конструктором.

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

fn distance(point1: Point, point2: Point) -> f64 {
    let dx = point1.x - point2.x;
    let dy = point1.y - point2.y;
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    let point1 = Point::new(3.0, 4.0);
    let point2 = Point::new(0.0, 0.0);
    let dist = distance(point1, point2);
    println!("Distance between points: {}", dist);
}