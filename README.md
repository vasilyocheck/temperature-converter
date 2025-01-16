# Temperature Converter

This is a simple command-line temperature converter written in Rust. It allows you to convert temperatures between Fahrenheit and Celsius.

## Features

- Convert Fahrenheit to Celsius
- Convert Celsius to Fahrenheit
- User-friendly command-line interface
- Input validation

## Requirements

- Rust programming language (stable version)

## Installation

1. Ensure you have Rust installed. If not, you can install it from [rustup.rs](https://rustup.rs/).
2. Clone the repository or download the source code.
3. Navigate to the project directory.

## Usage

1. Compile the program using Cargo:
    ```sh
    cargo build --release
    ```
2. Run the compiled binary:
    ```sh
    ./target/release/temperature_converter
    ```
3. Follow the on-screen instructions to convert temperatures.

## Example

```sh
Hi, this is a temperature converter!
If you want to convert Fahrenheit to Celsius input 1, and if Celsius to Fahrenheit - input 2:
Enter your choice(1 or 2): 1
We are going to convert Fahrenheit to Celsius
Please input the temperature in Fahrenheit to convert to Celsius: 100
The temperature is 37.78 degrees Celsius.
