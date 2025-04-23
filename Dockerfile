# Placeholder Dockerfile for Vorta
# This likely needs to be a multi-stage build in reality

# Base image - choose appropriately (e.g., Ubuntu with Gramine/SGX tools)
# Or use a specific Gramine image if available
FROM ubuntu:22.04 AS base

ENV DEBIAN_FRONTEND=noninteractive

# Install common dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    wget \
    curl \
    git \
#    python3 \
#    python3-pip \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# --- SGX/Gramine Setup Stage (Example) ---
# This section is highly dependent on the chosen base image and SGX/Gramine installation method
# See Gramine documentation: https://gramine.readthedocs.io/en/stable/installation.html
# RUN wget https://download.01.org/intel-sgx/sgx-linux/2.19/distro/ubuntu20.04-server/sgx_linux_x64_driver_XXXX.bin
# RUN chmod +x sgx_linux_x64_driver_XXXX.bin && ./sgx_linux_x64_driver_XXXX.bin
# RUN # Install SGX PSW/SDK
# RUN # Install Gramine

# --- Build Stage (Example) ---
FROM base AS builder
WORKDIR /app

# Install language toolchains if not in base
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
# RUN # Install Go, etc.

# Copy source code
COPY . .

# Build components using Makefile (or individual build steps)
RUN make all # Placeholder

# --- Final Runtime Stage (Example) ---
# Might need separate images for manager, worker, etc.
FROM base AS runtime
WORKDIR /app

# Copy necessary artifacts from builder stage
COPY --from=builder /app/bin /app/bin
COPY --from=builder /app/config /app/config
# Copy mainframe built files, worker binaries, signed manifests, etc.

# Set up entrypoint or command
# Example for mainframe (manager):
ENTRYPOINT ["/app/bin/vorta-mainframe"]
CMD ["--config", "/app/config/mainframe.yaml"]

# Example for worker:
# ENTRYPOINT ["gramine-sgx"] # Or direct binary if not using Gramine for worker mgmt part
# CMD ["/app/bin/vorta-worker", "--config", "/app/config/worker.yaml"]

# Expose ports if necessary (e.g., Manager API port)
# EXPOSE 8000 