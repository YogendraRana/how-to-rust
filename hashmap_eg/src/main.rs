use std::collections::HashMap;

fn main() {
    // create a new hashmap
    let mut result = HashMap::new();

    // insert key-value pairs into the hashmap
    result.insert("Maths", 98);
    result.insert("Science", 95);
    result.insert("English", 90);
    result.insert("Nepali", 95);
    result.insert("Social", 95);

    // accessing the values of the hashmap
    let maths_score = result.get("Maths");
    match maths_score {
        Some(marks) => println!("Marks obtained in Maths: {}", marks),
        None => println!("No score obtained in Maths"),
    }

    // updating the value of a key
    result.insert("Maths", 100);

    // removing a key-value pair from the hashmap
    result.remove("Social");

    // iterating over the hashmap
    for (key, value) in result.iter() {
        println!("The score obtained in {} is {}", key, value);
    }
}