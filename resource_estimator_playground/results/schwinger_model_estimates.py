import json
import matplotlib.pyplot as plt

# Load results from JSON file
with open("results/schwinger_model_estimates.json", "r") as f:
    data = json.load(f)

# Extract the values from the JSON array
hilbert_cutoffs = [entry["hilbert_cutoff"] for entry in data["estimation_results"]]
physical_qubits = [entry["physical_qubits"] for entry in data["estimation_results"]]
runtime = [entry["runtime_seconds"] for entry in data["estimation_results"]]

# Plotting Runtime vs. Physical Qubits for different Hilbert Cutoffs
plt.figure(figsize=(8, 6))
plt.plot(physical_qubits, runtime, marker='o', linestyle='-')  

# Label each point with its corresponding Hilbert cutoff
for i, cutoff in enumerate(hilbert_cutoffs):
    plt.text(physical_qubits[i], runtime[i], f"Cutoff={cutoff}", fontsize=9, ha="right")

plt.xlabel("Number of Physical Qubits")
plt.ylabel("Runtime (seconds)")
plt.title("Runtime vs. Physical Qubits for Different Hilbert Cutoffs")
plt.yscale("log")  # Log scale if runtime spans a large range

plt.show()
