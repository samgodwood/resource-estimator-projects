#![warn(missing_docs)]
//! Estimate the resources required for the Schwinger Model on a
//! cat-based quantum processor.
//!
//! Uses logical counts based on Schwinger model parameterization from 
//! <https://arxiv.org/abs/2002.11146> and Alice & Bob's resource estimation framework.

use std::rc::Rc;

use qsharp_alice_bob_resource_estimator::{
    AliceAndBobEstimates, CatQubit, LogicalCounts, RepetitionCode, ToffoliBuilder,
};
use resource_estimator::estimates::{ErrorBudget, PhysicalResourceEstimation};

/// Compute logical qubits and logical gate counts for the Schwinger Model.
/// Based on <https://arxiv.org/abs/2002.11146>, Theorem 8.
fn schwinger_model_params(lambda: u64) -> LogicalCounts {
    // Placeholder values for N (number of spatial lattice sites) and delta_circ (circuit synthesis error)
    let n: u64 = 10;
    let delta_circ: f64 = 1e-3;

    // Calculate eta (number of qubits per link register)
    let eta = ((2.0 * lambda as f64).log2().ceil()) as u64;

    // Calculate the number of qubits required using Theorem 8
    let num_qubits = n * (eta + 1) + 4 * eta - (eta as f64).log2().floor() as u64 - 1;

    // Compute ln_term = ln((6N - 5)/delta_circ)
    let ln_term = ((6.0 * n as f64 - 5.0) / delta_circ).ln();

    // Terms for lambda_delta calculation
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

    // Logical gate counts: CX and CCX are approximated as T-gates dominate
    LogicalCounts::new(num_qubits, 0, t_gates.ceil() as u64)
}

/// Estimate resources for the Schwinger Model using Alice & Bob's framework.
fn main() -> Result<(), anyhow::Error> {
    // This value can be changed to investigate different cutoffs
    let hilbert_cutoff = 50;

    let qubit = CatQubit::new();
    let qec = RepetitionCode::new();
    let builder = ToffoliBuilder::default();
    let count = schwinger_model_params(hilbert_cutoff);
    let budget = ErrorBudget::new(0.333 * 0.5, 0.333 * 0.5, 0.0);

    let estimation = PhysicalResourceEstimation::new(qec, Rc::new(qubit), builder, Rc::new(count), budget);
    let result: AliceAndBobEstimates = estimation.estimate()?.into();

    println!("Estimates from pre-computed logical count (Schwinger Model):");
    println!("{result}");

    println!("----------------------------------------");
    println!("Exploration of good estimates from pre-computed logical count (Schwinger Model):");
    let results = estimation.build_frontier()?;

    for r in results {
        println!("{}", AliceAndBobEstimates::from(r));
    }

    Ok(())
}