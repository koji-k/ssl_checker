extern crate native_tls;
use native_tls::TlsStream;
use native_tls::{Error, TlsConnector};
use std::env;
use std::net::TcpStream;

extern crate chrono;
extern crate chrono_tz;
extern crate openssl;

use chrono::prelude::*;
use openssl::error::ErrorStack;
use openssl::x509::X509;

fn now() -> DateTime<Utc> {
    chrono::offset::Utc::now()
}

fn parse_date(date_string: String) -> Vec<String> {
    let mut vec = vec![];
    for s in date_string.split(" ") {
        match s {
            "" => {}
            _ => vec.push(s.to_string()),
        }
    }
    vec
}

fn rearrange(vec: Vec<String>) -> Vec<String> {
    let mut vec2 = vec![];
    vec2.push(vec[3].to_string());
    vec2.push(vec[0].to_string());
    vec2.push(vec[1].to_string());
    vec2.push(vec[2].to_string());
    vec2
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let domain = &args[1];
    let port: i32 = 443;

    let connector = TlsConnector::new().unwrap();
    let stream: TcpStream = TcpStream::connect(format!("{}:{}", &domain, port)).unwrap();
    let stream: TlsStream<TcpStream> = connector.connect(&domain, stream).unwrap();

    let der_encoded_certificate = stream.peer_certificate().unwrap().unwrap().to_der();
    let date_string = match der_encoded_certificate {
        Ok(certificate) => match openssl::x509::X509::from_der(&certificate) {
            Ok(x509) => Ok(x509.not_after().to_string()),
            Err(_) => Err("Invalid Certificate"),
        },
        Err(_) => Err("Invalid Certificate"),
    };

    match date_string {
        Ok(date_string) => {
            let vec = parse_date(date_string);
            let hoge: String = rearrange(vec).join(" ");

            // https://docs.rs/chrono/0.4.0/chrono/format/strftime/index.html
            let limit = Utc.datetime_from_str(&hoge, "%Y %b %e %H:%M:%S").unwrap();
            let now = now();
            let limit_duration = limit.signed_duration_since(now);
            println!("{:?}", limit_duration.num_days());
        }
        Err(_) => {
            println!("Error!");
        }
    }
}
