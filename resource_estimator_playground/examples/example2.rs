use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use serde_json::json;

use resource_estimator::{
    estimates::{ErrorBudget, PhysicalResourceEstimation},
    system::{PhysicalQubit, TFactoryBuilder, Protocol, LogicalResourceCounts},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup the quantum error correction code, using the Floquet code for this example.
    let code = Protocol::floquet_code();

    // Define a qubit model with a specific error rate; here, we use a Majorana-type qubit.
    let qubit = Rc::new(PhysicalQubit::qubit_maj_ns_e6());

    // Setup a T factory builder to handle state distillation rounds for the estimation task.
    let builder = TFactoryBuilder::default();

    // Define logical resource counts with parameters that specify the quantum operations needed.
    let logical_counts = Rc::new(LogicalResourceCounts {
        num_qubits: 100,
        t_count: 10,
        rotation_count: 10,
        rotation_depth: 5,
        ccz_count: 100,
        ccix_count: 0,
        measurement_count: 10,
    });

    // Set up an error budget for resource estimation.
    let budget = ErrorBudget::new(0.001, 0.001, 0.001);

    // Create an estimation instance with the specified setup.
    let estimation = PhysicalResourceEstimation::new(code, qubit, builder, logical_counts, budget);

    // Perform the estimation and retrieve the results.
    let result = estimation
        .estimate()
        .expect("estimation does not fail");

    // Prepare the results in JSON format for analysis in Python.
    let json_result = json!({
        "physical_qubits": result.physical_qubits(),
        "runtime_seconds": result.runtime() as f64 / 1e9
    });
    
    // Write the results to a JSON file in the results directory for easy integration with the Python script.
    let mut file = File::create("./results/example2.json")?;

    file.write_all(json_result.to_string().as_bytes())?;

    Ok(())
}
