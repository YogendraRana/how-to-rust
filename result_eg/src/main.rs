// // Result<T, E> is an enum with two variants:
// // Ok(T) and Err(E)
// // T and E are generic type parameters
// // T is the type of the value that will be returned in a success case within the Ok variant
// // E is the type of the error that will be returned in a failure case within the Err variant

// #[derive(Debug)]
// enum Menu {
//     Pizza,
//     Burger,
//     Pasta,
// }

// fn get_order(order: &str) -> Result<Menu, String> {
//     match order {
//         "Pizza" => Ok(Menu::Pizza),
//         "Burger" => Ok(Menu::Burger),
//         "Pasta" => Ok(Menu::Pasta),
//         _ => Err("Not Available.".to_owned()),
//     }
// }

// fn print_order(order: &Menu) {
//     println!("Order: {:?}", order);
// }

// fn main() {
//     let order: Result<Menu, _> = get_order("Pizza");
//     match order {
//         OK(inner_order) => print_order(&inner_order),
//         Err(e) => println!("Error: {:?}", e),
//     }
// }
