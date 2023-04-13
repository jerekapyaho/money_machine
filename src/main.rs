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

impl Amount {
    fn new(cents: u32) -> Self {
        Amount {
            euros: cents / 100,
            cents: cents % 100
        }
    }
    
    fn as_cents(&self) -> u32 {
        self.euros * 100 + self.cents
    }
}

fn main() {
    let amount = Amount { euros: 2, cents: 57 };
    println!("{}", amount);

    let new_amount = Amount::new(586);
    println!("{}", new_amount);
}
