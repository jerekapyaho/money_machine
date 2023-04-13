use std::fmt;

/// Represents a monetary amount in euros and cents.
#[derive(Debug)]
struct Amount {
    euros: u32,
    cents: u32, 
}

impl Amount {
    fn as_cents(&self) -> u32 {
        self.euros * 100 + self.cents
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:02} \u{20AC}", self.euros, self.cents)
    }
}

fn main() {
    let amount = Amount { euros: 2, cents: 57 };
    println!("{}", amount);
    println!("In cents = {}", amount.as_cents());

    println!("The amount is still {:?}", amount);
}
