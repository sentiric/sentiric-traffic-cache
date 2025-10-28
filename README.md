# 🚀 sentiric-traffic-cache

[![Continuous Integration & Delivery](https://github.com/sentiric/sentiric-traffic-cache/actions/workflows/ci.yml/badge.svg)](https://github.com/sentiric/sentiric-traffic-cache/actions/workflows/ci.yml)

**The Invisible Network Assistant.** An intelligent, universal caching layer designed to accelerate development workflows and simplify network management.

> This project is being built based on the vision defined in [docs/PROJECT_GENESIS.md](docs/PROJECT_GENESIS.md).

## 🏃‍♂️ Quick Start (Development Environment)

This project is designed to be built and run inside Docker to ensure a consistent and reproducible environment.

### Prerequisites

-   [Docker](https://www.docker.com/products/docker-desktop/)
-   Docker Compose

### Running the Application

1.  Clone the repository:
    ```bash
    git clone https://github.com/sentiric/sentiric-traffic-cache.git
    cd sentiric-traffic-cache
    ```

2.  Start the application using Docker Compose:
    ```bash
    docker compose up --build
    ```
The first build may take some time. Subsequent builds will be much faster thanks to Docker's layer caching.

The proxy will be running and available at `http://127.0.0.1:3128`. You can now configure your browser or system to use this address.