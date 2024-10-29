# Project Overview

This repository hosts a small collection of examples around quantum resource estimation using the azure quantum resource estimator and its Rust API. The primary objective is to explore and adapt Microsoft’s resource estimation framework for quantum algorithms on different quantum architectures.

## Directory Structure

- **intro_to_resource_estimation**: A set of Jupyter notebooks introducing the basics of resource estimation with Q#. These tutorials are based on the official Microsoft guide, which can be found [here](https://learn.microsoft.com/en-us/azure/quantum/intro-to-resource-estimation).
- **resource_estimation_playground**: Uses Rust's API to allow custom resource estimation (which allows for different quantum architechtures)
 This includes custom examples and modifications inspired by Q#’s Alice & Bob resource estimator (found [here](https://github.com/Alice-Bob-SW/qsharp-alice-bob-resource-estimator/tree/main)).

## Setup & Installation

1. **Install Rust**: Follow the installation instructions at [rust-lang.org](https://rust-lang.org) to set up Rust.
2. **Clone this repository**:
    ```bash
    git clone https://github.com/samgodwood/resource-estimator-projects
    cd resource-estimator-projects
    ```
   Note: The `environment.yml` file contains all necessary dependencies, but Rust is installed separately.

