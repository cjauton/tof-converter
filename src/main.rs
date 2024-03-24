use clap::{command, Arg, Command};
use std::fmt;
use uom::fmt::DisplayStyle::Abbreviation;
use uom::si::energy::{electronvolt, gigaelectronvolt, joule, kiloelectronvolt, megaelectronvolt};
use uom::si::f32::{Energy, Length, Time};
use uom::si::length::{centimeter, kilometer, meter};
use uom::si::mass::kilogram;
use uom::si::time::{microsecond, millisecond, nanosecond, second};

// Define custom error type for unsupported units
#[derive(Debug)]
enum UnsupportedUnitError {
    Length(String),
    Time(String),
    Energy(String),
}

impl fmt::Display for UnsupportedUnitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnsupportedUnitError::Length(unit) => write!(f, "Unsupported length unit: {}", unit),
            UnsupportedUnitError::Time(unit) => write!(f, "Unsupported time unit: {}", unit),
            UnsupportedUnitError::Energy(unit) => write!(f, "Unsupported energy unit: {}", unit),
        }
    }
}

fn calculate_energy(time: Time, length: Length) -> Energy {
    let m = uom::si::f32::Mass::new::<kilogram>(1.67493e-27_f32);
    m * (length * length) / (2.0 * time * time)
}

fn calculate_time_of_flight(energy: Energy, length: Length) -> Time {
    let m = uom::si::f32::Mass::new::<kilogram>(1.67493e-27_f32);
    (m * length * length / 2.0 / energy).sqrt()
}

// Functions to parse length and time inputs
fn parse_length(quantity: f32, unit: &str) -> Result<Length, UnsupportedUnitError> {
    match unit.trim().to_lowercase().as_str() {
        "cm" | "centimeter" | "centimeters" => Ok(Length::new::<centimeter>(quantity)),
        "m" | "meter" | "meters" => Ok(Length::new::<meter>(quantity)),
        "km" | "kilometer" | "kilometers" => Ok(Length::new::<kilometer>(quantity)),
        _ => Err(UnsupportedUnitError::Length(unit.to_string())),
    }
}

fn parse_time(quantity: f32, unit: &str) -> Result<Time, UnsupportedUnitError> {
    match unit.trim().to_lowercase().as_str() {
        "ns" | "nanosecond" | "nanoseconds" => Ok(Time::new::<nanosecond>(quantity)),
        "mus" | "us" | "microsecond" | "microseconds" => Ok(Time::new::<microsecond>(quantity)),
        "ms" | "millisecond" | "milliseconds" => Ok(Time::new::<millisecond>(quantity)),
        "s" | "second" | "seconds" => Ok(Time::new::<second>(quantity)),
        _ => Err(UnsupportedUnitError::Time(unit.to_string())),
    }
}

fn parse_energy(quantity: f32, unit: &str) -> Result<Energy, UnsupportedUnitError> {
    match unit.trim().to_lowercase().as_str() {
        "ev" | "electronvolt" | "electronvolts" => Ok(Energy::new::<electronvolt>(quantity)),
        "kev" | "kiloelectronvolt" | "kiloelectronvolts" => {
            Ok(Energy::new::<kiloelectronvolt>(quantity))
        }
        "mev" | "megaelectronvolt" | "megaelectronvolts" => {
            Ok(Energy::new::<megaelectronvolt>(quantity))
        }
        "gev" | "gigaelectronvolt" | "gigaelectronvolts" => {
            Ok(Energy::new::<gigaelectronvolt>(quantity))
        }
        "j" | "joule" | "joules" => Ok(Energy::new::<joule>(quantity)),
        _ => Err(UnsupportedUnitError::Energy(unit.to_string())),
    }
}

// Enum for supported units
enum EnergyUnit {
    Electronvolt,
    Kiloelectronvolt,
    Megaelectronvolt,
    Gigaelectronvolt,
    Joule,
}

// Enum for supported units
enum TimeUnit {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
}

