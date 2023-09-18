struct Person {
    name: String,
    age: u8,
    class: u8,
    address: String,
}

fn main() {
    let person1 = Person {
        name: "Yogendra Rana".to_string(), // .to_string() is used to convert &str to String
        age: 23,
        class: 12,
        address: String::from("Pokhara, Nepal"), // String::from() is used to convert &str to String
    };

    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
    println!("Class: {}", person1.class);
    println!("Address: {}", person1.address);
}
