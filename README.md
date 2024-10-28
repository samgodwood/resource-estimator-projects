# Project Overview

This repository hosts a collection of projects around quantum resource estimation using Q# and Rust-based tools. The primary objective is to explore and adapt Microsoft’s resource estimation framework for quantum algorithms on different quantum architectures, with custom adaptations and examples.

## Directory Structure

- **RE_intro**: A set of Jupyter notebooks introducing the basics of resource estimation with Q#. These tutorials are based on the official Microsoft guide, which can be found [here](https://learn.microsoft.com/en-us/azure/quantum/intro-to-resource-estimation).
- **resource_estimator_playground**: Uses Rust's API to allow custom resource estimation (which allows for different quantum architechtures)
 This includes custom examples and modifications inspired by Q#’s Alice & Bob resource estimator, which can be found [here](https://github.com/Alice-Bob-SW/qsharp-alice-bob-resource-estimator/tree/main).

## Setup & Installation

1. **Install Rust**: Follow the installation instructions at [rust-lang.org](https://rust-lang.org) to set up Rust on your system.
2. **Clone this repository**:
    ```bash
    git clone https://github.com/samgodwood/resource-estimator-projects
    cd resource-estimator-projects
    ```
3. **Environment Setup**: Set up the Conda environment as specified in `environment.yml`:
    ```bash
    conda env create -f environment.yml
    conda activate resource-estimator-env
    ```
   Note: The `environment.yml` file contains all necessary dependencies, but Rust is installed separately.

