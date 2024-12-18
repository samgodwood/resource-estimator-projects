import json
import matplotlib.pyplot as plt

# Define the path to the displacement operator JSON file
file_path = "./results/displacement_operator/displacement_operator.json"

# Load the data from the JSON file
with open(file_path, "r") as file:
    data = json.load(file)

# Extract the results for Hilbert cutoffs and photon loss rates
hilbert_cutoff_results = data["hilbert_cutoff_results"]
photon_loss_results = data["photon_loss_results"]

# Plot for fixed error budget and varying Hilbert space cutoff
fig, ax = plt.subplots(figsize=(10, 6))

for result in hilbert_cutoff_results:
    hilbert_cutoff = result["hilbert_cutoff"]
    frontier_results = result["frontier_results"]

    # Extract physical qubits and runtime for the current cutoff
    physical_qubits = [point["physical_qubits"] for point in frontier_results]
    runtime_seconds = [point["runtime_seconds"] for point in frontier_results]

    # Scatter plot
    ax.scatter(
        physical_qubits,
        runtime_seconds,
        label=f"Hilbert Cutoff $n_{{max}}={hilbert_cutoff}$",
        s=60,
        alpha=0.8,
        edgecolor="black"
    )

# Add labels, legend, and grid
ax.set_xlabel("Physical Qubits", fontsize=12, fontweight="bold")
ax.set_ylabel("Runtime (s)", fontsize=12, fontweight="bold")
ax.set_title("Fixed Error Budget and Varying Hilbert Cutoff", fontsize=14, fontweight="bold")
ax.grid(alpha=0.5)
ax.legend(fontsize=10)

# Save the plot to a PDF file
output_path_cutoff = "./results/displacement_operator/hilbert_cutoff_plot.pdf"
plt.savefig(output_path_cutoff, format="pdf", bbox_inches="tight")
plt.close()

# Plot for fixed cutoff and varying photon loss rates
fig, ax = plt.subplots(figsize=(10, 6))

for result in photon_loss_results:
    photon_loss_rate = result["photon_loss_rate"]
    frontier_results = result["frontier_results"]

    # Extract physical qubits and runtime for the current photon loss rate
    physical_qubits = [point["physical_qubits"] for point in frontier_results]
    runtime_seconds = [point["runtime_seconds"] for point in frontier_results]

    # Scatter plot
    ax.scatter(
        physical_qubits,
        runtime_seconds,
        label=f"Photon Loss $x={photon_loss_rate}$",
        s=60,
        alpha=0.8,
        edgecolor="black"
    )

# Add labels, legend, and grid
ax.set_xlabel("Physical Qubits", fontsize=12, fontweight="bold")
ax.set_ylabel("Runtime (s)", fontsize=12, fontweight="bold")
ax.set_title("Fixed Cutoff and Varying Photon Loss Rates", fontsize=14, fontweight="bold")
ax.grid(alpha=0.5)
ax.legend(fontsize=10)

# Save the plot to a PDF file
output_path_photon_loss = "./results/displacement_operator/photon_loss_plot.pdf"
plt.savefig(output_path_photon_loss, format="pdf", bbox_inches="tight")
plt.close()
