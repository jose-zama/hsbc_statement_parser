use chrono::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct BalanceStatement {
	account: String,
	period_start: Date<Local>,
	period_end: Date<Local>,		
	initial_balance: Money,
	balance: Money,		
	ingress: Money,
	egress: Money,
    movements: Vec<Movement>
}

impl BalanceStatement {
    pub fn new(
    	account: String,
    	period_start: Date<Local>,
		period_end: Date<Local>,		
		initial_balance: String,
	    movements: Vec<Movement>
    ) -> BalanceStatement{


    	let balance = Money(movements.iter().fold(0,|total,mov| total+(mov.amount.0)));
    	let initial_balance = Money::new(initial_balance);
    	let ingress = Money::new("0".to_string());
    	let egress = Money::new("0".to_string());
    	BalanceStatement{
			account,
			period_start,
			period_end,
			initial_balance,	
			balance,
			ingress,
			egress,
		    movements
    	}
    }

    pub fn balance(&self)-> String{
		self.balance.to_string()
	}
}

#[derive(Debug, PartialEq)]
pub struct Movement{
	pub description: String,
	pub date: DateTime<Local>,
	amount: Money,
	pub parent_group: String,
}

impl Movement{
	pub fn new(description:String, date:DateTime<Local>, amount:String, parent_group:String) -> Movement{
		let amount = Money::new(amount);
		Movement{
			description,
			date,
			amount,
			parent_group,
		}
	}

}

#[derive(PartialEq)]
pub struct Money(i64);

impl Money{
	pub fn new(amount:String) -> Money{
		let amount = ((amount.parse::<f64>().unwrap())*100.00).trunc() as i64;
		Money(amount)
	}
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	if self.0 == 0{
    		return write!(f, "{}", format!("0.00"))
    	}
    	let money = self.0.to_string();
		let integers = &money[..money.len()-2];
    	let decimals = &money[money.len()-2..];
        write!(f, "{}", format!("{}.{}",integers,decimals))
    }
}

impl fmt::Debug for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
    	if self.0 == 0{
    		return write!(f, "{}", format!("0.00"))
    	}
    	let money = self.0.to_string();
		let integers = &money[..money.len()-2];
    	let decimals = &money[money.len()-2..];
        write!(f, "{}", format!("{}.{}",integers,decimals))
    }
}
