extern crate hsbc_parser;
extern crate chrono;

use hsbc_parser::balance_statement::BalanceStatement;
use hsbc_parser::balance_statement::Movement;
use chrono::prelude::*;

#[test]
fn new_should_return_statement_with_balance_being_the_sum_of_the_movs(){
	//setup
	let movs = vec![
		Movement::new(
			String::from("Mov 1"),
			Local::now(),
			"-92.01".to_string(),
			String::from("UNKNOWN"),
		),
 		Movement::new(
			String::from("Mov 2"),
			Local::now(),
			"91.00".to_string(),
			String::from("UNKNOWN"),
		)
	];
	
	//run
	let result = BalanceStatement::new(
		String::from("000001234567890"),
		Local.ymd(2018,08,01),
		Local.ymd(2018,08,31),
		"0".to_string(),
		movs,
	);

	//assert	
	assert_eq!(result.balance(), "-1.01");
}

#[test]
fn new_should_return_statement_with_balance_being_0_when_no_movs(){
	//setup
	let movs:Vec<Movement> = Vec::new();
	
	//run
	let result = BalanceStatement::new(
		String::from("000001234567890"),
		Local.ymd(2018,08,01),
		Local.ymd(2018,08,31),
		"0".to_string(),
		movs,
	);

	//assert	
	assert_eq!(result.balance(), "0.00");
}

#[test]
fn new_should_return_statement_with_ingress_being_the_sum_of_the_deposits(){
	//setup
	let movs = vec![
		Movement::new(
			String::from("Mov 1"),
			Local::now(),
			"0.01".to_string(),
			String::from("UNKNOWN"),
		),
 		Movement::new(
			String::from("Mov 2"),
			Local::now(),
			"99.99".to_string(),
			String::from("UNKNOWN"),
		)
	];
	
	//run
	let result = BalanceStatement::new(
		String::from("000001234567890"),
		Local.ymd(2018,08,01),
		Local.ymd(2018,08,31),
		"0".to_string(),
		movs,
	);

	//assert	
	assert_eq!(result.ingress(), "100.00");
}

#[test]
fn new_should_return_statement_with_egress_being_the_sum_of_the_withdrawals(){
	//setup
	let movs = vec![
		Movement::new(
			String::from("Mov 1"),
			Local::now(),
			"-0.02".to_string(),
			String::from("UNKNOWN"),
		),
 		Movement::new(
			String::from("Mov 2"),
			Local::now(),
			"-999.99".to_string(),
			String::from("UNKNOWN"),
		)
	];
	
	//run
	let result = BalanceStatement::new(
		String::from("000001234567890"),
		Local.ymd(2018,08,01),
		Local.ymd(2018,08,31),
		"0".to_string(),
		movs,
	);

	//assert	
	assert_eq!(result.egress(), "-1000.01");
}


