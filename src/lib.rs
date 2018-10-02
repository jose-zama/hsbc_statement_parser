extern crate chrono;
extern crate quick_xml;

pub mod balance_statement;

pub mod parser {
    
	use balance_statement::BalanceStatement;
	use chrono::prelude::*;
	use quick_xml::Reader;
	use quick_xml::events::Event;
	use std::str;
	use std::collections::HashMap;

pub fn parse(document:&str)-> BalanceStatement{

	let mut reader = Reader::from_str(document);
	reader.trim_text(true);

	let mut txt = Vec::new();
	let mut buf = Vec::new();

	let mut atts = HashMap::new();

	loop {
	    match reader.read_event(&mut buf) {
	        Ok(Event::Start(ref e)) => {
	            match e.name() {
	                b"DG:DatosGenerales" => {

						for att in e.attributes(){
							let cow = att.unwrap();
	                		let key = str::from_utf8(cow.key).unwrap().to_string();
	                		let value = str::from_utf8(&cow.value).unwrap().to_string();
	                		atts.insert(key,value);
	                	}

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

	let period = atts.get("periodo").unwrap().split('-').collect::<Vec<&str>>();
	let period_start:Date<Local> = Local.datetime_from_str(&(period[0].to_string()+"00:00:00"),"%d/%m/%Y%H:%M:%S").unwrap().date();
	let period_end:Date<Local>= Local.datetime_from_str(&(period[1].to_string()+"00:00:00"),"%d/%m/%Y%H:%M:%S").unwrap().date();

		BalanceStatement::new(	
			atts.get("numerodecuenta").unwrap().to_string(),
			period_start,
			period_end,
			0.0,
			0.0,
			0.0,
			0.0,		
			Vec::new(),			
		)
	}


}
