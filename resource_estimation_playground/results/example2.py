import json
import matplotlib.pyplot as plt

# Load results from JSON file
with open("results/example2.json", "r") as f:
    data = json.load(f)

# Extract values from the JSON array
qubits = [entry["physical_qubits"] for entry in data["estimation_results"]]
runtime = [entry["runtime_seconds"] for entry in data["estimation_results"]]
error_budgets = [entry["logical_error_budget"] for entry in data["estimation_results"]]

# Plotting Runtime vs. Physical Qubits with labeled error budgets
plt.figure(figsize=(10, 8))
plt.scatter(qubits, runtime, color='black', s=50, edgecolor='black', alpha=0.8)

# Add labels at each point for error budgets
for i, budget in enumerate(error_budgets):
    plt.annotate(f"$\\varepsilon={budget}$", 
                 (qubits[i], runtime[i]), 
                 textcoords="offset points", 
                 xytext=(5, 6),  # Offset to avoid overlap with the marker
                 ha='center', 
                 fontsize=10, 
                 color='black')

# # Set scales to logarithmic and add labels
# plt.xscale("log")
# plt.yscale("log")
plt.xlabel("Number of Physical Qubits", fontsize=12, fontweight='bold')
plt.ylabel("Runtime (seconds)", fontsize=12, fontweight='bold')
plt.title("Runtime vs. Physical Qubits for Different Logical Error Budgets ($\\varepsilon$)", fontsize=14, fontweight='bold', pad=15)

plt.xticks(fontsize=10)
plt.yticks(fontsize=10)

plt.tight_layout()

# Save plot to PDF
output_file = "results/example2.pdf"
plt.savefig(output_file, format='pdf')
plt.close()