// Function to parse energy unit input
fn parse_energy_unit(unit: &str) -> Result<EnergyUnit, UnsupportedUnitError> {
    match unit.trim().to_lowercase().as_str() {
        "ev" | "electronvolt" | "electronvolts" => Ok(EnergyUnit::Electronvolt),
        "kev" | "kiloelectronvolt" | "kiloelectronvolts" => Ok(EnergyUnit::Kiloelectronvolt),
        "mev" | "megaelectronvolt" | "megaelectronvolts" => Ok(EnergyUnit::Megaelectronvolt),
        "gev" | "gigaelectronvolt" | "gigaelectronvolts" => Ok(EnergyUnit::Gigaelectronvolt),
        "J" | "joule" | "joules" => Ok(EnergyUnit::Joule),
        _ => Err(UnsupportedUnitError::Energy(unit.to_string())),
    }
}

// Function to parse  time unit input
fn parse_time_unit(unit: &str) -> Result<TimeUnit, UnsupportedUnitError> {
    match unit.trim().to_lowercase().as_str() {
        "ns" | "nanosecond" | "nanoseconds" => Ok(TimeUnit::Nanosecond),
        "mus" | "us" | "microsecond" | "microseconds" => Ok(TimeUnit::Microsecond),
        "ms" | "millisecond" | "milliseconds" => Ok(TimeUnit::Millisecond),
        "s" | "second" | "seconds" => Ok(TimeUnit::Second),
        _ => Err(UnsupportedUnitError::Time(unit.to_string())),
    }
}

