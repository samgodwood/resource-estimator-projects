import json
import matplotlib.pyplot as plt

# Define the path to the Rabi Model JSON file
file_path = "./results/Rabi_Model/Rabi_Model.json"

# Load the data from the JSON file
with open(file_path, "r") as file:
    data = json.load(file)

# Extract the results for different Hilbert cutoffs
pareto_results = data["pareto_estimation_results"]

# Set up subplots (one for each Hilbert cutoff)
num_cutoffs = len(pareto_results)
fig, axes = plt.subplots(
    nrows=(num_cutoffs + 1) // 2,  # Arrange subplots in 2 columns
    ncols=2,
    figsize=(15, 5 * ((num_cutoffs + 1) // 2)),  # Adjust figure size dynamically
    constrained_layout=True  # Let Matplotlib handle spacing
)

# Flatten axes array for easy iteration
axes = axes.flatten()

# Plot each Hilbert cutoff
for i, result in enumerate(pareto_results):
    hilbert_cutoff = result["hilbert_cutoff"]
    frontier_results = result["frontier_results"]

    # Extract physical qubits and runtime for the current cutoff
    physical_qubits = [point["physical_qubits"] for point in frontier_results]
    runtime_seconds = [point["runtime_seconds"] for point in frontier_results]

    # Current subplot axis
    ax = axes[i]

    # Scatter plot with academic styling
    ax.scatter(
        physical_qubits,
        runtime_seconds,
        color="blue",
        s=60,
        edgecolor="black",
        alpha=0.8,
        label=f"Hilbert Cutoff $n_{{max}}={hilbert_cutoff}$"
    )

    # Label axes and add title
    ax.set_xlabel("Physical Qubits", fontsize=12, fontweight="bold")
    ax.set_ylabel("Runtime (s)", fontsize=12, fontweight="bold")
    ax.set_title(
        f"$n_{{max}}={hilbert_cutoff}$",
        fontsize=14,
        fontweight="bold"
    )

    ax.grid(alpha=0.5)
    ax.legend(fontsize=10)

# Turn off any unused subplots if num_cutoffs is odd
for j in range(i + 1, len(axes)):
    axes[j].axis("off")

# Save the plot to a PDF file
output_path = file_path.replace(".json", ".pdf")
plt.savefig(output_path, format="pdf", bbox_inches="tight")
plt.close()
