// strings can be a String (owned) and &str (borrowed)

struct Student {
    name: String,
    class: u8,
}

fn main() {
    let students = vec![
        Student {
            name: "Jane".to_string(),
            class: 2,
        },
        Student {
            name: String::from("Jill"),
            class: 4,
        },
    ];

    for student in students {
        println!("{} is in class {}", student.name, student.class);
    }
}
