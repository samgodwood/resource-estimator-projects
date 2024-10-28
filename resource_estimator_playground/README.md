# resource_estimator_playground

The *resource_estimator_playground* directory uses Microsoft Azure's Rust’s API for quantum resource estimation.

## Contents

- **examples/**
  - **example1.rs**: Uses predefined architectures to perform resource estimation by taking logical resource counts from an algorithm defined in a Q# file (*adder.qs*). An extension of the example given [here](https://github.com/microsoft/qsharp/blob/main/resource_estimator/examples/basic_logical_counts.rs).
  - **example2.rs**: Defines logical resource counts directly and writes the resource estimation results to a JSON file (*results/example2.json*) for further analysis.
- **results/**
  - **example2.json**: JSON output from *example2.rs* containing resource estimation data.
  - **example2.py**: A Python script for analyzing the JSON data produced by *example2.rs*.

## Usage

1. **Build the Project**: Run the following command to compile the code.
    ```bash
    cargo build
    ```

2. **Run Examples**: Execute a specific example using `cargo run`. For instance, to run *example1*:
    ```bash
    cargo run --example example1
    ```
   To run *example2* and output results to JSON:
    ```bash
    cargo run --example example2
    ```
