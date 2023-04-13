/// Represents a monetary amount in euros and cents.
#[derive(Debug)]
struct Amount {
    euros: u32,
    cents: u32, 
}

fn main() {
    let amount = Amount { euros: 2, cents: 57 };
    println!("{}.{:02} \u{20AC}", amount.euros, amount.cents);
}
