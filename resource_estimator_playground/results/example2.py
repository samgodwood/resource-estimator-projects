import json
import matplotlib.pyplot as plt

# Load results from JSON file
with open("results/example2.json", "r") as f:
    data = json.load(f)

# Extract the values
qubits = data["physical_qubits"]
runtime = data["runtime_seconds"]

# Plotting
plt.figure(figsize=(8, 6))
plt.plot(qubits, runtime, marker='o', linestyle='-')  # Line plot with markers
plt.yscale("log")  # Set y-axis to logarithmic scale
plt.xlabel("Number of Physical Qubits")
plt.ylabel("Runtime (seconds, log scale)")
plt.title("Runtime vs. Physical Qubits (Log Scale)")

# Display the plot
plt.show()
