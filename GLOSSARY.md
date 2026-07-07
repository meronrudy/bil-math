# Glossary

## Core Objects

- **Institutional graph**: a directed graph whose vertices are institutional
  sites and whose edges are workflow transitions.
- **Vertex**: a local site of state, such as underwriting, model service, or
  compliance review.
- **Edge**: an oriented workflow transition from one vertex to another.
- **Cycle**: an ordered, non-empty edge list whose adjacent endpoints match and
  whose final target is the initial source.
- **Path**: an ordered, non-empty edge list whose adjacent endpoints match, not
  necessarily closed.

## Linear Conventions

- **Local semantic state**: a column vector attached to a vertex.
- **Transport map**: a matrix attached to an edge that maps source-state
  coordinates to target-state coordinates.
- **Path holonomy**: the ordered product of edge transports along a path. For
  edges `e0, e1, e2`, the path map is `T(e2) * T(e1) * T(e0)`.
- **Cycle holonomy**: path holonomy around a closed cycle. It must be square
  before an obstruction can be measured.
- **Identity deviation**: the norm of `H - I`, where `H` is cycle holonomy.
- **Holonomy obstruction**: the measured failure of cycle holonomy to equal the
  identity map.

## Supporting Structures

- **Cellular sheaf**: local vector spaces on vertices and edges, plus
  restriction maps from vertex state to edge state.
- **Coboundary matrix**: the linear operator measuring disagreement between
  endpoint restrictions.
- **Sheaf residual**: the vector and energy produced by applying the coboundary
  to a zero-cochain.
- **Global section**: a zero-cochain whose sheaf residual is zero within a
  selected tolerance.
- **Policy lattice**: a finite ordered set of policy states.
- **Only-tighten rule**: a policy transition rule that accepts movement upward
  in the policy order and rejects loosening.
- **Provenance DAG**: an acyclic dependency structure over decision evidence
  events.
- **Replay invariant**: a stable count or structural property used to compare
  audit replays.
