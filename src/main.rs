use clap::Parser;


// // fn convert_to_centimeters(value: Length) -> Length {
// //     value.get::<centimeter>()
// // }

// // fn convert_to_meters(value: Length) -> Length {
// //     value.get::<meter>()
// // }


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// Flight path length from source to target.
    #[arg(short,long, num_args(2), value_names = ["LENGTH","UNIT"])]
    length: Vec<String>,

    /// Time-of-Flight to convert to neutron Energy.
    #[arg(short,long, num_args(2), value_names = ["TIME","UNIT"])]
    time: Vec<String>,

    /// Desired neutron energy units.
    #[arg(short,long, num_args(1))]
    unit: Option<String>
}

fn main() {
    let cli = Cli::parse();

    println!("length = {} {}, TOF = {} {}", cli.length[0].parse::<f64>().unwrap(), cli.length[1], cli.time[0].parse::<f64>().unwrap(), cli.time[1]);
}



