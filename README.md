# Vorta: The First Trustless Container Orchestrator for Verified AI Computations

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/vortaorg/vorta) <!-- Placeholder -->
[![Issues](https://img.shields.io/github/issues/vortaorg/vorta)](https://github.com/vortaorg/vorta/issues) <!-- Placeholder -->
**Website:** [https://vorta.org](https://vorta.org)

> Vorta is to trustless AI what Kubernetes is to cloud computing - a powerful orchestrator that ensures every computation is verifiable, secure, and truly decentralized.

## Project Description

Vorta is a distributed system designed for orchestrating secure containerized workloads. It leverages Zero-Knowledge (ZK) proof verification within Intel SGX enclaves (Trusted Execution Environments - TEEs) using the Gramine LibOS. Intel SGX provides hardware-based memory encryption and isolation, shielding code and data from the host OS, hypervisor, and even physical attacks. Gramine acts as a Library OS, enabling unmodified Linux applications (like ZK verifiers or AI models) to run seamlessly within these secure SGX enclaves by intercepting system calls and managing the trusted execution context. This combination ensures the integrity and confidentiality of computations, particularly crucial for AI workloads requiring trust and verification, by allowing remote parties to verify workload execution via SGX's remote attestation mechanism. Vorta coordinates the deployment, management, and attestation of these secure, Gramine-shielded containers across a cluster of SGX-enabled nodes.

## System Architecture

Vorta's architecture comprises three main components working in concert to deliver secure and efficient workload orchestration:

### 1. Scheduler

The scheduler intelligently determines the optimal placement of secure workloads across available worker nodes. This process involves three critical phases:

*   **Feasibility Analysis:** Evaluates if a task can run on a worker by checking SGX enclave requirements, TEE capabilities, and ZK-proof verification readiness. It validates the worker's capacity for proof verification, memory requirements for execution traces, and quote generation ability for remote attestation.
*   **Worker Scoring:** Scores candidate workers based on factors like available SGX Enclave Page Cache (EPC) memory, ZK-proof verification performance, historical success rates, current enclave utilization, network latency, and TEE attestation status.
*   **Optimal Selection:** Selects the best worker considering proof verification throughput needs, enclave memory pressure, load balancing across TEE workers, and data locality for proof artifacts.

### 2. Manager (Mainframe)

The Manager (referred to as Mainframe in the codebase) acts as the central control plane, potentially running within a TEE itself for enhanced security. It handles orchestration logic (including scheduling), job management, and exposes the primary API.

*   **API:** Exposes a comprehensive API (likely gRPC or REST) for users to:
    *   Submit ZK-proof verification jobs.
    *   Deploy secure containers in SGX enclaves.
    *   Monitor verification status.
    *   Query TEE capabilities.
    *   Control job lifecycle (start, stop, query).
    *   Access attestation reports and verification metrics.
*   **Job Storage:** Maintains a persistent state of all jobs, including enclave allocation status, proof execution traces, verification results, and attestation data.
*   **Metrics Collection:** Gathers critical performance and security metrics like SGX enclave utilization, proof throughput, memory usage, CPU load (secure/insecure worlds), TEE performance, and remote attestation statistics to inform scheduling and optimization.

### 3. Worker

Worker nodes are the execution agents within the Vorta cluster.

*   **Responsibilities:** Execute secure containers and perform ZK-proof verification within SGX enclaves. When a task is assigned by the Manager, the Worker uses Gramine to launch the specified container image within a hardware-isolated SGX enclave. Gramine handles the secure loading of the application and its dependencies, protecting them from external interference. During or after execution, the Worker interacts with the SGX hardware to generate a cryptographic quote (remote attestation report). This quote includes measurements of the executed code (MRENCLAVE) and the enclave's signing identity (MRSIGNER), cryptographically signed by the CPU's platform keys. This allows the Manager or end-users to verify that the computation ran correctly on genuine SGX hardware with the expected software configuration.
*   **API:** Provides an API for the Manager to:
    *   Accept tasks.
    *   Launch secure containers via Gramine (`gramine-sgx`).
    *   Report enclave metrics.
    *   Provide attestation data (quotes).
    *   Manage proof execution traces.
    *   Control container lifecycle.

This architecture guarantees secure proof verification within hardware-isolated SGX enclaves, protects execution traces, produces verifiable computation results, enables scalable secure container deployment, and optimizes resource utilization while upholding strong security guarantees through TEEs and remote attestation.

## How it's Made

Vorta integrates a diverse set of technologies to achieve its goals:

### Software

*   **Remote Attestation:** Phala Dstack SDK
*   **Smart Contracts:** Solidity with Foundry
*   **Mainframe (Manager/Scheduler):** Rust (designed for TEE execution)
*   **TEE Runtime:** Gramine LibOS
*   **Hardware Interface:** Intel SGX Driver
*   **Containerization:** Docker Runtime
*   **ZK-Proof Verification:** Custom implementations in Rust and Golang
*   **Attestation Service:** Dedicated service for managing attestation verification

### Hardware Requirements

*   **CPU:** Intel processor with SGX support enabled.
*   **Memory:** Sufficient Enclave Page Cache (EPC) memory allocated in BIOS/UEFI.
*   **Storage:** Secure and performant storage for ZK proofs and execution traces.
*   **Network:** High-bandwidth network connectivity for efficient proof distribution.
*   **Platform:** TEE-enabled server or cloud instance.

## Getting Started

### Prerequisites

*   Intel SGX enabled hardware & drivers installed.
*   Gramine installed (`gramine-sgx`).
*   Docker installed and configured.
*   Rust toolchain (`rustup`).
*   Go toolchain.
*   Foundry installed (`foundryup`).
*   Access to a cloud VM or suitable hardware with SGX support.

### Installation

```bash
# 1. Clone the repository
git clone https://github.com/vortaorg/vorta.git
cd vorta

# 2. Set up environment variables
cp .env.example .env
# (Edit .env with your specific configurations)

# 3. Build components (using Makefile)
# This is a placeholder - actual build steps might be more complex
make all

# 4. (Optional) Build specific components manually
# make mainframe
# make worker
# make contracts
```

*(Detailed installation steps specific to each component and platform dependencies might be required. See component-specific READMEs if available.)*

## Usage

*(Instructions on how to deploy the system, submit jobs, and interact with the API TBD)*

```bash
# Example: Start the Vorta mainframe (Placeholder)
./bin/vorta-mainframe --config config/mainframe.yaml

# Example: Start a Vorta worker (Placeholder)
./bin/vorta-worker --config config/worker.yaml --node-id worker-01

# Example: Submit a job via CLI (Placeholder)
vorta-cli submit --endpoint <mainframe-api-url> --job-spec jobs/example_job.json --proof artifacts/proof.bin
```

## Project Structure (Overview)

```
vorta/
├── .env.example         # Environment variable template
├── .gitignore           # Git ignore rules
├── Cargo.toml           # Rust workspace/dependencies (Worker, ZKP)
├── Dockerfile           # Main Docker build file (or multi-stage)
├── LICENSE              # Project License (MIT)
├── Makefile             # Build automation script
├── README.md            # This file
├── attestation/         # Attestation service code/config
│   └── ...
├── config/              # Configuration files (YAML, TOML, etc.)
│   ├── mainframe.yaml   # Mainframe configuration
│   └── worker.yaml
├── contracts/           # Solidity smart contracts
│   ├── src/
│   ├── test/
│   └── foundry.toml
├── go.mod               # Go dependencies (e.g., for ZKP verifier/utils)
├── gramine-manifests/   # Gramine manifest templates
│   └── worker.manifest.template
├── jobs/                # Example job specifications
│   └── example_job.json
├── mainframe/           # Rust Mainframe (Manager/Scheduler in TEE)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── enclave.rs
│       ├── attestation.rs
│       ├── error.rs
│       ├── config.rs
│       ├── scheduler.rs
│       └── api.rs
├── scripts/             # Utility and deployment scripts
│   ├── setup_sgx.sh     # Script to check/setup SGX environment
│   └── deploy.sh        # Example deployment script
├── sgx_config/          # SGX specific configuration files (if needed)
│   └── enclave_signing_key.pem # Example placeholder
├── worker/              # Worker node implementation (e.g., Rust/Go)
│   ├── src/
│   └── Cargo.toml       # Worker specific Cargo.toml (if not in workspace)
└── zkp/                 # ZK-Proof verifier components (Rust/Go)
    ├── rust-verifier/
    └── go-verifier/
```

## Contributing

Contributions are welcome! Please ensure your code adheres to the project's style guidelines and all tests pass.

1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request

*(A more formal `CONTRIBUTING.md` may be added later.)*

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgements

*   Gramine Project
*   Intel SGX Team
*   Phala Network
*   Rust Community
*   Foundry Team
*   *(Add other relevant acknowledgements)*
