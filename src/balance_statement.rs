use chrono::prelude::*;

#[derive(Debug, PartialEq)]
pub struct BalanceStatement {
	account: String,
	period_start: Date<Local>,
	period_end: Date<Local>,		
	initial_balance: f64,
	balance: f64,		
	ingress: f64,
	egress: f64,
    movements: Vec<Movement>
}

impl BalanceStatement {
    pub fn new(
    	account: String,
    	period_start: Date<Local>,
		period_end: Date<Local>,		
		initial_balance: f64,
		balance: f64,		
		ingress: f64,
		egress: f64,
	    movements: Vec<Movement>
    ) -> BalanceStatement{
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
}

#[derive(Debug, PartialEq)]
pub struct Movement{
	description: String,
	date: DateTime<Local>,
	amount: f64,
	parent_group: String,
}

impl Movement{
	pub fn new(description:String, date:DateTime<Local>, amount:f64, parent_group:String) -> Movement{
		Movement{
			description,
			date,
			amount,
			parent_group,
		}
	}
}
