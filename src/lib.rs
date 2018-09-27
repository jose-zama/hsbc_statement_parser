extern crate chrono;
extern crate quick_xml;

pub mod balance_statement;

// pub use self::balance_statement::BalanceStatement;
// pub use self::balance_statement::Movement;

pub mod parser {
    
	use balance_statement::BalanceStatement;
	use chrono::prelude::*;
	use quick_xml::Reader;
	use quick_xml::events::Event;
	use std::str;

pub fn parse(document:&str)-> BalanceStatement{

	let mut reader = Reader::from_str(document);
	reader.trim_text(true);

	let mut txt = Vec::new();
	let mut buf = Vec::new();

	// let mut period_start:Date<Local>;
	// let mut period_end:Date<Local>;

	let mut periodo_value: String = String::from("");

	loop {
	    match reader.read_event(&mut buf) {
	        Ok(Event::Start(ref e)) => {
	            match e.name() {
	                b"DG:DatosGenerales" => {

	                	println!("attributes values: {:?}",
	                                    e.attributes()
	                                    .map(|a| a.unwrap().value)
	                                    .collect::<Vec<_>>());
	                	let vaca = &e.attributes()
	                                    .map(|a| a.unwrap().value)
	                                    .collect::<Vec<_>>()[1];
	                    periodo_value = str::from_utf8(vaca).unwrap().to_string();
	                    println!("---{:?}", vaca);
	                }
	                _ => (),
	            }
	        },
	        Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
	        Ok(Event::Eof) => break, // exits the loop when reaching end of file
	        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
	        _ => (), // There are several other `Event`s we do not consider here
	    }

	    // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
	    buf.clear();
	}

	println!("{:?}", periodo_value);

		BalanceStatement::new(	
			String::from(""),
			Local::today(),
			Local::today(),
			0.0,
			0.0,
			0.0,
			0.0,		
			Vec::new(),			
		)
	}


}
