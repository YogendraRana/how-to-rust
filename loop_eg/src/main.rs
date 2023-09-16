fn main() {
    // looping with loop
    // loop runs forever until break is called
    // break is used to exit the loop
    let mut val1 = 3;
    println!("Loop countdown:");
    loop {
        println!("{}", val1);
        val1 -= 1;
        if val1 == 0 {
            break;
        }
    }

    // looping with while
    let mut val2 = 5;
    println!("While loop countdown:");
    while val2 > 0 {
        println!("{}", val2);
        val2 -= 1;
    }
}
