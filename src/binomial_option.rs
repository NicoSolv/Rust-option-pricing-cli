use itertools::{Itertools, zip};
use std::io;

#[derive(PartialEq, Debug)]
pub enum Side {
    Call,
    Put,
}

pub struct BinomialOption {
    
    pub side: Side,
    pub current_price: f32,
    pub strike_price: f32,
    pub up_probability: f32,
    pub price_change: f32,
    pub periods: u32,
}

impl BinomialOption {
    pub fn value(&self) -> f32 {
        let mut final_prices: Vec<f32> = Vec::new();
        let mut final_probabilities: Vec<f32> = Vec::new();

        let max_price: f32 = self.current_price + self.periods as f32 * self.price_change;
        let min_price: f32 = self.current_price - self.periods as f32 * self.price_change;
        
        let mut current_price: f32 = max_price;
        while current_price >= min_price {
            final_prices.push(current_price);
            current_price -= 2 as f32 * self.price_change;
        }

        for n in 0..final_prices.len() {
            let combs: usize = (1..final_prices.len()).combinations(n).count();
            final_probabilities.push(
                combs as f32 
                * self.up_probability.powi(self.periods as i32 - n as i32) 
                * (1.0 - self.up_probability).powi(n as i32)
            )
        }

        let mut option_price: f32 = 0.0;

        if self.side == Side::Call {
            for (a, b) in zip(final_prices, final_probabilities) {
                if a > self.strike_price {
                    option_price += (a-self.strike_price) * b;
                }
            }
        } else {
        for (a, b) in zip(final_prices, final_probabilities) {
            if a < self.strike_price {
                option_price += (self.strike_price -  a) * b;
                }
            }
        }

        option_price

    }
}


pub fn binomial_pricing() {

    println!("Welcome to the Binomial Option pricer.");
    println!("(Step 1/6) What is the current price of the underlying asset?");
    let mut curr_price = String::new();
    io::stdin().read_line(&mut curr_price).expect("Failed to read line");

    println!("(Step 2/6) Do you want a call option ('C') or a put option ('P') ?");
    let mut side_input = String::new();
    io::stdin().read_line(&mut side_input).expect("Failed to read line");

    let side: Side;
    match side_input.trim() {
        "C" | "c" | "Call" | "call" => side = Side::Call,
        "P" | "p" | "Put" | "put" => side = Side::Put,
        _ => panic!("Invalide side argument! Side has to be either 'C' or 'P'."),
    }

    println!("(Step 3/6) What's the stike price you want?");
    let mut strike = String::new();
    io::stdin().read_line(&mut strike).expect("Failed to read line");

    println!("(Step 4/6) What's the probability of the asset going up in one period?");
    println!("E.g.: Enter 50% chance as 0.50 ");
    let mut prob = String::new();
    io::stdin().read_line(&mut prob).expect("Failed to read line");

    println!("(Step 5/6) Whats the average amount of change of the asset price in one period?");
    let mut change = String::new();
    io::stdin().read_line(&mut change).expect("Failed to read line");

    println!("(Step 6/6) In how many periods do you want the expiry to be?");
    let mut expiry = String::new();
    io::stdin().read_line(&mut expiry).expect("Failed to read line");

    let option1 = BinomialOption {
        side: side,
        current_price: curr_price.trim().parse::<f32>().unwrap(),
        strike_price: strike.trim().parse::<f32>().unwrap(),
        up_probability: prob.trim().parse::<f32>().unwrap(),
        price_change: change.trim().parse::<f32>().unwrap(),
        periods: expiry.trim().parse::<u32>().unwrap(),
    };
    println!("This option is currently worth ${}!", option1.value());

}