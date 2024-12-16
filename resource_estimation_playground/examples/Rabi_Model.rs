// Use build frontier for a single Trotter step of a spin-boson Hamiltonian with n_max = 3
// see https://github.com/samgodwood/bosonic-qiskit/blob/main/playground/Rabi_Model/Trotter_dynamics.ipynb


use std::fs::File;
use std::io::Write;
use serde_json::json;
use std::rc::Rc;

use resource_estimator::{
    estimates::{ErrorBudget, PhysicalResourceEstimation},
    system::{PhysicalQubit, TFactoryBuilder, Protocol, LogicalResourceCounts},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare a vector to store results for the frontier exploration.
    let mut frontier_results = Vec::new();

    // Define a single error budget value.
    let error_budget_value = 0.01;

    // Choose the surface code as our QEC code
    let code = Protocol::surface_code_gate_based();

    // Choose a superconducting qubit
    let qubit = Rc::new(PhysicalQubit::qubit_gate_ns_e3());

    // Setup a T factory builder to handle state distillation rounds for the estimation task.
    let builder = TFactoryBuilder::default();

    // Define logical resource counts with a fixed number of qubits.
    let logical_counts = Rc::new(LogicalResourceCounts {
        num_qubits: 3,
        t_count: 0,
        rotation_count: 20,
        rotation_depth: 12,
        ccz_count: 0,
        ccix_count: 0,
        measurement_count: 0,
    });

    // Set up an error budget for the resource estimation.
    let budget = ErrorBudget::new(error_budget_value, error_budget_value, error_budget_value);

    // Create an estimation instance with the specified setup.
    let estimation = PhysicalResourceEstimation::new(
        code,      // Passing instance of `code`
        qubit,     // Passing instance of `qubit`
        builder,   // Passing instance of `builder`
        logical_counts, // Passing instance of `logical_counts`
        budget,    // Passing instance of `budget`
    );

    // Perform the frontier exploration and retrieve the results.
    let frontier = estimation
        .build_frontier()
        .expect("frontier exploration does not fail");

    // Process and collect the frontier results.
    for result in frontier {
        frontier_results.push(json!({
            "physical_qubits": result.physical_qubits(),
            "runtime_seconds": result.runtime() as f64 / 1e9,
        }));
    }

    // Write all frontier results to a JSON file in the results directory.
    let json_results = json!({ "frontier_results": frontier_results });
    let mut file = File::create("./results/Rabi_Model/Rabi_Model.json")?;
    file.write_all(json_results.to_string().as_bytes())?;

    println!("Frontier results written to ./results/Rabi_Model/Rabi_Model.json");

    Ok(())
}