# resource_estimation_playground

The *resource_estimatiom_playground* directory uses Microsoft Azure's Rust’s API for quantum resource estimation.

## Contents

- **examples/**
  - **example1.rs**: Uses predefined architectures to perform resource estimation by taking logical resource counts from an algorithm defined in a Q# file (*adder.qs*). An small extension of the example given [here](https://github.com/microsoft/qsharp/blob/main/resource_estimator/examples/basic_logical_counts.rs), by using *counter.rs* from [here](https://github.com/Alice-Bob-SW/qsharp-alice-bob-resource-estimator/blob/main/src/counter.rs).
  - **example2.rs**: Defines logical resource counts directly and writes the resource estimation results to a JSON file (*results/example2.json*).
  - **schwinger_model_estimates.rs** Implements a resource estimation for a single timestep of the Schwinger model, based on resource counts from [arXiv:2002.11146](https://arxiv.org/abs/2002.11146) (theorem 8). Writes the resource estimation results to a JSON file (*results/schwinger_model_estimates.json*).

## Usage

1. **Build the Project**: Run the following command to compile the code.
    ```bash
    cargo build
    ```

2. **Run Examples**: Execute a specific example using `cargo run`. For instance, to run *example2*:
    ```bash
    cargo run --example=example2
    ```
   Then use `python results/example2.py` to run the python script.

Note: All examples currently use the Azure pre-defined floquet code and majorana qubits, however this can be changed to use the (also predefined) surface code and superconducting qubits.