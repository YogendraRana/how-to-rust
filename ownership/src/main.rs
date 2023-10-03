struct Book {
    name: String,
    price: i32,
}

fn display_name(book: &Book) {
    println!("Name: {}", book.name);
}

fn display_price(book: &Book) {
    println!("Price: {}", book.price);
}

fn main() {
    let book1 = Book {
        name: String::from("Romeo and Juliet"),
        price: 1200,
    };

    display_name(&book1);
    display_price(&book1);
}