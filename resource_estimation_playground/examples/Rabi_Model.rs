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

// Function to calculate the number of Rz gates, Rz gate depth, and number of qubits based on cutoff
fn rabi_model_params(cutoff: usize) -> (usize, usize, usize) {
    let rz_gates = (6.59 * (cutoff as f64).powf(1.15)).round() as usize;
    let rz_depth = (5.15 * (cutoff as f64).powf(1.15)).round() as usize;
    let n_qubits = ((cutoff + 1) as f64).log2().ceil() as usize + 1; // Calculate n_qubits based on n_max (cutoff)
    (rz_gates, rz_depth, n_qubits)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare a vector to store results for different Hilbert cutoffs
    let mut all_results = Vec::new();

    // Loop over different values of Hilbert space cutoffs
    for hilbert_cutoff in [10, 20, 50, 100, 200, 500] {
        // Get the number of Rz gates, Rz gate depth, and number of qubits for the current cutoff
        let (rz_gates, rz_depth, n_qubits) = rabi_model_params(hilbert_cutoff);

        // Choose the surface code as our QEC code
        let code = Protocol::surface_code_gate_based();

        // Choose a superconducting qubit
        let qubit = Rc::new(PhysicalQubit::qubit_gate_ns_e3());

        // Setup a T factory builder
        let builder = TFactoryBuilder::default();

        // Define logical resource counts using the calculated parameters
        let logical_counts = Rc::new(LogicalResourceCounts {
            num_qubits: n_qubits as u64,
            t_count: 0,    // No T gates in this model
            rotation_count: rz_gates as u64,
            rotation_depth: rz_depth as u64,
            ccz_count: 0,
            ccix_count: 0,
            measurement_count: 0,
        });

        // Set up an error budget for resource estimation
        let budget = ErrorBudget::new(0.01, 0.01, 0.01);

        // Create an estimation instance
        let estimation = PhysicalResourceEstimation::new(
            code,
            qubit,
            builder,
            logical_counts,
            budget,
        );

        // Perform Pareto frontier estimation
        let frontier = estimation
            .build_frontier()
            .expect("frontier exploration does not fail");

        // Collect the frontier results for this Hilbert cutoff
        let mut frontier_results = Vec::new();
        for result in frontier {
            frontier_results.push(json!({
                "physical_qubits": result.physical_qubits(),
                "runtime_seconds": result.runtime() as f64 / 1e9,
            }));
        }

        // Append the results to the overall results
        all_results.push(json!({
            "hilbert_cutoff": hilbert_cutoff,
            "rz_gates": rz_gates,
            "rz_depth": rz_depth,
            "num_logical_qubits": n_qubits,
            "frontier_results": frontier_results,
        }));
    }

    // Write all results to a JSON file
    let json_results = json!({ "pareto_estimation_results": all_results });
    let mut file = File::create("./results/Rabi_Model/Rabi_Model.json")?;
    file.write_all(json_results.to_string().as_bytes())?;

    println!("Results written to ./results/Rabi_Model/Rabi_Model.json");

    Ok(())
}
