enum Resource {
    Coal(u8),
    Iron(u8),
    Copper(u8),
    Electricity(u8),
}

struct Entity {
    inventory: Vec<Resource>,
}

fn main() {
    println!("Hello, world!");
}