fn main() {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("to_energy")
                .about("Converts neutron time-of-flight to neutron energy")
                .arg(
                    Arg::new("length_of_flight_path")
                        .help("Flight path length from source to target in units of (cm, m, km)")
                        .short('l')
                        .long("length-of-flight-path")
                        .required(true)
                        .value_names(&["LENGTH", "UNIT"])
                        .num_args(2)
                )
                .arg(
                    Arg::new("time_of_flight")
                        .help("Time-of-Flight to convert to neutron Energy in units of (ns, us, ms, s)")
                        .short('t')
                        .long("time-of-flight")
                        .required(true)
                        .value_names(&["LENGTH", "UNIT"])
                        .num_args(2)
                )
                .arg(
                    Arg::new("unit")
                        .help("Desired neutron energy units of (eV, KeE, MeV, J)")
                        .short('u')
                        .long("unit")
                        .required(false)
                        .value_name("UNIT")
                        .num_args(1)
                        .default_value("eV")
                ),
        )
        .subcommand(
            Command::new("to_tof")
                .about("Converts neutron energy to neutron time-of-flight")
                .arg(
                    Arg::new("length_of_flight_path")
                        .help("Flight path length from source to target in units of (cm, m, km)")
                        .short('l')
                        .long("length-of-flight-path")
                        .required(true)
                        .value_names(&["LENGTH", "UNIT"])
                        .num_args(2)
                )
                .arg(
                    Arg::new("energy")
                        .help("Energy to convert to neutron time-of-flight in units of (eV, KeE, MeV, GeV J)")
                        .short('e')
                        .long("energy")
                        .required(true)
                        .value_names(&["ENERGY", "UNIT"])
                        .num_args(2)
                )
                .arg(
                    Arg::new("unit")
                        .help("Desired neutron time-of-flight units of (ns, us, ms, s)")
                        .short('u')
                        .long("unit")
                        .required(false)
                        .value_name("UNIT")
                        .num_args(1)
                        .default_value("us")
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("to_energy", sub_matches)) => {
            // Parse length
            let length_of_flight_path: Vec<_> = sub_matches
                .get_many::<String>("length_of_flight_path")
                .unwrap()
                .collect();

            let input_length_value: f32 = match length_of_flight_path[0].parse::<f32>() {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Error: parsing length value: {}", err);
                    return;
                }
            };

            let input_length_unit: &str = &length_of_flight_path[1].trim().to_lowercase();

            // Parse time
            let time_of_flight: Vec<_> = sub_matches
                .get_many::<String>("time_of_flight")
                .unwrap()
                .collect();

            let input_time_value: f32 = match time_of_flight[0].parse() {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Error: parsing time value: {}", err);
                    return;
                }
            };

            let input_time_unit: &str = &time_of_flight[1].trim().to_lowercase();

            // Parse unit
            let unit: _ = sub_matches.get_one::<String>("unit").unwrap();

            let output_energy_unit: &str = &unit.trim().to_lowercase();

            let length_quantity = parse_length(input_length_value, input_length_unit)
                .unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                });

            let time_quantity =
                parse_time(input_time_value, input_time_unit).unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                });

            let energy_quantity: Energy = calculate_energy(time_quantity, length_quantity);

            let energy_unit = parse_energy_unit(output_energy_unit).unwrap_or_else(|err| {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            });

            let formatted_energy = match energy_unit {
                EnergyUnit::Electronvolt => {
                    let a = energy_quantity.into_format_args(electronvolt, Abbreviation);
                    format!("{}", a)
                }
                EnergyUnit::Kiloelectronvolt => {
                    let a = energy_quantity.into_format_args(kiloelectronvolt, Abbreviation);
                    format!("{}", a)
                }
                EnergyUnit::Megaelectronvolt => {
                    let a = energy_quantity.into_format_args(megaelectronvolt, Abbreviation);
                    format!("{}", a)
                }
                EnergyUnit::Gigaelectronvolt => {
                    let a = energy_quantity.into_format_args(gigaelectronvolt, Abbreviation);
                    format!("{}", a)
                }
                EnergyUnit::Joule => {
                    let a = energy_quantity.into_format_args(joule, Abbreviation);
                    format!("{}", a)
                }
            };

            println!("Energy = {}", formatted_energy);
        }
        Some(("to_tof", sub_matches)) => {
            // Parse length
            let length_of_flight_path: Vec<_> = sub_matches
                .get_many::<String>("length_of_flight_path")
                .unwrap()
                .collect();

            let input_length_value: f32 = match length_of_flight_path[0].parse::<f32>() {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Error: parsing length value: {}", err);
                    return;
                }
            };

            let input_length_unit: &str = &length_of_flight_path[1].trim().to_lowercase();

            // Parse time
            let energy: Vec<_> = sub_matches.get_many::<String>("energy").unwrap().collect();

            let input_energy_value: f32 = match energy[0].parse() {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Error: parsing energy value: {}", err);
                    return;
                }
            };

            let input_energy_unit: &str = &energy[1].trim().to_lowercase();

            // Parse unit
            let unit: _ = sub_matches.get_one::<String>("unit").unwrap();

            let output_time_unit: &str = &unit.trim().to_lowercase();

            let length_quantity = parse_length(input_length_value, input_length_unit)
                .unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                });

            let energy_quantity = parse_energy(input_energy_value, input_energy_unit)
                .unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    std::process::exit(1);
                });

            let time_quantity: Time = calculate_time_of_flight(energy_quantity, length_quantity);

            let time_unit = parse_time_unit(output_time_unit).unwrap_or_else(|err| {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            });

            let formatted_energy = match time_unit {
                TimeUnit::Nanosecond => {
                    let a = time_quantity.into_format_args(nanosecond, Abbreviation);
                    format!("{}", a)
                }
                TimeUnit::Microsecond => {
                    let a = time_quantity.into_format_args(microsecond, Abbreviation);
                    format!("{}", a)
                }
                TimeUnit::Millisecond => {
                    let a = time_quantity.into_format_args(millisecond, Abbreviation);
                    format!("{}", a)
                }
                TimeUnit::Second => {
                    let a = time_quantity.into_format_args(second, Abbreviation);
                    format!("{}", a)
                }
            };

            println!("TOF = {}", formatted_energy);
        }

        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
