# Neutron Energy Calculator

This is a simple command-line application written in Rust to calculate the energy of a neutron given the flight path length and the time-of-flight. It uses the `uom` crate for handling units, providing flexibility to work with various unit systems.

## Prerequisites

- Rust programming language (<https://www.rust-lang.org/learn/get-started>)

## Installation

Clone the repository then compile with the `--release` flag with cargo.

```bash
cargo build --release
```

Install the binary with cargo.

```bash
cargo install --path .
```

## How to Use

To run the program, you need to provide the flight path length and the time-of-flight as command-line arguments. You can specify the units for both length and time-of-flight, as well as the desired energy unit for the output.

```bash
tof-calculator -l <LENGTH> <UNIT> -t <TIME> <UNIT>
```

### Command-line Arguments

The program supports the following command-line arguments:

- `--length-of-flight-path` or `-l`: The flight path length with its unit (e.g., `100 meters`, `10 cm`, `2.5 km`).
- `--time-of-flight` or `-t`: The time-of-flight with its unit (e.g., `500 ns`, `2 ms`, `0.05 s`).
- `--unit` or `-u`: The desired unit for the output energy (e.g., `eV`, `MeV`, `Joules`). Default unit is `eV`.

### Examples

1. Calculate neutron energy in electronvolts (default unit):

    ```bash
    tof-calculator -l 1000 meters -t 2 microseconds
    ```

2. Calculate neutron energy in megaelectronvolts:

    ```bash
    tof-calculator -l 50 cm -t 0.05 seconds -u MeV
    ```

3. Calculate neutron energy in joules:

    ```bash
    tof-calculator -l 1.5 km -t 1000 nanoseconds -u Joules
    ```

### Note

The application uses the `uom` crate to handle different units for length, time, and energy. Please make sure to provide valid unit representations in the command-line arguments.

For further information on how to use the program and available options, you can use the `--help` flag:

```bash
tof-calculator --help
```

This will display the usage and available options for the application.

Please note that the provided code snippet is just a part of the actual application, and it assumes the existence of the required dependencies (`clap` and `uom`) in the Cargo.toml file.

The given code is a useful start, and you can extend it to add more features, error handling, or even create a full-fledged CLI tool for neutron energy calculations with various units.
