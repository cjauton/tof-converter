#![allow(unused)]

use clap::Parser;
use uom::si::f32::*;
use uom::si::length::{kilometer,meter,centimeter};
use uom::si::time::{second, millisecond, microsecond, nanosecond};
use uom::si::energy::{electronvolt,kiloelectronvolt,megaelectronvolt,gigaelectronvolt,joule};
use uom::si::mass::kilogram;
use uom::fmt::DisplayStyle::Abbreviation;

// // fn convert_to_centimeters(value: Length) -> Length {
// //     value.get::<centimeter>()
// // }

// // fn convert_to_meters(value: Length) -> Length {
// //     value.get::<meter>()
// // }

fn calculate_energy( time: Time, length: Length ) -> Energy {

    let m = uom::si::f32::Mass::new::<kilogram>(1.67493e-27_f32);
    let energy: Energy = m * (length * length) / (2.0 *time * time);
    // energy.get::<electronvolt>().into()
    energy
    
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

    let input_unit_length: &str = &cli.length_fp[1].trim().to_lowercase();
    let input_unit_time: &str = &cli.tof[1].trim().to_lowercase();

    let l = match input_unit_length{
        "cm" | "meter" | "meters" => uom::si::f32::Length::new::<centimeter>(cli.length_fp[0].parse::<f32>().unwrap()),
        "m" | "meter" | "meters" => uom::si::f32::Length::new::<meter>(cli.length_fp[0].parse::<f32>().unwrap()),
        "km" | "kilometer" | "kilometers" => uom::si::f32::Length::new::<kilometer>(cli.length_fp[0].parse::<f32>().unwrap()),
        _ =>  uom::si::f32::Length::new::<meter>(cli.length_fp[0].parse::<f32>().unwrap()),

    };

    let t = match input_unit_time{
        "ns" | "nanosecond" | "nanoseconds" => uom::si::f32::Time::new::<nanosecond>(cli.tof[0].parse::<f32>().unwrap()),
        "mus" | "us" | "microsecond" | "microseconds" => uom::si::f32::Time::new::<microsecond>(cli.tof[0].parse::<f32>().unwrap()),
        "ms" | "millisecond" | "milliseconds" => uom::si::f32::Time::new::<millisecond>(cli.tof[0].parse::<f32>().unwrap()),
        "s" | "second" | "seconds" => uom::si::f32::Time::new::<second>(cli.tof[0].parse::<f32>().unwrap()),
        _ =>  uom::si::f32::Time::new::<second>(cli.tof[0].parse::<f32>().unwrap()),

    };


    let out_unit: &str = &cli.unit.unwrap().trim().to_lowercase();


    let out_energy: Energy = calculate_energy(t,l);

    match out_unit {
        "ev" | "electronvolt" | "electronvolts" =>  println!("Energy = {}", out_energy.into_format_args(electronvolt,Abbreviation)),
        "kev" | "kiloelectronvolt" | "kiloelectronvolts" =>  println!("Energy = {}", out_energy.into_format_args(kiloelectronvolt,Abbreviation)),
        "mev" | "megaelectronvolt" | "megaelectronvolts" =>  println!("Energy = {}", out_energy.into_format_args(megaelectronvolt,Abbreviation)),
        "gev" | "gigaelectronvolt" | "gigaelectronvolts" =>  println!("Energy = {}", out_energy.into_format_args(gigaelectronvolt,Abbreviation)),
        "j" | "joule" | "joules" =>  println!("Energy = {}", out_energy.into_format_args(joule,Abbreviation)),
        _ => println!("Energy = {}", out_energy.into_format_args(electronvolt,Abbreviation)),

    }

    // println!("Energy = {}", out_energy)

    // println!("Energy = {}", calculate_energy(t,l).into_format_args(electronvolt,Abbreviation));

    // println!("length = {:?}, TOF = {:?}", l,t);
    // println!("length = {} {}, TOF = {} {}", cli.length_fp[0].parse::<f64>().unwrap(), cli.length_fp[1], cli.tof[0].parse::<f64>().unwrap(), cli.tof[1]);
}



