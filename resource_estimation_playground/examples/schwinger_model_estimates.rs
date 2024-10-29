use std::fs::File;
use std::io::Write;
use serde_json::json;
use std::rc::Rc;

use resource_estimator::{
    estimates::{ErrorBudget, PhysicalResourceEstimation},
    system::{PhysicalQubit, TFactoryBuilder, Protocol, LogicalResourceCounts},
};

/// Function to calculate the number of qubits and T-gates for the Schwinger model
/// based on Theorem 8 from https://arxiv.org/abs/2002.11146.
fn schwinger_model_params(lambda: u64) -> (u64, u64) {
    // Currently just placeholder values for N (number of spatial lattice sites) and detla_circ (circuit synthesis error)
    let n: u64 = 10; 
    let delta_circ: f64 = 1e-3; 

    // Calculate eta (number of qubits per link register)
    let eta = ((2.0 * lambda as f64).log2().ceil()) as u64;

    // Calculate the number of qubits required using Theorem 8
    let num_qubits = n * (eta + 1) + 4 * eta - (eta as f64).log2().floor() as u64 - 1;

    // Compute ln_term = ln((6N - 5)/detla_circ)
    let ln_term = ((6.0 * n as f64 - 5.0) / delta_circ).ln();

    // Defining terms for lambda_delta calculation
    let term1 = 2.0 * (n as f64 - 1.0) * (96.0 * eta.pow(2) as f64 + 24.0 * (1.0 - eta as f64)) * (eta as f64).log2();
    let term2 = 4.45 * eta as f64 * (3.0 * eta as f64).log2();
    let term3 = (10.35 + 4.45 * eta as f64) * ln_term;
    let term4 = -200.0 * eta as f64 + 133.95;
    let term5 = 1.15 * ((2.0 * (6.0 * n as f64 - 5.0)) / delta_circ).log2();

    // Numerator
    let numerator = term1 + term2 + term3 + term4 + term5;

    // Denominator
    let denominator = n as f64 * eta.pow(2) as f64 + n as f64 * eta as f64 * ln_term;

    // Calculate lambda_delta
    let lambda_delta = numerator / denominator;

    // Compute the number of T-gates required
    let t_gates = ((n as f64) * (eta as f64).powi(2) + (n as f64) * eta as f64 * ln_term) * lambda_delta;

    // Return the number of qubits and T-gates
    (num_qubits, t_gates.ceil() as u64)

}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare a vector to store results for different Hilbert cutoffs
    let mut results = Vec::new();

    // Loop over different values of Hilbert space cutoffs
    for hilbert_cutoff in [10, 20, 50] {
        // Get the number of qubits and T gates for the current cutoff
        let (num_qubits, num_t_gates) = schwinger_model_params(hilbert_cutoff);

        // Define the protocol (quantum error correction code)
        let code = Protocol::floquet_code();

        // Define a qubit model with a specific error rate; we use a Majorana-type qubit as before
        let qubit = Rc::new(PhysicalQubit::qubit_maj_ns_e6());

        // Setup a T factory builder
        let builder = TFactoryBuilder::default();

        // Define logical resource counts with calculated `num_qubits` and `num_t_gates`
        let logical_counts = Rc::new(LogicalResourceCounts {
            num_qubits,
            t_count: num_t_gates,
            rotation_count: 0,
            rotation_depth: 0,
            ccz_count: 0,
            ccix_count: 0,
            measurement_count: 0,
        });

        // Set up a default error budget for resource estimation
        let budget = ErrorBudget::new(0.001, 0.001, 0.001);

        // Create an estimation instance
        let estimation = PhysicalResourceEstimation::new(
            code,
            qubit,
            builder,
            logical_counts,
            budget,
        );

        // Perform the estimation and retrieve the results
        let result = estimation
            .estimate()
            .expect("estimation does not fail");

        // Append each result to the results vector in JSON format
        results.push(json!({
            "hilbert_cutoff": hilbert_cutoff,
            "num_logical_qubits": num_qubits,
            "num_t_gates": num_t_gates,
            "physical_qubits": result.physical_qubits(),
            "runtime_seconds": result.runtime() as f64 / 1e9,
        }));
    }

    // Write all results to a JSON file
    let json_results = json!({ "estimation_results": results });
    let mut file = File::create("./results/schwinger_model_estimates.json")?;
    file.write_all(json_results.to_string().as_bytes())?;

    Ok(())
}
