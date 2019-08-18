extern crate hsbc_parser;

use hsbc_parser::parser::parse;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // list_concepts();
    let xml = r#"<?xml version="1.0" encoding="utf-8"?><cfdi:Comprobante xmlns:cfdi="http://www.sat.gob.mx/cfd/3" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:tfd="http://www.sat.gob.mx/TimbreFiscalDigital" xmlns:DG="http://www.hsbc.com.mx/schema/DG" Version="3.3" Folio="4912818603326715" Fecha="2018-08-09T01:05:24" Sello="mn87/h5xBvcBlO/j5rkakPE4KYpY7YriIJ22kd9OWuBU7nhdd7//+zqwiVoUDQnc62acUcPhFOxS5XBDXLgnjp8YMRzy9BMT4YCphpv9HcgFKIq9tHAZgIVZ64VL8pv2lR4WSocZv7t7Dfmmk75amhhA//cRJlsJJFkerAmT1c8YT+F6ykCMP05+hGUkvKJ5jokfSXtkyIwfTwqwwXNcCfdS+gDohisfzHqqBbk0U3ohwVOeM/ZquZ3LSgSvpJaWgP1bmF2elORwfQ9QlDXWfNWgrwNqCYS4e47h2uUwAKB+8XVHlRgW9TAIYCsjx0e2PmIYUGtOfHpdZYgl62C51A==" FormaPago="03" NoCertificado="00001000000404463606" Certificado="MIIGnDCCBISgAwIBAgIUMDAwMDEwMDAwMDA0MDQ0NjM2MDYwDQYJKoZIhvcNAQELBQAwggGyMTgwNgYDVQQDDC9BLkMuIGRlbCBTZXJ2aWNpbyBkZSBBZG1pbmlzdHJhY2nDs24gVHJpYnV0YXJpYTEvMC0GA1UECgwmU2VydmljaW8gZGUgQWRtaW5pc3RyYWNpw7NuIFRyaWJ1dGFyaWExODA2BgNVBAsML0FkbWluaXN0cmFjacOzbiBkZSBTZWd1cmlkYWQgZGUgbGEgSW5mb3JtYWNpw7NuMR8wHQYJKoZIhvcNAQkBFhBhY29kc0BzYXQuZ29iLm14MSYwJAYDVQQJDB1Bdi4gSGlkYWxnbyA3NywgQ29sLiBHdWVycmVybzEOMAwGA1UEEQwFMDYzMDAxCzAJBgNVBAYTAk1YMRkwFwYDVQQIDBBEaXN0cml0byBGZWRlcmFsMRQwEgYDVQQHDAtDdWF1aHTDqW1vYzEVMBMGA1UELRMMU0FUOTcwNzAxTk4zMV0wWwYJKoZIhvcNAQkCDE5SZXNwb25zYWJsZTogQWRtaW5pc3RyYWNpw7NuIENlbnRyYWwgZGUgU2VydmljaW9zIFRyaWJ1dGFyaW9zIGFsIENvbnRyaWJ1eWVudGUwHhcNMTYxMjA1MjM0OTU2WhcNMjAxMjA1MjM0OTU2WjCCATsxSTBHBgNVBAMTQEhTQkMgIE1FWElDTyBTQSBJTlNUSVRVQ0lPTiBERSBCQU5DQSBNVUxUSVBMRSBHUlVQTyBGSU5BTkNJRVJPIEgxTDBKBgNVBCkTQ0hTQkMgIE1FWElDTyBTQSBJTlNUSVRVQ0lPTiBERSBCQU5DQSBNVUxUSVBMRSBHUlVQTyBGSU5BTkNJRVJPIEhTQkMxSTBHBgNVBAoTQEhTQkMgIE1FWElDTyBTQSBJTlNUSVRVQ0lPTiBERSBCQU5DQSBNVUxUSVBMRSBHUlVQTyBGSU5BTkNJRVJPIEgxJTAjBgNVBC0THEhNSTk1MDEyNUtHOCAvIENBR0Y4MTA4MjlOQzAxHjAcBgNVBAUTFSAvIENBR0Y4MTA4MjlIREZNTVIwMDEOMAwGA1UECxMFU1VDMDEwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQC9E4oEDmajt/1iv1up7JqHEDgrp5vWZThlV4n79zJ21Egis5TIlyn2DluJGFC0BSqJAsouj7uwlPffEXhnrxy7uMUZAY4cp/yAXfK/b6ACeNPQbLvSEMvLXmnLkovhijy4mvTtuaoBIJwVO4vR1qFISvGtoD+1qwBafzrtI6jOtfDvesMZ3bDYMh3LkzJDKmqwu0voEgHkVBRvlIRZG1irV0XlIWo36wUfn551zY+tGfmqZvQWgvhKHRwhok1/3cx7f6vRZqRlU8nVpV/XSkQ+L7oXpk3mvkVx3Uo8tj78zOkIEfE3ZXgyIqCsCOZFuxzr6B9UtGvcbFuvxTxuB3BZAgMBAAGjHTAbMAwGA1UdEwEB/wQCMAAwCwYDVR0PBAQDAgbAMA0GCSqGSIb3DQEBCwUAA4ICAQCbPn9e5+hNzUEp7n4QNDjakjYooH0S9anGw9q18FpOdidqKG5xOh/VGHq0vrrIFmee72yP9K2Er0pbgfF7xd5hQdkGw1y/Q+QAvLZrMhOGItUCkzA2hE4TB5GQvq64fWk6Ynj66YQTS9tlRc7UEQdIYdjt7sJ3kR2pieESUN880+L9Y09Ghvftk7veoGq/RlcBYaxvP0+9xjYFXPqDd5Lz48F02ASt49E2SxrCm/1i7RP0/Xvd7j2hSYAqL0boGi9JTetXZ7/zwITzY8+AQ4e7iXtyeNnhoJleT4ADBEy9E8I5ftH4sQPuR9M7yR2DFxtIBpwE9qZIYQWqozP+aIyT7uPyy6upq5x4L4ATgFRGq9uxff9xLskA+ZZrMt7/GtdDexcJvDGxcgeDjtp8zZHSI0ZxPjvzNl4V+NVyF5Q+LQrXPaXK6ELXwQpWRArOf8SasNDtX0oVhqqafNcVLS9DGh8/gs7wL3sTFjBmOvASupfDC3XpTVJizx/JoaaXuL2w6qCFvNFUCv/HqMWDVsXshNePj2heMVn4Dq+Cc7gvU3Q+d4mapjKXSK7Qs9gGtBVXybNneNa8TnqCrgpnJ6CSqPSnG9wnm7bwzb6HbjH8TqByM2qiSv1zkFgIgkmewgwQEBwXNeoQFRjKeVr4Lp1GToh5PWlYhwm38xB/eJBMIw==" SubTotal="0.01" Descuento="0.01" Moneda="MXN" TipoCambio="1" Total="0.00" TipoDeComprobante="I" MetodoPago="PUE" LugarExpedicion="06500" xsi:schemaLocation="http://www.sat.gob.mx/cfd/3 http://www.sat.gob.mx/sitio_internet/cfd/3/cfdv33.xsd http://www.sat.gob.mx/TimbreFiscalDigital http://www.sat.gob.mx/sitio_internet/TimbreFiscalDigital/TimbreFiscalDigital.xsd http://www.hsbc.com.mx/schema/DG http://www.hsbc.com.mx/schema/DG.xsd"><cfdi:Emisor Rfc="HMI950125KG8" Nombre="HSBC MEXICO, S.A. INSTITUCION DE BANCA MULTIPLE GRUPO FINANCIERO HSBC" RegimenFiscal="601"></cfdi:Emisor><cfdi:Receptor Rfc="ZABA860712II4" Nombre="JOSE ARMANDO ZAMUDIO BARRERA" UsoCFDI="G03"></cfdi:Receptor><cfdi:Conceptos><cfdi:Concepto ClaveProdServ="84121500" Cantidad="1" ClaveUnidad="E48" Unidad="Unidad de servicio" Descripcion="Servicios de Facturacion" ValorUnitario="0.01" Importe="0.01" Descuento="0.01"></cfdi:Concepto></cfdi:Conceptos><cfdi:Complemento><tfd:TimbreFiscalDigital xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="http://www.sat.gob.mx/TimbreFiscalDigital http://www.sat.gob.mx/sitio_internet/cfd/TimbreFiscalDigital/TimbreFiscalDigitalv11.xsd" Version="1.1" UUID="F9F5AF60-CB68-4B5D-BDF3-34C6B373F256" FechaTimbrado="2018-08-09T01:37:24" RfcProvCertif="CEC961028A98" SelloCFD="mn87/h5xBvcBlO/j5rkakPE4KYpY7YriIJ22kd9OWuBU7nhdd7//+zqwiVoUDQnc62acUcPhFOxS5XBDXLgnjp8YMRzy9BMT4YCphpv9HcgFKIq9tHAZgIVZ64VL8pv2lR4WSocZv7t7Dfmmk75amhhA//cRJlsJJFkerAmT1c8YT+F6ykCMP05+hGUkvKJ5jokfSXtkyIwfTwqwwXNcCfdS+gDohisfzHqqBbk0U3ohwVOeM/ZquZ3LSgSvpJaWgP1bmF2elORwfQ9QlDXWfNWgrwNqCYS4e47h2uUwAKB+8XVHlRgW9TAIYCsjx0e2PmIYUGtOfHpdZYgl62C51A==" NoCertificadoSAT="00001000000407058485" SelloSAT="K1wDV6boykdIfVVUlTQRZR/reeb3PxXsjyejjUeamesbwr3eXxxHojOhd/z9+7H/c5i8Bo8Xa1fpqZ3TEYjmRoYbMBfoh0/ApJJm3DCxAFWYSnbiHDl8p7Jgvz4N0mE0kqF0JR8gpvkxRrN3bUntUr7+EB3/Av1Z+50qnIW4spgQt2AxwiS7NvLz5i3UvcQtyUgwWkdzOzQbRLrm3ES+tkk8OKWbs30C9c17Dnh+957ntdNY06BT+f2RMRI08e4uNYm8uitSjb9DzKrdsczpFB+Ak/ZkzNS6P4k1n4hB0ouSvTk7fqhqAksRoo7ncKI7fnB91jRuVuinvBeIwJuQkg==" xmlns:tfd="http://www.sat.gob.mx/TimbreFiscalDigital"/></cfdi:Complemento><cfdi:Addenda><DG:DatosGenerales Version="1.0" NumeroDeCuenta="491281860332671" NombreDelCliente="JOSE ARMANDO ZAMUDIO BARRERA" Periodo="09 Jul - 08 Ago 18"><DG:Movimientos><DG:MovimientosDelCliente Fecha="2018-07-08T12:00:00" Descripcion="PAYPAL UBER BV 35314369001 NL" Importe="151.50"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-07-15T12:00:00" Descripcion="PAYPAL UBER BV 35314369001 NL" Importe="41.03"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-07-21T12:00:00" Descripcion="PAYPAL UBER BV 35314369001 NL" Importe="306.03"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-07-22T12:00:00" Descripcion="PAYPAL UBER BV 35314369001 NL" Importe="140.39"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-07-24T12:00:00" Descripcion="SU PAGO GRACIAS" Importe="4740.03"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-07-28T12:00:00" Descripcion="PAYPAL UBER BV 35314369001 NL" Importe="140.39"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-07-28T12:00:00" Descripcion="PAYPAL UBER BV 35314369001 NL" Importe="121.20"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-07-28T12:00:00" Descripcion="GYMPASS 967866524875 SCHIPHOL NL" Importe="604.99"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-07-29T12:00:00" Descripcion="PAYPAL UBER BV 35314369001 NL" Importe="133.71"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-08-04T12:00:00" Descripcion="PAYPAL UBER BV 35314369001 NL" Importe="53.50"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-08-04T12:00:00" Descripcion="PAYPAL UBER BV 35314369001 NL" Importe="56.36"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-08-04T12:00:00" Descripcion="PAYPAL UBER BV 35314369001 NL" Importe="50.51"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-08-05T12:00:00" Descripcion="PAYPAL UBER BV 35314369001 NL" Importe="40.94"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-08-05T12:00:00" Descripcion="SM CITY PARKING METERS SANTA MONICA US" Importe="55.94"></DG:MovimientosDelCliente><DG:MovimientosDelCliente Fecha="2018-08-06T12:00:00" Descripcion="PAYPAL UBER BV 35314369001 NL" Importe="51.67"></DG:MovimientosDelCliente><DG:MovimientoDelClienteFiscal Fecha="2018-07-28T12:00:00" Descripcion="CINEPOLIS TAQUILLA MOR" RFCenajenante="CME981208VE4" Importe="332.00"></DG:MovimientoDelClienteFiscal><DG:MovimientoDelClienteFiscal Fecha="2018-08-05T12:00:00" Descripcion="FOX RENT TJ TIJ" RFCenajenante="TGA9411308Q6" Importe="0.38"></DG:MovimientoDelClienteFiscal><DG:MovimientoDelClienteFiscal Fecha="2018-08-06T12:00:00" Descripcion="FOX RENT TJ TIJ" RFCenajenante="TGA9411308Q6" Importe="0.01"></DG:MovimientoDelClienteFiscal></DG:Movimientos></DG:DatosGenerales></cfdi:Addenda></cfdi:Comprobante>                                                                                                                                                                                                          "#;

    //run
    let result = parse(xml);

    println!("Account: {}", result.account);
    println!("Balance: {}", result.balance());
    println!("Ingress: {}", result.ingress());
    println!("Egress: {}", result.egress());
    println!("Period start: {}", result.period_start);
    println!("Period end: {}", result.period_end);
    for mov in result.movements() {
        println!(
            "Date: {}, Amount: {}, \tDesc: {}",
            mov.date,
            mov.amount(),
            mov.description
        );
    }
}

fn list_concepts() {
    //iterate files in folder
    let training_path = Path::new("");

    let mut descriptions = String::from("");

    match fs::read_dir(training_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => {
            for path in paths {
                let path = path.unwrap().path();
                let ext = path.extension();
                let ext = ext.unwrap_or_else(|| OsStr::new("")).to_str().unwrap();
                if ext == "xml" {
                    let contents =
                        fs::read_to_string(&path).expect("Something went wrong reading the file");
                    let result = parse(&contents);
                    for mov in result.movements() {
                        // writeln!(descriptions, "{}", &mov.description).unwrap();
                        descriptions.push_str(&mov.description);
                        descriptions.push_str("\n");
                    }
                }
            }
        }
    }

    //print in file
    let mut file = File::create("").expect("fail creating file");
    file.write_all(descriptions.as_bytes()).unwrap();
}
