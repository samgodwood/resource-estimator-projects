import json
import matplotlib.pyplot as plt

# Define the path to the example3.json file
file_path = "./results/example3.json"

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
plt.plot(physical_qubits, runtime_seconds, marker="o", linestyle="-")
plt.xlabel("Physical Qubits")
plt.ylabel("Runtime (seconds)")
plt.title("Runtime vs Physical Qubits")
plt.grid(True)

# Save the plot to a PDF file
output_path = file_path.replace(".json", ".pdf")
plt.savefig(output_path, format="pdf", bbox_inches="tight")
plt.close()
