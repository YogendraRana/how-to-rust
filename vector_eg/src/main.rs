// vector in Rust is similar to array in C

fn main() {

    // vector using vec! macro
    let my_vector = vec![1, 2, 4, 5];
    println!("my_vector: {:?}", my_vector);

    // vector using new() method
    let mut my_another_vector = Vec::new();
    my_another_vector.push(100);
    my_another_vector.push(101);
    my_another_vector.push(102);
    println!("my_another_vector: {:?}", my_another_vector);

    // vector methods
    println!("\nVector methods:");
    println!("my_vector length: {}", my_vector.len());
    println!("my_vector is empty: {}", my_vector.is_empty());
    println!("my_vector contains 4: {}", my_vector.contains(&4));  
    
    // vector iteration
    println!("\nVector iteration:");

    for num in my_vector {
        println!("{}", num);
    }
}
