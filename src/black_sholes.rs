use std::io;
use libm::{log, exp};
use statrs::distribution::{Normal, Continuous};

#[derive(PartialEq, Debug)]
pub enum Side {
    Call,
    Put,
}
pub struct BlackSholesOption {

    pub side: Side,
    pub current_price: f64,
    pub strike_price: f64,
    pub dividend_yield: f64,
    pub volatility: f64,
    pub time_to_maturity: f64,  // In terms of years 
    pub risk_free_rate: f64,
}

impl BlackSholesOption {
    pub fn value(&self) -> f64 {
        let d1 = (log(self.current_price/self.strike_price) + (self.risk_free_rate - self.dividend_yield + self.volatility.powf(2.0)/2.0)*self.time_to_maturity) / (self.volatility*self.time_to_maturity.powf(0.5));
        let d2 = d1 - self.volatility* self.time_to_maturity.powf(0.5);

        let norm = Normal::new(0.0, 1.0).unwrap();
        
        // TO DO: implement a norm.cdf() fuction as has not yet been implemented in the statrs package and replace pdf by cdf

        if self.side == Side::Call {
            let  option_price = self.current_price * exp(-self.dividend_yield*self.time_to_maturity) * norm.pdf(d1) - self.strike_price * exp(-self.risk_free_rate*self.time_to_maturity)*norm.pdf(d2);
            return option_price;
        } else {
            let  option_price = self.strike_price * exp(-self.risk_free_rate*self.time_to_maturity)*norm.pdf(-d2) - self.current_price * exp(-self.dividend_yield*self.time_to_maturity) * norm.pdf(-d1);
            return option_price;
        }

    }    
}

pub fn blacksholes_pricing() {

    println!("Welcome to the Black-Scholes Option pricer.");
    println!("(Step 1/7) What is the current price of the underlying asset?");
    let mut curr_price = String::new();
    io::stdin().read_line(&mut curr_price).expect("Failed to read line");

    println!("(Step 2/7) Do you want a call option ('C') or a put option ('P') ?");
    let mut side_input = String::new();
    io::stdin().read_line(&mut side_input).expect("Failed to read line");

    let side: Side;
    match side_input.trim() {
        "C" | "c" | "Call" | "call" => side = Side::Call,
        "P" | "p" | "Put" | "put" => side = Side::Put,
        _ => panic!("Invalide side argument! Side has to be either 'C' or 'P'."),
    }

    println!("(Step 3/7) What's the stike price you want?");
    let mut strike = String::new();
    io::stdin().read_line(&mut strike).expect("Failed to read line");

    println!("(Step 4/7) What's the expected annualized volatility in %?");
    println!("E.g.: Enter 50% chance as 0.50 ");
    let mut vol = String::new();
    io::stdin().read_line(&mut vol).expect("Failed to read line");

    println!("(Step 5/7) What's the risk-free rate in %?");
    let mut rf = String::new();
    io::stdin().read_line(&mut rf).expect("Failed to read line");

    println!("(Step 6/7) In how many years do you want the expiry to be?");
    let mut expiry = String::new();
    io::stdin().read_line(&mut expiry).expect("Failed to read line");

    println!("(Step 7/7) What is the dividend yield on this stock?");
    let mut div = String::new();
    io::stdin().read_line(&mut div).expect("Failed to read line");

    let option = BlackSholesOption {
        side: side,
        current_price: curr_price.trim().parse::<f64>().unwrap(),
        strike_price: strike.trim().parse::<f64>().unwrap(),
        volatility: vol.trim().parse::<f64>().unwrap(),
        time_to_maturity: expiry.trim().parse::<f64>().unwrap(),
        risk_free_rate: rf.trim().parse::<f64>().unwrap(),
        dividend_yield: div.trim().parse::<f64>().unwrap(),
    };
    println!("This option is currently worth ${}!", option.value());

}