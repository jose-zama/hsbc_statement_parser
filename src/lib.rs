extern crate chrono;
extern crate quick_xml;

pub mod balance_statement;

pub mod parser {

    use balance_statement::BalanceStatement;
    use balance_statement::Movement;
    use chrono::prelude::*;
    use quick_xml::events::Event;
    use quick_xml::Reader;
    use std::collections::HashMap;
    use std::str;

    pub fn parse(document: &str) -> BalanceStatement {
        let mut reader = Reader::from_str(document);
        reader.trim_text(true);

        let mut txt = Vec::new();
        let mut buf = Vec::new();

        let mut atts_datos_generales = HashMap::new();
        let mut movs = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"DG:DatosGenerales" => {
                        for att in e.attributes() {
                            match att {
                                Ok(expr) => {
                                    let cow = expr;
                                    let key =
                                        str::from_utf8(cow.key).unwrap().to_string().to_lowercase();
                                    let value = str::from_utf8(&cow.value).unwrap().to_string();
                                    atts_datos_generales.insert(key, value);
                                }
                                Err(err) => println!(
                                    "error parsing attribute: {} in {}",
                                    err,
                                    str::from_utf8(e.name()).unwrap()
                                ),
                            }
                        }
                    }
                    b"DG:MovimientosDelCliente" | b"DG:MovimientoDelClienteFiscal" => {
                        let mut atts = HashMap::new();
                        for att in e.attributes() {
                            match att {
                                Ok(expr) => {
                                    let cow = expr;
                                    let key =
                                        str::from_utf8(cow.key).unwrap().to_string().to_lowercase();
                                    let value = str::from_utf8(&cow.value).unwrap().to_string();
                                    atts.insert(key, value);
                                }
                                Err(err) => println!(
                                    "error parsing attribute: {} in {}",
                                    err,
                                    str::from_utf8(e.name()).unwrap()
                                ),
                            }
                        }

                        let mut amount = atts
                            .get("importe")
                            .unwrap_or(&String::from("0"))
                            .to_string();

                        let desc = atts
                            .get("descripcion")
                            .unwrap_or(&String::from(""))
                            .to_string();
                        {
                            if desc.contains("NETNM")
                                || desc.contains("ABONO")
                                || desc.contains("SU PAGO GRACIAS")
                            {
                                amount = format!("-{}", amount)
                            }
                        }

                        match desc.as_ref() {
                            "PROMOCION MESES SIN INTERESES" | "APLICACION DE PROMOCION" => (),
                            _ => movs.push(Movement::new(
                                desc,
                                Local
                                    .datetime_from_str(
                                        &atts.get("fecha").unwrap().to_string(),
                                        "%Y-%m-%dT%H:%M:%S",
                                    )
                                    .unwrap(),
                                amount,
                                String::from("UNKNOWN"),
                            )),
                        }
                    }
                    _ => (),
                },
                Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }

        let period = atts_datos_generales.get("periodo").unwrap().to_string();
        let period_start: Date<Local>;
        let period_end: Date<Local>;
        if period.len() == 21 {
            let period = period.split('-').collect::<Vec<&str>>();
            period_start = Local
                .datetime_from_str(&(period[0].to_string() + "00:00:00"), "%d/%m/%Y%H:%M:%S")
                .unwrap()
                .date();
            period_end = Local
                .datetime_from_str(&(period[1].to_string() + "00:00:00"), "%d/%m/%Y%H:%M:%S")
                .unwrap()
                .date();
        } else {
            period_start = movs[0].date.date();
            period_end = movs[movs.len() - 1].date.date();
        }

        BalanceStatement::new(
            atts_datos_generales
                .get("numerodecuenta")
                .unwrap_or(&String::from("missing"))
                .to_string(),
            period_start,
            period_end,
            "0".to_string(),
            movs,
        )
    }
    // "09 Abr - 08 May 18"
    // 01/10/2018-31/10/2018

}
