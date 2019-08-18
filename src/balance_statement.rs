use chrono::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct BalanceStatement {
    pub account: String,
    pub period_start: Date<Local>,
    pub period_end: Date<Local>,
    initial_balance: Money,
    balance: Money,
    ingress: Money,
    egress: Money,
    movements: Vec<Movement>,
}

impl BalanceStatement {
    pub fn new(
        account: String,
        period_start: Date<Local>,
        period_end: Date<Local>,
        initial_balance: String,
        movements: Vec<Movement>,
    ) -> BalanceStatement {
        let initial_balance = Money::new(initial_balance);
        let balance = Money(
            movements
                .iter()
                .fold(0, |total, mov| total + (mov.amount.0))
                + initial_balance.0,
        );
        let ingress = Money(
            movements
                .iter()
                .filter(|mov| mov.amount.0 > 0)
                .fold(0, |total, mov| total + (mov.amount.0)),
        );
        let egress = Money(
            movements
                .iter()
                .filter(|mov| mov.amount.0 < 0)
                .fold(0, |total, mov| total + (mov.amount.0)),
        );

        BalanceStatement {
            account,
            period_start,
            period_end,
            initial_balance,
            balance,
            ingress,
            egress,
            movements,
        }
    }

    pub fn balance(&self) -> String {
        self.balance.to_string()
    }

    pub fn ingress(&self) -> String {
        self.ingress.to_string()
    }

    pub fn egress(&self) -> String {
        self.egress.to_string()
    }

    pub fn movements(&self) -> &Vec<Movement> {
        &self.movements
    }
}

#[derive(Debug, PartialEq)]
pub struct Movement {
    pub description: String,
    pub date: DateTime<Local>,
    amount: Money,
    pub parent_group: String,
}

impl Movement {
    pub fn new(
        description: String,
        date: DateTime<Local>,
        amount: String,
        parent_group: String,
    ) -> Movement {
        let amount = Money::new(amount);
        Movement {
            description,
            date,
            amount,
            parent_group,
        }
    }

    pub fn amount(&self) -> String {
        self.amount.to_string()
    }
}

#[derive(PartialEq)]
struct Money(i64);

impl Money {
    fn new(amount: String) -> Money {
        let amount = ((amount.parse::<f64>().unwrap()) * 100.00).trunc() as i64;
        Money(amount)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.abs() < 10 {
            return write!(f, "{}", format!("0.0{}", self.0));
        }
        if self.0.abs() < 100 {
            return write!(f, "{}", format!("0.{}", self.0));
        }
        let money = self.0.to_string();
        let integers = &money[..money.len() - 2];
        let decimals = &money[money.len() - 2..];
        write!(f, "{}", format!("{}.{}", integers, decimals))
    }
}

impl fmt::Debug for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == 0 {
            return write!(f, "{}", format!("0.00"));
        }
        let money = self.0.to_string();
        let integers = &money[..money.len() - 2];
        let decimals = &money[money.len() - 2..];
        write!(f, "{}", format!("{}.{}", integers, decimals))
    }
}
