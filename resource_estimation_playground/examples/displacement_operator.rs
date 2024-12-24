// Use build frontier for a bosonic displacement operator using qubits.
// We will calculate the Rz gate and Rz depth required for a fixed
// bosonic hilbert space cutoff.
// We will also calculate the error budget needed to match the fidelity
// of a bosonic operation on oscilator-based hardware with gate time = 5ns.

use std::fs::File;
use std::io::Write;
use serde_json::json;
use std::rc::Rc;

use resource_estimator::{
    estimates::{ErrorBudget, PhysicalResourceEstimation},
    system::{PhysicalQubit, TFactoryBuilder, Protocol, LogicalResourceCounts},
};

// Function to calculate the number of Rz gates, Rz gate depth, and number of qubits based on cutoff
fn displacement_params(cutoff: usize) -> (usize, usize, usize) {
    let rz_gates = (5.94 * (cutoff as f64).powf(1.16)).round() as usize;
    let rz_depth = (4.63 * (cutoff as f64).powf(1.16)).round() as usize;
    let n_qubits = ((cutoff + 1) as f64).log2().ceil() as usize;
    (rz_gates, rz_depth, n_qubits)
}

// Function to calculate fidelity based on photon loss rate with gate time = 5ns
fn fidelity(photon_loss: f64) -> f64 {
    -61.3842 * (0.0840 * photon_loss).exp() + 62.3847
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare a vector to store results for different scenarios
    let mut all_results = Vec::new();

    // Loop over different values of Hilbert space cutoffs
    for hilbert_cutoff in [10, 20, 50] {
        let (rz_gates, rz_depth, n_qubits) = displacement_params(hilbert_cutoff);

        let code = Protocol::surface_code_gate_based();
        let qubit = Rc::new(PhysicalQubit::qubit_gate_ns_e3());
        let builder = TFactoryBuilder::default();

        let logical_counts = Rc::new(LogicalResourceCounts {
            num_qubits: n_qubits as u64,
            t_count: 0,
            rotation_count: rz_gates as u64,
            rotation_depth: rz_depth as u64,
            ccz_count: 0,
            ccix_count: 0,
            measurement_count: 0,
        });

        let budget = ErrorBudget::new(0.01, 0.01, 0.01);
        let estimation = PhysicalResourceEstimation::new(
            code,
            qubit,
            builder,
            logical_counts.clone(),
            budget,
        );

        let frontier = estimation
            .build_frontier()
            .expect("frontier exploration does not fail");

        let mut frontier_results = Vec::new();
        for result in frontier {
            frontier_results.push(json!({
                "physical_qubits": result.physical_qubits(),
                "runtime_seconds": result.runtime() as f64 / 1e9,
            }));
        }

        all_results.push(json!({
            "hilbert_cutoff": hilbert_cutoff,
            "rz_gates": rz_gates,
            "rz_depth": rz_depth,
            "num_logical_qubits": n_qubits,
            "frontier_results": frontier_results,
        }));
    }

    // Fixed cutoff and varying photon loss rates
    let fixed_cutoff = 50;
    let (rz_gates, rz_depth, n_qubits) = displacement_params(fixed_cutoff);
    let mut photon_loss_results = Vec::new();

    for photon_loss_rate in [0.001, 0.005, 0.01, 0.02] {
        let fidelity_value = fidelity(photon_loss_rate);
        let error_budget = 1.0 - fidelity_value;
        let code = Protocol::surface_code_gate_based();
        let qubit = Rc::new(PhysicalQubit::qubit_gate_ns_e3());
        let builder = TFactoryBuilder::default();

        let logical_counts = Rc::new(LogicalResourceCounts {
            num_qubits: n_qubits as u64,
            t_count: 0,
            rotation_count: rz_gates as u64,
            rotation_depth: rz_depth as u64,
            ccz_count: 0,
            ccix_count: 0,
            measurement_count: 0,
        });

        let budget = ErrorBudget::new(error_budget, error_budget, error_budget);
        let estimation = PhysicalResourceEstimation::new(
            code,
            qubit,
            builder,
            logical_counts.clone(),
            budget,
        );

        let frontier = estimation
            .build_frontier()
            .expect("frontier exploration does not fail");

        let mut frontier_results = Vec::new();
        for result in frontier {
            frontier_results.push(json!({
                "physical_qubits": result.physical_qubits(),
                "runtime_seconds": result.runtime() as f64 / 1e9,
            }));
        }

        photon_loss_results.push(json!({
            "photon_loss_rate": photon_loss_rate,
            "fidelity": fidelity_value,
            "error_budget": error_budget,
            "frontier_results": frontier_results,
        }));
    }

    // Combine results and write to JSON
    let json_results = json!({
        "hilbert_cutoff_results": all_results,
        "photon_loss_results": photon_loss_results,
    });

    let mut file = File::create("./results/displacement_operator/displacement_operator.json")?;
    file.write_all(json_results.to_string().as_bytes())?;

    println!("Results written to ./results/displacement_operator/displacement_operator.json");

    Ok(())
}

