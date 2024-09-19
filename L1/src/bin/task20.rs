// Реализовать паттерн «адаптер» на любом примере.

trait Target {
    fn request(&self) -> String;
}

struct Adaptee {
    specific_request: String,
}

impl Adaptee {
    fn new(request: String) -> Self {
        Adaptee { specific_request: request }
    }

    fn specific_request(&self) -> String {
        self.specific_request.clone()
    }
}

struct Adapter {
    adaptee: Adaptee,
}

impl Adapter {
    fn new(adaptee: Adaptee) -> Self {
        Adapter { adaptee }
    }
}

impl Target for Adapter {
    fn request(&self) -> String {
        format!("Адаптированный запрос: {}", self.adaptee.specific_request())
    }
}

fn main() {
    let adaptee = Adaptee::new("Hello, world!".to_string());
    let adapter = Adapter::new(adaptee);

    println!("{}", adapter.request());
}
