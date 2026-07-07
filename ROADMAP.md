# Roadmap

## Milestone 1: Holonomy Obstruction Spine

Status: active.

- Define the graph, path, cycle, transport, path holonomy, cycle holonomy, and
  obstruction conventions in plain Markdown.
- Link every holonomy definition, lemma, and theorem to Rust APIs, tests, and
  examples.
- Keep `cargo test --workspace` and `cargo fmt --all -- --check` passing.
- Use `bil-math holonomy-loop` as the first executable witness of nontrivial
  institutional holonomy.

## Milestone 2: Sheaf Residual Support

Status: scaffolded.

- Connect sheaf residuals to the local-to-global consistency interpretation.
- Add theorem IDs for coboundary construction, residual energy, and global
  section checks.
- Preserve the toy-bank example as the first sheaf residual witness.

## Milestone 3: Policy and Provenance Foundations

Status: scaffolded.

- Treat the policy lattice as an order-theoretic constraint layer.
- Treat provenance DAG replay as the audit trace layer.
- Add tests that show how policy tightening and replay invariants support the
  holonomy and sheaf chapters.

## Milestone 4: Dissertation-Quality Expansion

Status: planned.

- Expand proof sketches into longer notes with references.
- Add benchmarks only after the mathematics is stable.
- Consider a formal proof assistant only after the Markdown proof spine and
  executable Rust witnesses have stabilized.
