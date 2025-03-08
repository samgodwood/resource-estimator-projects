// Use build frontier for a bosonic displacement operator using qubits.
// Compare two methods ("pauli decomp" and "newton iterations") for a fixed bosonic Hilbert space cutoff.
// Removed fidelity dependence. Results are produced for K = 7, 63, 256.
//
// For a given cutoff K, let q = ceil(log₂(K+1)).
// For pauli decomp:
//   rotations = q * 2^(q-1)
//   rotation depth = q * 2^(q-1)
//   logical qubits = q
//
// For newton iterations (with m = 2):
//   rotations = 2q² + 2q
//   rotation depth = 136*q² - 32*q - 56
//   logical qubits = 10*q + 1

use std::fs::File;
use std::io::Write;
use serde_json::json;
use std::rc::Rc;

use resource_estimator::{
    estimates::{ErrorBudget, PhysicalResourceEstimation},
    system::{PhysicalQubit, TFactoryBuilder, Protocol, LogicalResourceCounts},
};

fn pauli_decomp_params(K: usize, max_qubits: usize) -> (usize, usize, usize) {
    let q = ((K + 1) as f64).log2().ceil() as usize;
    let rotations = q * (1 << (q - 1)); // q * 2^(q-1)
    let depth = rotations; // same scaling for depth
    let n_qubits = q; // uses q qubits
    if n_qubits > max_qubits {
        panic!(
            "number of qubits exceeds max for cutoff = {} in pauli decomp",
            K
        );
    }
    (rotations, depth, n_qubits)
}

fn newton_iterations_params(K: usize, max_qubits: usize) -> (usize, usize, usize) {
    let q = ((K + 1) as f64).log2().ceil() as usize;
    let m = 2;
    let rotations = 2 * q * q + 2 * q;
    let depth = 136 * q * q - 32 * q - 56; // 136*q^2 - 32*q - 56
    let n_qubits = 10 * q + 1; // q*(2*m+6)+1 for m = 2
    if n_qubits > max_qubits {
        panic!(
            "number of qubits exceeds max for cutoff = {} in newton iterations",
            K
        );
    }
    (rotations, depth, n_qubits)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare a vector to store results for different scenarios.
    let mut all_results = Vec::new();
    let cutoffs = [7, 63, 256];
    let max_qubits = 100;
    // Use a fixed error budget.

    for &K in &cutoffs {
        // PAULI DECOMP method.
        let (rz_gates, rz_depth, n_qubits) = pauli_decomp_params(K, max_qubits);
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
            logical_counts,
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
            "method": "pauli_decomp",
            "K": K,
            "rotations": rz_gates,
            "rotation_depth": rz_depth,
            "num_logical_qubits": n_qubits,
            "frontier_results": frontier_results,
        }));

        // NEWTON ITERATIONS method.
        let (rz_gates, rz_depth, n_qubits) = newton_iterations_params(K, max_qubits);
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
        let budget2 = ErrorBudget::new(0.01, 0.01, 0.01);

        let estimation = PhysicalResourceEstimation::new(
            code,
            qubit,
            builder,
            logical_counts,
            budget2,
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
            "method": "newton_iterations",
            "K": K,
            "rotations": rz_gates,
            "rotation_depth": rz_depth,
            "num_logical_qubits": n_qubits,
            "frontier_results": frontier_results,
        }));
    }

    // Combine results and write to JSON.
    let json_results = json!({
        "results": all_results,
    });

    let mut file = File::create("./results/displacement_operator/displacement_operator.json")?;
    file.write_all(json_results.to_string().as_bytes())?;

    println!("Results written to ./results/displacement_operator/displacement_operator.json");

    Ok(())
}

