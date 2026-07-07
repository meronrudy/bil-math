# Holonomy Obstruction

This chapter treats institutional semantics as local vector state transported
around workflow loops. A loop is coherent when transporting state around the
loop returns the original coordinates. Failure to return is holonomy
obstruction.

## `HOL-DEF-001`: Institutional Path

An institutional path is a non-empty ordered edge list
`p = (e0, ..., en)` in an institutional graph such that
`t(ei) = s(ei+1)` for every adjacent pair.

Executable witness:

- API: `path_holonomy`
- Test: `hol_lem_003_non_composable_path_returns_adjacency_error`

## `HOL-DEF-002`: Institutional Cycle

An institutional cycle is a non-empty institutional path whose final target is
its initial source:

```text
t(en) = s(e0)
```

Executable witness:

- API: `Cycle`, `cycle_holonomy`
- Test: `hol_thm_002_non_closed_cycle_is_rejected`

## `HOL-DEF-003`: Edge Transport

For each edge `e: u -> v`, an edge transport is a matrix

```text
T(e): X_u -> X_v
```

acting on column vectors. A missing transport prevents holonomy computation.

Executable witness:

- API: `DiscreteConnection::transport`, `TransportMap`
- Test: `hol_def_003_missing_transport_returns_missing_edge`

## `HOL-DEF-004`: Path Holonomy

For a path `(e0, e1, e2)`, path holonomy is the ordered product

```text
H(e0, e1, e2) = T(e2) * T(e1) * T(e0)
```

The product order follows from column-vector action:

```text
x_final = H(e0, e1, e2) * x_initial
```

Proof sketch: applying `T(e0)` first moves from `X_s(e0)` to `X_t(e0)`.
Applying `T(e1)` next requires `t(e0) = s(e1)`, and the composed linear map is
`T(e1) * T(e0)`. Induction over path length gives the right-to-left product.

Executable witness:

- API: `path_holonomy`
- Tests: `hol_lem_002_path_composition_is_right_to_left`,
  `hol_lem_002_path_composition_is_associative_for_compatible_maps`

## `HOL-DEF-005`: Cycle Holonomy

Cycle holonomy is path holonomy for a valid closed cycle. It must be square
before it can be compared to identity.

Proof sketch: a cycle begins and ends at the same graph vertex, but the
executable model still checks the composed matrix shape. This prevents
declaring an obstruction when the computed linear map is not an endomorphism.

Executable witness:

- API: `cycle_holonomy`
- Tests: `hol_thm_002_non_closed_cycle_is_rejected`,
  `hol_thm_002_non_square_cycle_holonomy_is_rejected`

## `HOL-DEF-006`: Holonomy Obstruction

For square cycle holonomy `H`, the obstruction is

```text
||H - I||
```

where `I` is the identity matrix on the same coordinate space. A cycle is
trivial at tolerance `epsilon` when `||H - I|| <= epsilon`.

Executable witness:

- API: `HolonomyObstruction::try_from_holonomy`, `cycle_obstruction`
- Example: `run_holonomy_loop_obstruction`

## `HOL-LEM-001`: Identity Loops Are Trivial

If every transport around a valid cycle is identity, then the cycle holonomy is
identity and the obstruction is zero.

Proof sketch: the ordered product of identity maps is identity. Therefore
`H - I = 0`, and the norm is zero.

Executable witness:

- Tests: `hol_lem_001_identity_loop_has_zero_obstruction`,
  `hol_lem_001_identity_connection_property_over_valid_cycle`

## `HOL-LEM-002`: Compatible Composition Is Associative

For compatible transports, grouped multiplication represents the same linear
map up to floating-point tolerance.

Proof sketch: matrix multiplication is associative over exact scalars. The Rust
test uses finite floating-point matrices and compares grouped products within a
tight tolerance.

Executable witness:

- Tests: `hol_lem_002_path_composition_is_right_to_left`,
  `hol_lem_002_path_composition_is_associative_for_compatible_maps`

## `HOL-LEM-003`: Invalid Structure Is Rejected

Holonomy is computed only after graph and matrix validation. Empty paths,
missing edges, broken adjacency, non-closed cycles, and incompatible dimensions
return errors before obstruction measurement.

Executable witness:

- Tests: `hol_def_003_missing_transport_returns_missing_edge`,
  `hol_lem_003_non_composable_path_returns_adjacency_error`,
  `hol_lem_003_non_composable_path_returns_dimension_error`,
  `hol_thm_002_non_closed_cycle_is_rejected`

## `HOL-THM-001`: Nontrivial Loop Transport Gives Positive Obstruction

If a closed cycle has square holonomy `H` and `H != I`, then its obstruction is
positive under the implemented matrix norm.

Proof sketch: the obstruction is `||H - I||`. For the finite-dimensional norm
used here, a nonzero matrix has positive norm. Therefore any non-identity
cycle holonomy produces positive obstruction.

Executable witness:

- Test: `hol_thm_001_non_identity_loop_has_positive_obstruction`
- CLI: `cargo run -p bil-math-cli -- holonomy-loop`

## `HOL-THM-002`: Obstruction Domain

Cycle obstruction is defined only for closed cycles with square holonomy.

Proof sketch: comparing holonomy to identity requires an endomorphism on one
coordinate space. A non-closed edge list is not a loop, and a rectangular
composite cannot be compared with a square identity on the same space.

Executable witness:

- Tests: `hol_thm_002_non_closed_cycle_is_rejected`,
  `hol_thm_002_non_square_cycle_holonomy_is_rejected`
