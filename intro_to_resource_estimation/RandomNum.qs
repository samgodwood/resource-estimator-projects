/// # Sample
/// Random Bit
///
/// # Description
/// This Q# program generates a random bit by setting a qubit in a superposition
/// of the computational basis states |0〉 and |1〉, and returning the measurement
/// result.
namespace Sample {

    @EntryPoint()
    operation RandomBit() : Result {
        // Qubits are only accessible for the duration of the scope where they
        // are allocated and are automatically released at the end of the scope.
        use qubits = Qubit[3];

        // Apply Hadamard gates
        H(qubits[0]);
        H(qubits[1]);
        H(qubits[2]);

        // Apply a Toffoli gate
        CCNOT(qubits[0], qubits[1], qubits[2]);

        // Measure the qubits
        let results = [M(qubits[0]), M(qubits[1]), M(qubits[2])];

        // Reset the qubits
        ResetAll(qubits);
        return results[2];


    }
}
