trait Action {
    fn say(&self);
}

struct Person {
    name: String,
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_say() {
        let person = Person {
            name: String::from("Alice"),
        };
        person.say();
    }
}
