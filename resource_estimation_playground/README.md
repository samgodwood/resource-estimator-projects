# resource_estimation_playground

The *resource_estimatiom_playground* directory uses Microsoft Azure's Rust’s API for quantum resource estimation.

## Contents

- **examples/**
  - **example1.rs**: Uses predefined architectures to perform resource estimation by taking logical resource counts from an algorithm defined in a Q# file (*adder.qs*). An small extension of the example given [here](https://github.com/microsoft/qsharp/blob/main/resource_estimator/examples/basic_logical_counts.rs), by using *counter.rs* from [here](https://github.com/Alice-Bob-SW/qsharp-alice-bob-resource-estimator/blob/main/src/counter.rs).
  - **example2.rs**: Defines logical resource counts directly and writes the resource estimation results to a JSON file (*results/example2.json*).
  - **example3.rs**: Performs Pareto frontier estimation using defined logical resource counts and writes results to a JSON file (*results/example3.json*).
  - **schwinger_model_estimates.rs** Implements a resource estimation for a single timestep of the Schwinger model, based on resource counts from [arXiv:2002.11146](https://arxiv.org/abs/2002.11146) (theorem 8). Writes the resource estimation results to a JSON file (*results/schwinger_model_estimates.json*).
  - **Rabi_Model.rs** Uses build frontier for to calculate the physical resource requirements for a  single Trotter step of a simple spin-boson Hamiltonian with a bosonic cutoff $n_{max} = 3$.
- **beamsplitter.rs and displacement_operator.rs**: Calculates physical resource requirements for the beamsplitter and bosonic displacement operator using a build frontier approach, for:
    1. Implementing the operator on all-qubit hardware across various bosonic cutoffs.
    2. Implementing the operator for a fixed cutoff with an error budget to match the error-rate of oscillator-qubit hardware under a specific photon loss rate.



## Usage

1. **Build the Project**: Compile the code with the following command:
    ```bash
    cargo build
    ```

2. **Run Examples**: Use the `Makefile` to automate running Rust examples and their corresponding Python scripts. For instance:

    - To run the `beamsplitter` example:
      ```bash
      make beamsplitter
      ```
    - To run the `displacement_operator` example:
      ```bash
      make displacement_operator
      ```

   The `Makefile` automates both running the Rust example and executing the corresponding Python script to generate the plots.
    ```

### **Notes:**
- All examples currently use the Azure pre-defined surface code and superconducting qubits.
- Results are saved in the `results/<example_name>/` directory, and plots are generated as `.pdf` files.
