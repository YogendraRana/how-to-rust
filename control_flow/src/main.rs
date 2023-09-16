fn main() {
    let number = 100 as i32;

    // using small bracket result is error
    if number < 100 {
        println!("{} is less than 100", number);
    }else if number > 100 {
        println!("{} is greater than 100", number);
    }else {
        println!("{} is equal to 100", number);
    }
}