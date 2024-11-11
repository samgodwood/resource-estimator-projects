import json
import matplotlib.pyplot as plt

# Load results from JSON file
with open("results/schwinger_model_estimates.json", "r") as f:
    data = json.load(f)

# Extract values
hilbert_cutoffs = [entry["hilbert_cutoff"] for entry in data["estimation_results"]]
physical_qubits = [entry["physical_qubits"] for entry in data["estimation_results"]]
runtime = [entry["runtime_seconds"] for entry in data["estimation_results"]]

plt.figure(figsize=(10, 8))

plt.scatter(physical_qubits, runtime, color='black', s=50, edgecolor='black', alpha=0.8)

# Add labels at each point for Hilbert cutoffs
for i, cutoff in enumerate(hilbert_cutoffs):
    plt.annotate(f"$\Lambda={cutoff}$", 
                 (physical_qubits[i], runtime[i]), 
                 textcoords="offset points", 
                 xytext=(5, 6),  # Offset to avoid overlap with the marker
                 ha='center', 
                 fontsize=10, 
                 color='black')

plt.xlabel("Number of Physical Qubits", fontsize=12, fontweight='bold')
plt.ylabel("Runtime (seconds)", fontsize=12, fontweight='bold')
plt.title("Runtime vs. Physical Qubits for Different Link Hilbert Space Cutoffs ($\Lambda$)", fontsize=14, fontweight='bold', pad=15)

# plt.xscale("log")  
# plt.yscale("log")

plt.xticks(fontsize=10)
plt.yticks(fontsize=10)

plt.tight_layout()

plt.show()