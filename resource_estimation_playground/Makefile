# Define a default target to run the beamsplitter example
.PHONY: all
all: beamsplitter

# Example target for beamsplitter
beamsplitter:
	cargo run --example=beamsplitter
	python results/beamsplitter/beamsplitter.py

# Example target for displacement_operator
displacement_operator:
	cargo run --example=displacement_operator
	python results/displacement_operator/displacement_operator.py

# Add additional targets for other examples
example1:
	cargo run --example=example1
	python results/example1/example1.py

example2:
	cargo run --example=example2
	python results/example2/example2.py

example3:
	cargo run --example=example3
	python results/example3/example3.py

# # Clean target (optional) to remove generated files
# clean:
# 	rm -f results/*/*.json results/*/*.pdf
