# Resource Estimator Playground

This project is a custom setup for experimenting with Microsoft’s resource estimator. Based on Alice & Bob's architecture, this directory has been adapted to support custom Q# files and different quantum error correction protocols.

## Structure

- **src**: Contains the main Rust code for resource estimation, adapted to run Q# scripts and generate physical resource estimates.
- **examples**: Sample scripts demonstrating the resource estimation process for different algorithms and logical setups.
- **qsharp**: Folder for custom Q# files defining specific quantum algorithms to estimate resources for.

## Usage

1. **Building the Project**:
    ```bash
    cd resource_estimator_playground
    cargo build --release
    ```

2. **Running Examples**:
    Run the `elliptic_log` example:
    ```bash
    cargo run --example elliptic_log
    ```

    Run a custom Q# file:
    ```bash
    cargo run --example <your_example>
    ```

---
