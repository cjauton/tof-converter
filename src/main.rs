#![allow(unused)]

use clap::Parser;
use uom::si::f32::*;
use uom::si::length::{kilometer,meter,centimeter};
use uom::si::time::{second, millisecond, microsecond, nanosecond};
use uom::si::energy::{electronvolt,kiloelectronvolt,megaelectronvolt,gigaelectronvolt,joule};
use uom::si::mass::kilogram;
use uom::fmt::DisplayStyle::Abbreviation;


fn calculate_energy( time: Time, length: Length ) -> Energy {

    let m = uom::si::f32::Mass::new::<kilogram>(1.67493e-27_f32);
    let energy: Energy = m * (length * length) / (2.0 *time * time);
    // energy.get::<electronvolt>().into()
    energy
    
}

fn parse_length(quantity: f32, unit: &str) -> Length {
    match unit.trim().to_lowercase().as_str() {
        "cm" | "centimeter" | "centimeters" => Length::new::<centimeter>(quantity),
        "m" | "meter" | "meters" => Length::new::<meter>(quantity),
        "km" | "kilometer" | "kilometers" => Length::new::<kilometer>(quantity),
        _ => Length::new::<meter>(quantity),
    }
}

fn parse_time(quantity: f32, unit: &str) -> Time {
    match unit.trim().to_lowercase().as_str() {
        "ns" | "nanosecond" | "nanoseconds" => Time::new::<nanosecond>(quantity),
        "mus" | "us" | "microsecond" | "microseconds" => Time::new::<microsecond>(quantity),
        "ms" | "millisecond" | "milliseconds" => Time::new::<millisecond>(quantity),
        "s" | "second" | "seconds" => Time::new::<second>(quantity),
        _ => Time::new::<second>(quantity),
    }
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
    // let input_length_value: f32 = cli.length_fp[0].parse::<f32>().unwrap();

    let input_time_unit: &str = &cli.tof[1].trim().to_lowercase();
    // let input_time_value: f32 = cli.tof[0].parse::<f32>().unwrap();
    
    let length_quantity = parse_length(input_length_value, input_length_unit);
    let time_quantity = parse_time(input_time_value, input_time_unit);



    let output_energy_unit: &str = &cli.unit.unwrap().trim().to_lowercase();

    let energy_quantity: Energy = calculate_energy(time_quantity,length_quantity);

    match output_energy_unit {
        "ev" | "electronvolt" | "electronvolts" =>  println!("Energy = {}", energy_quantity.into_format_args(electronvolt,Abbreviation)),
        "kev" | "kiloelectronvolt" | "kiloelectronvolts" =>  println!("Energy = {}", energy_quantity.into_format_args(kiloelectronvolt,Abbreviation)),
        "mev" | "megaelectronvolt" | "megaelectronvolts" =>  println!("Energy = {}", energy_quantity.into_format_args(megaelectronvolt,Abbreviation)),
        "gev" | "gigaelectronvolt" | "gigaelectronvolts" =>  println!("Energy = {}", energy_quantity.into_format_args(gigaelectronvolt,Abbreviation)),
        "j" | "joule" | "joules" =>  println!("Energy = {}", energy_quantity.into_format_args(joule,Abbreviation)),
        _ => println!("Energy = {}", energy_quantity.into_format_args(electronvolt,Abbreviation)),

    }

}



