fn main() {

    // match boolean
    let status = false;
    match status {
        true => println!("Status is true"),
        false => println!("Status is false"),
    }

    // match number
    let number = 6;
    match number {
        1 => println!("Number is 1"),
        2 => println!("Number is 2"),
        3 => println!("Number is 3"),
        4 => println!("Number is 4"),
        5 => println!("Number is 5"),
        _ => println!("Number is not 1-5"),
    }
}
