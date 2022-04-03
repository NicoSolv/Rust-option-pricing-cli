mod binomial_option;
mod monte_carlo;
mod black_sholes;
use std::io;
// use structopt::StructOpt;

fn main() {
    println!("Do you want to use the Binomial (1), Black-Sholes (2) or Monte-Carlo (3) option pricing model?");
    let mut model = String::new();
    io::stdin().read_line(&mut model).expect("Failed to read line");
    let model: u8= model.parse::<u8>().unwrap();

    if model == 1 {
        binomial_option::binomial_pricing();
    } else if model == 2 {
        black_sholes::blacksholes_pricing();
    } else if model == 3 {
        monte_carlo::montecarlo_pricing();
    } else {
        println!("You gave a wrong number! The accepted arguments are 1, 2 and 3.")
    }
}

