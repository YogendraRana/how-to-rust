enum Access {
    Public, 
    Private
}

fn main() {
    let access = Access::Public;

    let can_access = match access {
        Access::Public => true,
        Access::Private => false
    };
    
    println!("Can access: {}", can_access);
}
