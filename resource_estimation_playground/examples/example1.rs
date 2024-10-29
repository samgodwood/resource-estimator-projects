use std::rc::Rc;

use resource_estimator::{
    estimates::{ErrorBudget, PhysicalResourceEstimation},
    system::{PhysicalQubit, TFactoryBuilder, Protocol},
};

use qsharp_alice_bob_resource_estimator::LogicalCounts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // There are 5 ingredients that we need to perform resource estimation.

    // 1) A quantum error correction code; in this example we are using a
    //    Floquet code.
    let code = Protocol::floquet_code();

    // 2) A qubit model; in this example we are using a Majorana type qubit
    //    using a physical error rate of 1e-6.
    let qubit = Rc::new(PhysicalQubit::qubit_maj_ns_e6());

    // 3) A factory builder to provide magic states; in this example we are
    //    using a T factory builder that can create T factories using multiple
    //    distillation rounds.
    let builder = TFactoryBuilder::default();

    // 4) The logical resource overhead taken from a qsharp file;
    let filename = format!("{}/qsharp/adder.qs", env!("CARGO_MANIFEST_DIR"));
    let logical_counts = Rc::new(
        LogicalCounts::from_qsharp(filename).map_err(|e| format!("Error: {}", e))?
    );

    // 5) An error budget; in this example we are using a uniform error budget
    //    of 0.1% distributed uniformly among logical errors, rotation synthesis
    //    errors, and T state production errors.
    let budget = ErrorBudget::new(0.001, 0.001, 0.001);

    // After we have set up all required inputs for the resource estimation
    // task, we can set up an estimation instance.
    let estimation = PhysicalResourceEstimation::new(code, qubit, builder, logical_counts, budget);

    // In this example, we perform a standard estimation without any further
    // constraints.
    let result = estimation
        .estimate()
        .expect("estimation does not fail");

    // There is a lot of data contained in the resource estimation result
    // object, but in this sample we are only printing the total number of
    // physical qubits and the runtime in seconds (the value is returned in nano
    // seconds).
    println!("Number of physical qubits: {}", result.physical_qubits());
    println!(
        "Runtime:                   {:.2e} ns",
        result.runtime() as f64 / 1e9
    );

    Ok(())
}
