use std::fs::File;
use std::io::Write;
use serde_json::json;
use std::rc::Rc;

use resource_estimator::{
    estimates::{ErrorBudget, PhysicalResourceEstimation},
    system::{PhysicalQubit, TFactoryBuilder, Protocol, LogicalResourceCounts},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare a vector to store results for multiple error budgets.
    let mut results = Vec::new();

    // Loop over different values of the logical error budget (e.g., 0.001, 0.01, 0.1)
    for error_budget_value in [0.001, 0.01, 0.1] {
        // Re-create each component needed for the estimation to avoid ownership issues.
        
        // Choose the surface code as our QEC code
        let code = Protocol::surface_code_gate_based();

        // // Choose a superconducting qubit
        let qubit = Rc::new(PhysicalQubit::qubit_gate_ns_e3());

        // Setup a T factory builder to handle state distillation rounds for the estimation task.
        let builder = TFactoryBuilder::default();

        // Define logical resource counts with a fixed number of qubits.
        let logical_counts = Rc::new(LogicalResourceCounts {
            num_qubits: 100,
            t_count: 10,
            rotation_count: 10,
            rotation_depth: 5,
            ccz_count: 100,
            ccix_count: 0,
            measurement_count: 10,
        });

        // Set up an error budget for the resource estimation using the current budget value.
        let budget = ErrorBudget::new(error_budget_value, error_budget_value, error_budget_value);

        // Create an estimation instance with the specified setup, passing fresh instances each time.
        let estimation = PhysicalResourceEstimation::new(
            code,      // Passing fresh instance of `code`
            qubit,     // Passing fresh instance of `qubit`
            builder,   // Passing fresh instance of `builder`
            logical_counts, // Passing fresh instance of `logical_counts`
            budget,    // Passing fresh instance of `budget`
        );

        // Perform the estimation and retrieve the results.
        let result = estimation
            .estimate()
            .expect("estimation does not fail");

        // Append each result to the results vector in JSON format.
        results.push(json!({
            "logical_error_budget": error_budget_value,
            "physical_qubits": result.physical_qubits(),
            "runtime_seconds": result.runtime() as f64 / 1e9
        }));
    }

    // Write all results to a JSON file in the results directory.
    let json_results = json!({ "estimation_results": results });
    let mut file = File::create("./results/example2.json")?;
    file.write_all(json_results.to_string().as_bytes())?;

    Ok(())
}

