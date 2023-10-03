enum Ticket {
    Economy(f32, String),
    Business(f32, String),
    FirstClass(f32, String),
}

fn main() {

    let tickets = vec![
        Ticket::Economy(5.0, "John".to_string()),
        Ticket::Business(300.0, "Jane".to_string()),
        Ticket::FirstClass(199.9, "Jill".to_string()),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Economy(price, name) => println!("{} paid ${} for an economy ticket", name, price),
            Ticket::Business(price, name) => println!("{} paid ${} for a business ticket", name, price),
            Ticket::FirstClass(price, name) => println!("{} paid ${} for a first class ticket", name, price),
        }
    }
    
}
