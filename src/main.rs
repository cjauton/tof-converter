use clap::Parser;
use uom::si::f32::*;
use uom::si::length::{kilometer,meter,centimeter};
use uom::si::time::{second, millisecond, microsecond, nanosecond};
use uom::si::energy::{electronvolt,kiloelectronvolt,megaelectronvolt,gigaelectronvolt,joule};
use uom::si::mass::kilogram;
use uom::fmt::DisplayStyle::Abbreviation;
use std::error::Error;
use std::fmt;

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

impl Error for UnsupportedUnitError {}


fn calculate_energy( time: Time, length: Length ) -> Energy {

    let m = uom::si::f32::Mass::new::<kilogram>(1.67493e-27_f32);
    let energy: Energy = m * (length * length) / (2.0 *time * time);

    energy
    
}


// Functions to parse length and time inputs
fn parse_length(quantity: f32, unit: &str) -> Result<Length, UnsupportedUnitError> {
    match unit.trim().to_lowercase().as_str() {
        "cm" | "centimeter" | "centimeters" => Ok(Length::new::<centimeter>(quantity)),
        "m" | "meter" | "meters" => Ok(Length::new::<meter>(quantity)),
        "km" | "kilometer" | "kilometers" => Ok(Length::new::<kilometer>(quantity)),
        _ => Err(UnsupportedUnitError::Length(unit.to_string()).into()),
    }
}


fn parse_time(quantity: f32, unit: &str) -> Result<Time, UnsupportedUnitError> {
    match unit.trim().to_lowercase().as_str() {
        "ns" | "nanosecond" | "nanoseconds" => Ok(Time::new::<nanosecond>(quantity)),
        "mus" | "us" | "microsecond" | "microseconds" => Ok(Time::new::<microsecond>(quantity)),
        "ms" | "millisecond" | "milliseconds" => Ok(Time::new::<millisecond>(quantity)),
        "s" | "second" | "seconds" => Ok(Time::new::<second>(quantity)),
        _ => Err(UnsupportedUnitError::Time(unit.to_string()).into()),
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


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// Flight path length from source to target in units of (cm, m, km).
    #[arg(short,long, num_args(2), value_names = ["LENGTH","UNIT"])]
    length_fp: Vec<String>,

    /// Time-of-Flight to convert to neutron Energy in units of (ns, us, ms, s).
    #[arg(short,long, num_args(2), value_names = ["TOF","UNIT"])]
    tof: Vec<String>,

    /// Desired neutron energy units of (eV, KeE, MeV, J).
    #[arg(short,long, num_args(1), default_value = "eV")]
    unit: Option<String>
}

// Function to parse energy unit input
fn parse_energy_unit(unit: &str) -> Result<EnergyUnit, UnsupportedUnitError> {
    match unit.trim().to_lowercase().as_str() {
        "ev" | "electronvolt" | "electronvolts" => Ok(EnergyUnit::Electronvolt),
        "kev" | "kiloelectronvolt" | "kiloelectronvolts" => Ok(EnergyUnit::Kiloelectronvolt),
        "mev" | "megaelectronvolt" | "megaelectronvolts" => Ok(EnergyUnit::Megaelectronvolt),
        "gev" | "gigaelectronvolt" | "gigaelectronvolts" => Ok(EnergyUnit::Gigaelectronvolt),
        "j" | "joule" | "joules" => Ok(EnergyUnit::Joule),
        _ => Err(UnsupportedUnitError::Energy(unit.to_string()).into()),
    }
}

fn main() {
    let cli = Cli::parse();

    // Parse length value
    let input_length_value: f32 = match cli.length_fp[0].parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error parsing length value: {}", err);
            return;
        }
    };

    // Parse time value
    let input_time_value: f32 = match cli.tof[0].parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error parsing time value: {}", err);
            return;
        }
    };


    let input_length_unit: &str = &cli.length_fp[1].trim().to_lowercase();
    let input_time_unit: &str = &cli.tof[1].trim().to_lowercase();

    let length_quantity = match parse_length(input_length_value, input_length_unit) {
        Ok(length) => length,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let time_quantity = match parse_time(input_time_value, input_time_unit) {
        Ok(time) => time,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };


    let output_energy_unit: &str = &cli.unit.unwrap().trim().to_lowercase();

    let energy_quantity: Energy = calculate_energy(time_quantity,length_quantity);

    let energy_unit = match parse_energy_unit(output_energy_unit) {
        Ok(unit) => unit,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    match energy_unit {
        EnergyUnit::Electronvolt => println!("Energy = {}", energy_quantity.into_format_args(electronvolt, Abbreviation)),
        EnergyUnit::Kiloelectronvolt => println!("Energy = {}", energy_quantity.into_format_args(kiloelectronvolt, Abbreviation)),
        EnergyUnit::Megaelectronvolt => println!("Energy = {}", energy_quantity.into_format_args(megaelectronvolt, Abbreviation)),
        EnergyUnit::Gigaelectronvolt => println!("Energy = {}", energy_quantity.into_format_args(gigaelectronvolt, Abbreviation)),
        EnergyUnit::Joule => println!("Energy = {}", energy_quantity.into_format_args(joule, Abbreviation)),
    }

}