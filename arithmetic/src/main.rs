fn main() {
    let x: i32 = 6;
    let y: i32 = 4;

    // sum function
    let sum_val = sum(x, y);
    println!("Sum of {} and {} is {}", x, y, sum_val);

    // sum function
    let sub_val = sub(x, y);
    println!("Sub of {} and {} is {}", x, y, sub_val);

    // div function
    let div_val = div(x, x);
    println!("Div of {} by {} is {}", x, x, div_val);


    // rem function
    let rem_val = rem(x, y);
    println!("Rem of {} by {} is {}", x, y, rem_val);
}

fn sum (a: i32, b: i32) -> i32 {
    a+b
}

fn sub(a: i32, b:i32) -> i32 {
    a-b
}

fn div(a: i32, b: i32) -> i32 {
    a / b
}

fn rem(a: i32, b: i32) -> i32 {
    a % b
}

