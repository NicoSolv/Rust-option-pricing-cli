

#[derive(PartialEq, Debug)]
pub enum Side {
    Call,
    Put,
}
pub struct MonteCarloOption {

    pub side: Side,
    pub current_price: f64,
    pub strike_price: f64,
    pub dividend_yield: f64,
    pub volatility: f64,
    pub time_to_maturity: f64,  // In terms of years 
    pub risk_free_rate: f64,
}

impl MonteCarloOption {
    pub fn value(&self) -> f64 {
        todo!("Implement monte carlo simulation")
    }    
}

pub fn montecarlo_pricing() {
    todo!("io")
}