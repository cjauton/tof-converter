use {
    clap::{command, Arg, Command},
    thiserror::Error,
    uom::{
        fmt::DisplayStyle::Abbreviation,
        si::{
            energy::{electronvolt, gigaelectronvolt, joule, kiloelectronvolt, megaelectronvolt},
            f32::{Energy, Length, Time},
            length::{centimeter, kilometer, meter},
            mass::kilogram,
            time::{microsecond, millisecond, nanosecond, second},
        },
    },
};

#[derive(Error, Debug)]
enum UnitError {
    #[error("Unsupported unit: {0}")]
    UnsupportedUnit(String),
}

fn calculate_energy(time: Time, length: Length) -> Result<Energy, Box<dyn std::error::Error>> {
    let m = uom::si::f32::Mass::new::<kilogram>(1.67493e-27_f32);
    Ok(m * (length * length) / (2.0 * time * time))
}

fn calculate_time_of_flight(
    energy: Energy,
    length: Length,
) -> Result<Time, Box<dyn std::error::Error>> {
    let m = uom::si::f32::Mass::new::<kilogram>(1.67493e-27_f32);
    Ok((m * length * length / 2.0 / energy).sqrt())
}

// Functions to parse length and time inputs
fn parse_length(quantity: f32, unit: &str) -> Result<Length, Box<dyn std::error::Error>> {
    match unit.trim().to_lowercase().as_str() {
        "cm" | "centimeter" | "centimeters" => Ok(Length::new::<centimeter>(quantity)),
        "m" | "meter" | "meters" => Ok(Length::new::<meter>(quantity)),
        "km" | "kilometer" | "kilometers" => Ok(Length::new::<kilometer>(quantity)),
        _ => Err(UnitError::UnsupportedUnit(unit.to_string()).into()),
    }
}

fn parse_time(quantity: f32, unit: &str) -> Result<Time, Box<dyn std::error::Error>> {
    match unit.trim().to_lowercase().as_str() {
        "ns" | "nanosecond" | "nanoseconds" => Ok(Time::new::<nanosecond>(quantity)),
        "mus" | "us" | "microsecond" | "microseconds" => Ok(Time::new::<microsecond>(quantity)),
        "ms" | "millisecond" | "milliseconds" => Ok(Time::new::<millisecond>(quantity)),
        "s" | "second" | "seconds" => Ok(Time::new::<second>(quantity)),
        _ => Err(UnitError::UnsupportedUnit(unit.to_string()).into()),
    }
}

fn parse_energy(quantity: f32, unit: &str) -> Result<Energy, Box<dyn std::error::Error>> {
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
        _ => Err(UnitError::UnsupportedUnit(unit.to_string()).into()),
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
fn parse_energy_unit(unit: &str) -> Result<EnergyUnit, Box<dyn std::error::Error>> {
    match unit.trim().to_lowercase().as_str() {
        "ev" | "electronvolt" | "electronvolts" => Ok(EnergyUnit::Electronvolt),
        "kev" | "kiloelectronvolt" | "kiloelectronvolts" => Ok(EnergyUnit::Kiloelectronvolt),
        "mev" | "megaelectronvolt" | "megaelectronvolts" => Ok(EnergyUnit::Megaelectronvolt),
        "gev" | "gigaelectronvolt" | "gigaelectronvolts" => Ok(EnergyUnit::Gigaelectronvolt),
        "J" | "joule" | "joules" => Ok(EnergyUnit::Joule),
        _ => Err(UnitError::UnsupportedUnit(unit.to_string()).into()),
    }
}

// Function to parse  time unit input
fn parse_time_unit(unit: &str) -> Result<TimeUnit, Box<dyn std::error::Error>> {
    match unit.trim().to_lowercase().as_str() {
        "ns" | "nanosecond" | "nanoseconds" => Ok(TimeUnit::Nanosecond),
        "mus" | "us" | "microsecond" | "microseconds" => Ok(TimeUnit::Microsecond),
        "ms" | "millisecond" | "milliseconds" => Ok(TimeUnit::Millisecond),
        "s" | "second" | "seconds" => Ok(TimeUnit::Second),
        _ => Err(UnitError::UnsupportedUnit(unit.to_string()).into()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!() // requires `cargo` feature
        .about("Convert neutron energy and neutron time-of-flight") 
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("to_energy")
                .about("Converts neutron time-of-flight to neutron energy")
                .short_flag('E')
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
                        .help("Desired neutron energy units of (eV, KeE, MeV, GeV J)")
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
                .short_flag('T')
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
            let input_length_value: f32 = length_of_flight_path[0].parse::<f32>()?;
            let input_length_unit: &str = &length_of_flight_path[1].trim().to_lowercase();

            // Parse time
            let time_of_flight: Vec<_> = sub_matches
                .get_many::<String>("time_of_flight")
                .unwrap()
                .collect();
            let input_time_value: f32 = time_of_flight[0].parse()?;
            let input_time_unit: &str = &time_of_flight[1].trim().to_lowercase();

            // Parse unit
            let unit = sub_matches.get_one::<String>("unit").unwrap();
            let output_energy_unit: &str = &unit.trim().to_lowercase();

            let length_quantity = parse_length(input_length_value, input_length_unit)?;
            let time_quantity = parse_time(input_time_value, input_time_unit)?;
            let energy_quantity: Energy = calculate_energy(time_quantity, length_quantity)?;

            let energy_unit = parse_energy_unit(output_energy_unit)?;

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
            let input_length_value: f32 = length_of_flight_path[0].parse::<f32>()?;
            let input_length_unit: &str = &length_of_flight_path[1].trim().to_lowercase();

            // Parse time
            let energy: Vec<_> = sub_matches.get_many::<String>("energy").unwrap().collect();
            let input_energy_value: f32 = energy[0].parse()?;
            let input_energy_unit: &str = &energy[1].trim().to_lowercase();

            // Parse unit
            let unit = sub_matches.get_one::<String>("unit").unwrap();
            let output_time_unit: &str = &unit.trim().to_lowercase();

            let length_quantity = parse_length(input_length_value, input_length_unit)?;
            let energy_quantity = parse_energy(input_energy_value, input_energy_unit)?;
            let time_quantity: Time = calculate_time_of_flight(energy_quantity, length_quantity)?;

            let time_unit = parse_time_unit(output_time_unit)?;

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

    Ok(())
}
