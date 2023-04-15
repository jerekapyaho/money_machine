use std::fmt;

/// Represents a monetary amount in euros and cents.
#[derive(Debug)]
struct Amount {
    euros: u32,
    cents: u32, 
}

// Custom Display trait implementation for Amount
impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:02} \u{20AC}", self.euros, self.cents)
    }
}

impl Amount {
    /// Makes a new amount from cents
    fn new(cents: u32) -> Self {
        Amount {
            euros: cents / 100,
            cents: cents % 100
        }
    }
    
    /// Gets the amount as cents
    fn as_cents(&self) -> u32 {
        self.euros * 100 + self.cents
    }
}

/// Euro coins
enum Coin {
    Cent1,
    Cent2,
    Cent5,
    Cent10,
    Cent20,
    Cent50,
    Eur1,
    Eur2,
}

impl Coin {
    /// Gets the monetary value of this coin in cents.
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Cent1 => 1,
            Coin::Cent2 => 2,
            Coin::Cent5 => 5,
            Coin::Cent10 => 10,
            Coin::Cent20 => 20,
            Coin::Cent50 => 50,
            Coin::Eur1 => 100,
            Coin::Eur2 => 200 
        }    
    }
}

fn main() {
    // Make an amount with euros and cents
    let amount = Amount { euros: 2, cents: 57 };
    println!("{}", amount);

    // Make a new amount with cents only 
    let new_amount = Amount::new(586);
    println!("{}", new_amount);

    // Make a coin
    let twenty_cents = Coin::Cent20;
    println!("{}", twenty_cents.value_in_cents());

    // Make an array of coins
    let my_coins = [Coin::Cent20, Coin::Cent5, Coin::Cent10];

    // Sum up the values of the coins
    let mut sum = 0;
    for coin in my_coins.iter() {
        sum += coin.value_in_cents();
    }

    // Make an amount from the total coin value
    // and show it (implicitly using the Display trait) 
    let my_fortune = Amount::new(sum);
    println!("My fortune = {}", my_fortune);
}
