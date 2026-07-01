# BIL Math

Pure Rust mathematical foundations for the Banking Integration Layer.

BIL Math defines typed, deterministic primitives for regulated AI assurance:

- institutional graphs
- policy lattices
- provenance DAGs
- cellular sheaves
- sheaf residuals
- discrete transport
- institutional holonomy
- audit replay invariants

This repository is math-first. It is not the commercial BIL product, and it does not contain customer integrations, bank-specific workflows, production evidence schemas, or hardware RTL.

## Why Rust?

Rust is used here because BIL’s mathematical substrate should be:

- deterministic
- memory-safe
- testable
- portable
- suitable for eventual production hardening
- suitable for later FPGA, SmartNIC, and EPU interface boundaries

The initial goal is not performance. The initial goal is mathematical clarity with a credible path to hardened infrastructure.

## Core Mathematical Thesis

Regulated AI assurance is a local-to-global consistency problem.

Each institution, policy engine, model service, human approval queue, or compliance subsystem has a local view of a decision. BIL Math asks whether those local views glue into a coherent global institutional state.

When they do not glue, the failure is measured as a sheaf residual.

When meaning changes around a workflow loop, the accumulated transformation is measured as institutional holonomy.

## Repository Boundary

This repo contains:

- mathematical definitions
- typed Rust reference implementations
- toy examples
- tests
- benchmarks
- paper drafts

This repo does not contain:

- production BIL APIs
- customer data
- bank integrations
- pricing
- investor pitch materials
- hardware RTL