struct Fruit {
    name: String,
}

impl Fruit {
    fn print_fruit_name (&self) {
        println!("{}s are delicious", self.name);
    }

    fn return_self() -> Self {
        Fruit { name: "banana".to_string() }
    }
}

fn main() {
    let fruit = Fruit { name: "apple".to_string()};

    Fruit::print_fruit_name(&fruit);
    fruit.print_fruit_name();

    let another_fruit = Fruit::return_self();
    another_fruit.print_fruit_name();
}
