import json
import matplotlib.pyplot as plt

# Define the path to the example3.json file
file_path = "./results/Rabi_Model/Rabi_Model.json"

# Load the data from the JSON file
with open(file_path, "r") as file:
    data = json.load(file)

# Extract the frontier results
frontier_results = data["frontier_results"]

# Extract physical qubits and runtime_seconds into separate lists
physical_qubits = [result["physical_qubits"] for result in frontier_results]
runtime_seconds = [result["runtime_seconds"] for result in frontier_results]

# Plot the graph
plt.figure(figsize=(10, 6))
plt.plot(physical_qubits, runtime_seconds, marker="o", linestyle="-", label="Error Budget = 0.01")
plt.xlabel("Physical Qubits")
plt.ylabel("Runtime (seconds)")
plt.title("Runtime vs Physical Qubits for $n_{max} = 3$ Rabi Model")
plt.grid(True)

# Add legend to the plot
plt.legend()

# Save the plot to a PDF file
output_path = file_path.replace(".json", ".pdf")
plt.savefig(output_path, format="pdf", bbox_inches="tight")
plt.close()
