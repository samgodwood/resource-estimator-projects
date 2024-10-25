# Project Overview

This repository hosts a collection of projects around quantum resource estimation using Q# and Rust-based resource estimation tools. The primary goal is to explore and adapt resource estimation for different quantum architectures, with a focus on adapting Microsoft’s resource estimation for custom architectures, such as Alice & Bob's.

## Directory Structure

- **RE_intro**: Contains introductory notebooks for the resource estimator, focusing on the fundamentals of using the resource estimator with Q#.
- **qsharp-alice-bob-resource-estimator**: A cloned repository that implements a resource estimator specifically for Alice & Bob's architecture. Documentation and setup are provided in this repo.
- **resource_estimator_playground**: A custom module and library developed to build on Alice & Bob's example. This allows experimentation with custom Q# files, which are converted into logical qubit counts and resource estimations.

## Setup & Installation

1. **Install Rust**: Follow the instructions at https://rust-lang.org to install Rust.
2. **Clone this repository**: 
    ```bash
    git clone <repository_url>
    cd resource-estimator-projects
    ```
3. **Environment Setup**: If using Conda, set up the environment with:
    ```bash
    conda env create -f environment.yml
    conda activate resource-estimator-env
    ```

### Running Examples

Refer to individual project READMEs for detailed usage instructions.

