/// Represents a monetary amount in euros and cents.
struct Amount {
    euros: u32,
    cents: u32, 
}

fn main() {
    let amount = Amount { euros: 2, cents: 57 };

    // NOTE: this will not compile! (This is intentional.)
    println!("{}", amount);
}
