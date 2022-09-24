//! 
//! # Funksteckdosen Rest Server
//! 

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::io::Cursor;

use clap::Parser;
#[cfg(feature = "wiringpi")]
use funksteckdose::{wiringpi::WiringPiPin, Device, EncodingA, Funksteckdose, Protocol1, State};
#[cfg(not(feature = "wiringpi"))]
use log::info;
use log::{trace, warn};
use rocket::{
    config::{Config, ConfigError, Environment},
    http::Status,
    Response,
};
use std::net::IpAddr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Server port
    #[clap(short, long, value_parser)]
    port: u16,

    /// Bind IP
    #[clap(short, long, value_parser, default_value = "127.0.0.1")]
    bind: IpAddr,
}

fn main() -> Result<(), ConfigError> {
    let args = Args::parse();

    let config = Config::build(Environment::Production)
        .address(args.bind.to_string())
        .port(args.port)
        .log_level(rocket::logger::LoggingLevel::Normal)
        .secret_key("7hlhV2GabziqFYrrbvyPdzxPd6/NFo+GwLfRUqWsdeQ=") // prevent warning, not used
        .finalize()?;

    rocket::custom(config).mount("/pin", routes![pin]).launch();
    Ok(())
}

///
/// Get endpoint 
/// 
#[get("/<system_code>/<unit_code>/<command_code>")]
fn pin(system_code: String, unit_code: u8, command_code: u8) -> Response<'static> {
    trace!("Call {} {} {}", system_code, unit_code, command_code);
    match send(&system_code, unit_code, command_code) {
        Ok(_) => Response::build().status(Status::NoContent).finalize(),
        Err(message) => {
            warn!("Error {}", message);
            Response::build()
                .status(Status::BadRequest)
                .sized_body(Cursor::new(message))
                .finalize()
        }
    }
}

///
/// Dummy method for sending.
/// 
#[cfg(not(feature = "wiringpi"))]
fn send(system_code: &String, unit_code: u8, command_code: u8) -> Result<(), &'static str> {
    validate_code(system_code)?;
    info!("Call {} {} {}", system_code, unit_code, command_code);
    Ok(())
}

///
/// Send command to wiringPi
/// 
/// # Example
/// ```
/// send("11100", 1, 0);
/// ```
/// 
#[cfg(feature = "wiringpi")]
fn send(system_code: &String, unit_code: u8, command_code: u8) -> Result<(), &'static str> {
    validate_code(system_code)?;
    type Dose = Funksteckdose<WiringPiPin, EncodingA, Protocol1>;
    let pin = WiringPiPin::new(0);
    let d = Dose::new(pin);
    d.send(
        system_code,
        &convert_unit(unit_code),
        &convert_command(command_code),
    )
    .expect("Failed to send");
    Ok(())
}

///
/// Validate the system code. Code have to be a String of "0" and "1" with a length of 5.
/// 
/// # Example
/// ```
/// validate_code("11100"); // returns Ok(())
/// validate_code("111000") // returns Err(message)
/// ```
/// 
fn validate_code(code: &String) -> Result<(), &'static str> {
    if code.len() != 5 {
        return Err("Code length is not 5");
    }
    for c in code.chars() {
        if c != '0' && c != '1' {
            return Err("Code contains not only 0 and 1");
        }
    }
    Ok(())
}

///
/// Convert the unit code into a funksteckdose::Device
/// 
/// # Example
/// ```
/// let device = convert_unit(1);
/// 
/// assert_eq!(funksteckdose::Device::A, device);
/// ```
/// 
#[cfg(feature = "wiringpi")]
fn convert_unit(no: u8) -> Device {
    match no {
        2 => Device::B,
        3 => Device::C,
        4 => Device::D,
        5 => Device::E,
        _ => Device::A,
    }
}

///
/// Convert a command into a funksteckdose::State
/// 
/// # Example
/// ```
/// let state = convert_command(1);
/// 
/// assert_eq!(funksteckdose::State::On, state);
/// ```
/// 
#[cfg(feature = "wiringpi")]
fn convert_command(state: u8) -> State {
    match state {
        1 => State::On,
        _ => State::Off,
    }
}
