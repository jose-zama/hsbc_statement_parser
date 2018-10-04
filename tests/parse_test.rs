extern crate hsbc_parser;
extern crate chrono;

use hsbc_parser::parser::parse;
use hsbc_parser::balance_statement::BalanceStatement;
use hsbc_parser::balance_statement::Movement;
use chrono::prelude::*;

#[test]
fn parse_document_with_0_movs_should_return_a_balance_statement_with_empty_movs(){
	//setup
	let xml = r#"<?xml ?>
	<cfdi:Comprobante>
	<cfdi:Addenda>
	<DG:DatosGenerales numerodecuenta="000001234567890" periodo="01/08/2018-31/08/2018">
	<DG:Movimientos></DG:Movimientos>
	</DG:DatosGenerales>
	</cfdi:Addenda>
	</cfdi:Comprobante>"#;

	//run
	let result=parse(xml);

	//assert	
	let expected:BalanceStatement = BalanceStatement::new(
		String::from("000001234567890"),
		Local.ymd(2018,08,01),
		Local.ymd(2018,08,31),
		"0".to_string(),
		Vec::new(),
	);
	assert_eq!(result, expected);
}

#[test]
fn parse_document_with_unordered_atts_should_return_a_balance_statement_with_empty_movs(){
	//setup
	let xml = r#"<?xml ?>
	<cfdi:Comprobante>
	<cfdi:Addenda>
	<DG:DatosGenerales periodo="01/08/2018-31/08/2018" numerodecuenta="000001234567890" >
	<DG:Movimientos></DG:Movimientos>
	</DG:DatosGenerales>
	</cfdi:Addenda>
	</cfdi:Comprobante>"#;

	//run
	let result=parse(xml);

	//assert	
	let expected:BalanceStatement = BalanceStatement::new(
		String::from("000001234567890"),
		Local.ymd(2018,08,01),
		Local.ymd(2018,08,31),
		"0".to_string(),
		Vec::new(),
	);
	assert_eq!(result, expected);
}

#[test]
fn parse_document_with_one_mov(){
	//setup
	let xml = r#"<?xml ?>
	<cfdi:Comprobante>
	<cfdi:Addenda>
	<DG:DatosGenerales periodo="01/08/2018-31/08/2018" numerodecuenta="000001234567890" 	>
	<DG:Movimientos>
		<DG:MovimientosDelCliente fecha="2018-08-01T12:00:00" descripcion="8213354446LGR 150220P98CAFE BARISTI COSM" importe="97.00">
		</DG:MovimientosDelCliente>
	</DG:Movimientos>
	</DG:DatosGenerales>
	</cfdi:Addenda>
	</cfdi:Comprobante>"#;

	//run
	let result=parse(xml);

	//assert	
	let movement = Movement::new(
		String::from("8213354446LGR 150220P98CAFE BARISTI COSM"),
		Local.ymd(2018,08,01).and_hms(12,00,00),
		"97.00".to_string(),
		String::from("UNKNOWN"),
		);

	let movs = vec![movement];

	let expected:BalanceStatement = BalanceStatement::new(
		String::from("000001234567890"),
		Local.ymd(2018,08,01),
		Local.ymd(2018,08,31),
		"0".to_string(),
		movs,
	);
	assert_eq!(result, expected);
}




