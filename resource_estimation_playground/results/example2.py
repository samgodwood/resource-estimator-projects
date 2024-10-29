import json
import matplotlib.pyplot as plt

# Load results from JSON file
with open("results/example2.json", "r") as f:
    data = json.load(f)

# Extract the values from the JSON array
qubits = [entry["physical_qubits"] for entry in data["estimation_results"]]
runtime = [entry["runtime_seconds"] for entry in data["estimation_results"]]
error_budgets = [entry["logical_error_budget"] for entry in data["estimation_results"]]

# Plotting Runtime vs. Physical Qubits, with each point labeled by its error budget
plt.figure(figsize=(8, 6))
plt.plot(qubits, runtime, marker='o', linestyle='-')  

# Label each point with its logical error budget
for i, budget in enumerate(error_budgets):
    plt.text(qubits[i], runtime[i], f"{budget}", fontsize=9, ha="right")

plt.xscale("log")  
plt.yscale("log")  
plt.xlabel("Number of Physical Qubits (log scale)")
plt.ylabel("Runtime (seconds, log scale)")
plt.title("Runtime vs. Physical Qubits for Different Logical Error Budgets (Log-Log Scale)")

plt.show()
