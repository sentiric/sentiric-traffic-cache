# Project Genesis: sentiric-traffic-cache

- **Document Version:** 1.0
- **Date:** 2025-10-27
- **Status:** APPROVED. This is the primary plan for development.

## 1. Philosophy: "The Invisible Network Assistant"

`sentiric-traffic-cache` is more than a tool; it's a philosophy. It serves as an invisible layer at the heart of your network, intelligently managing, accelerating, and simplifying your entire network experience. Its purpose is to eliminate the "inefficiency tax" imposed by modern development and network usage.

For any device to benefit from it, all it needs to do is "breathe" on the same network.

## 2. Core Principles (The Rules of Construction)

1.  **Modularity:** Each component will be a "box" with well-defined responsibilities, physically enforced by the Cargo Workspace structure.
2.  **Single Responsibility:** Every module and function will do one thing and do it well.
3.  **Testability:** Every unit and integration must be testable. Tests will be written alongside features as proof of robustness.
4.  **Explicitness:** Avoid "magical" or hidden behaviors. Code should clearly state its intent.
5.  **User-Centricity:** Every technical decision will be filtered through the question of how it makes the end-user's experience simpler, faster, or more powerful.

## 3. The Final Architecture & Skeleton

This skeleton is the target structure that will be built from the very first commit.

```tree
sentiric-traffic-cache/
├── .github/                # CI/CD (Test, Lint, Build) workflows
├── Cargo.toml              # <-- Main Workspace definition
├── crates/
│   ├── core/               # 🧠 THE BRAIN: Shared logic, types, and traits. NO I/O or frameworks.
│   ├── service/            # ⚙️ THE SERVICE LAYER: Where the real work happens.
│   ├── cli/                # 🐳 THE HEADLESS RUNNER (For Docker/Servers)
│   └── companion/          # 💻 THE DESKTOP COMPANION (Tauri)
├── web/                    # 🎨 THE SINGLE, CONSISTENT UI
├── packaging/              # 📦 PACKAGING SCRIPTS (MSI, DMG, DEB)
├── docs/                   # 📚 DOCUMENTATION
└── README.md
```

## 4. Accelerated Development Roadmap (Sprints)

-   **Sprint 0: Laying the Foundation (1-3 Days):** Establish the robust project skeleton.
-   **Sprint 1: Core Proxy & Cache (3-5 Days):** Implement the primary value proposition (HTTP/S caching).
-   **Sprint 2: Management UI & Observability (4-6 Days):** Make the project manageable and observable.
-   **Sprint 3 & Beyond: New Horizons:** Develop the Companion App, Smart DNS, and productize the application.
