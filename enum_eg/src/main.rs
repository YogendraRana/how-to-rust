// enum should be defined outside of main function
enum Role {
    Admin,
    User,
}

fn print_role(role: Role) {
    match role {
        Role::Admin => println!("The user is an admin!"),
        Role::User => println!("The user is a normal user!"),
    }
}

fn main() {
    print_role(Role::Admin)
}
