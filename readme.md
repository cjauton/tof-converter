# Neutron TOF Converter

This is a simple command-line application written in Rust to convert between energy and time-of-flight for non-relativistic neutrons given the flight-path-length. The tool use `clap` for command line parsing and the `uom` crate for handling units, providing flexibility to work with various common units.

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

### Converting to Neutron Energy

Use the `to_energy` or `-E` subcommand to convert from time-of-flight to energy by providing the flight-path-length and the time-of-flight.

```bash
tof-convert to_energy -l <LENGTH> <UNIT> -t <TIME> <UNIT>
```

The program supports the following command-line arguments:

- `--length-of-flight-path` or `-l`: The flight path length with its unit (`cm`, `m`, `km`).
- `--time-of-flight` or `-t`: The time-of-flight with its unit (`ns`, `us`, `ms`, `s`).
- `--unit` or `-u`: The desired unit for the output energy (`eV`,`keV`, `MeV`, `GeV`, `Js`). Default unit is `eV`.

### Converting to Neutron Time-of-Flight

Use the `to_tof` or `-T` subcommand to convert from energy to time-of-flight by providing the flight-path-length and the energy.

```bash
tof-convert to_tof -l <LENGTH> <UNIT> -e <ENERGY> <UNIT>
```

The program supports the following command-line arguments:

- `--length-of-flight-path` or `-l`: The flight path length with its unit (`cm`, `m`, `km`).
- `--energy` or `-e`: The energy with its unit (`eV`,`keV`, `MeV`, `GeV`, `Js`).
- `--unit` or `-u`: The desired unit for the output energy (`ns`, `us`, `ms`, `s`). Default unit is `us`.

### Examples

1. Calculate neutron energy in electronvolts (default unit):

    ```bash
    tof-convert to_energy -l 1000 meters -t 2 microseconds
    ```

2. Calculate neutron time-of-flight in nanoseconds:

    ```bash
    tof-convert -T -l 50 cm -t 20 eV -u ns
    ```

3. Calculate neutron energy in joules:

    ```bash
    tof-convert -E -l 1.5 km -t 1000 nanoseconds -u Joules
    ```

### Note

The application uses the `uom` crate to handle different units for length, time, and energy. Please make sure to provide valid unit representations in the command-line arguments.

For further information on how to use the program and available options, you can use the `--help` or `-h` flag:

```bash
tof-convert --help
```

This will display the usage and available options for the application.
