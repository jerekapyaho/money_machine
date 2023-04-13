use std::fmt;

/// Represents a monetary amount in euros and cents.
#[derive(Debug)]
struct Amount {
    euros: u32,
    cents: u32, 
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:02} \u{20AC}", self.euros, self.cents)
    }
}

fn value_in_cents(amount: &Amount) -> u32 {
    amount.euros * 100 + amount.cents
}

fn main() {
    let amount = Amount { euros: 2, cents: 57 };
    println!("{}", amount);
    println!("In cents = {}", value_in_cents(&amount));

    println!("The amount is still {:?}", amount);
}
