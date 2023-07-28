# Neutron Energy Calculator

This is a simple command-line application written in Rust to calculate the energy of a neutron given the flight path length and the time-of-flight. It uses the `uom` crate for handling units, providing flexibility to work with various unit systems.

## Prerequisites

- Rust programming language (<https://www.rust-lang.org/learn/get-started>)

## How to Use

To run the program, you need to provide the flight path length and the time-of-flight as command-line arguments. You can specify the units for both length and time-of-flight, as well as the desired energy unit for the output.

### Command-line Arguments

The program supports the following command-line arguments:

- `--length_fp` or `-l`: The flight path length with its unit (e.g., `100 meters`, `10 cm`, `2.5 km`).
- `--tof` or `-t`: The time-of-flight with its unit (e.g., `500 ns`, `2 ms`, `0.05 s`).
- `--unit` or `-u`: The desired unit for the output energy (e.g., `eV`, `MeV`, `Joules`). Default unit is `eV`.

### Usage

If the binary is installed or added to path.

```bash
tof-converter [OPTIONS] --l <LENGTH> <UNIT> --t <TOF> <UNIT>
```

### Examples

1. Calculate neutron energy in electronvolts (default unit):

    ```bash
    cargo run -- --length_fp 1000 meters --tof 2 microseconds
    ```

2. Calculate neutron energy in megaelectronvolts:

    ```bash
    cargo run -- --length_fp 50 cm --tof 0.05 seconds --unit MeV
    ```

3. Calculate neutron energy in joules:

    ```bash
    cargo run -- --length_fp 1.5 km --tof 1000 nanoseconds --unit Joules
    ```

### Note

The application uses the `uom` crate to handle different units for length, time, and energy. Please make sure to provide valid unit representations in the command-line arguments.

For further information on how to use the program and available options, you can use the `--help` flag:

```bash
cargo run -- --help

```

This will display the usage and available options for the application.

Please note that the provided code snippet is just a part of the actual application, and it assumes the existence of the required dependencies (`clap` and `uom`) in the Cargo.toml file.

The given code is a useful start, and you can extend it to add more features, error handling, or even create a full-fledged CLI tool for neutron energy calculations with various units.
