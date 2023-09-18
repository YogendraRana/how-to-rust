fn main() {
    let num = numbers();
    
    if num.0 < num.1 {
        println!("{} is less than {}", num.0, num.1);
    } else if num.0 > num.1 {
        println!("{} is greater than {}", num.0, num.1);
    } else {
        println!("{} is equal to {}", num.0, num.1);
    }
}

fn numbers() -> (i32, i32) {
    // return a tuple
    (4, 5)
}
