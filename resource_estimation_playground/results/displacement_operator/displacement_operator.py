import json
import matplotlib.pyplot as plt

# Define the path to the displacement operator JSON file
file_path = "./results/displacement_operator/displacement_operator.json"

# Define marker styles for each Hilbert cutoff K
markers = {7: 'o', 63: 's', 256: '^'}

# Define colors for each method
colors = {"pauli_decomp": "blue", "newton_iterations": "red"}

# Load the data from the JSON file
with open(file_path, "r") as file:
    data = json.load(file)

results = data["results"]

# Group data by (method, K)
grouped = {}
for result in results:
    method = result["method"]
    K = result["K"]
    key = (method, K)
    if key not in grouped:
        grouped[key] = {"physical_qubits": [], "runtime_seconds": []}
    for point in result["frontier_results"]:
        grouped[key]["physical_qubits"].append(point["physical_qubits"])
        grouped[key]["runtime_seconds"].append(point["runtime_seconds"])

# Create a scatter plot for runtime vs physical qubits
plt.figure(figsize=(10, 6))
for (method, K), values in grouped.items():
    plt.scatter(
        values["physical_qubits"],
        values["runtime_seconds"],
        s=60,
        alpha=0.8,
        marker=markers.get(K, 'o'),
        color=colors.get(method, "black"),
        label=f"{method}, K={K}",
        edgecolor="black"
    )

plt.xlabel("Physical Qubits", fontsize=12, fontweight="bold")
plt.ylabel("Runtime (s)", fontsize=12, fontweight="bold")
plt.title("Runtime vs Physical Qubits by Method and Hilbert Cutoff", fontsize=14, fontweight="bold")
plt.grid(alpha=0.5)
plt.legend(fontsize=10)
output_path = "./results/displacement_operator/runtime_vs_physical_qubits.pdf"
plt.savefig(output_path, format="pdf", bbox_inches="tight")
plt.close()

print("Runtime vs Physical Qubits plot saved successfully!")

