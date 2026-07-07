# Trace Matrix

This table links every current holonomy definition, lemma, and theorem to the
Rust surface that witnesses it.

| ID | Claim | Rust API | Tests | Example or CLI |
| --- | --- | --- | --- | --- |
| `HOL-DEF-001` | Institutional path is a non-empty ordered edge list with connected adjacent endpoints. | `path_holonomy` | `hol_lem_003_non_composable_path_returns_adjacency_error` | `holonomy-loop` uses a connected loop path |
| `HOL-DEF-002` | Institutional cycle is a non-empty path whose final target equals initial source. | `Cycle`, `cycle_holonomy` | `hol_thm_002_non_closed_cycle_is_rejected` | `run_holonomy_loop_obstruction` |
| `HOL-DEF-003` | Edge transport maps source column-vector state to target state. | `DiscreteConnection`, `TransportMap` | `hol_def_003_missing_transport_returns_missing_edge` | `run_holonomy_loop_obstruction` |
| `HOL-DEF-004` | Path holonomy composes transports right-to-left. | `path_holonomy` | `hol_lem_002_path_composition_is_right_to_left` | `holonomy-loop` |
| `HOL-DEF-005` | Cycle holonomy is square path holonomy over a closed cycle. | `cycle_holonomy` | `hol_thm_002_non_closed_cycle_is_rejected`, `hol_thm_002_non_square_cycle_holonomy_is_rejected` | `run_holonomy_loop_obstruction` |
| `HOL-DEF-006` | Holonomy obstruction is `||H - I||`. | `HolonomyObstruction::try_from_holonomy`, `cycle_obstruction` | `hol_lem_001_identity_loop_has_zero_obstruction`, `hol_thm_001_non_identity_loop_has_positive_obstruction` | `holonomy-loop` |
| `HOL-LEM-001` | Identity connection has zero obstruction around valid cycles. | `cycle_obstruction` | `hol_lem_001_identity_loop_has_zero_obstruction`, `hol_lem_001_identity_connection_property_over_valid_cycle` | Not separate; covered by tests |
| `HOL-LEM-002` | Compatible path composition is associative and matches right-to-left multiplication. | `path_holonomy` | `hol_lem_002_path_composition_is_right_to_left`, `hol_lem_002_path_composition_is_associative_for_compatible_maps` | Not separate; covered by tests |
| `HOL-LEM-003` | Invalid paths and cycles are rejected before obstruction measurement. | `path_holonomy`, `cycle_holonomy` | `hol_lem_003_non_composable_path_returns_adjacency_error`, `hol_lem_003_non_composable_path_returns_dimension_error`, `hol_thm_002_non_closed_cycle_is_rejected` | Not separate; covered by tests |
| `HOL-THM-001` | A closed loop with non-identity holonomy has positive obstruction. | `cycle_obstruction` | `hol_thm_001_non_identity_loop_has_positive_obstruction` | `cargo run -p bil-math-cli -- holonomy-loop` |
| `HOL-THM-002` | Cycle obstruction is defined only for closed cycles with square holonomy. | `cycle_holonomy`, `cycle_obstruction` | `hol_thm_002_non_closed_cycle_is_rejected`, `hol_thm_002_non_square_cycle_holonomy_is_rejected` | Not separate; covered by tests |

## Maintenance Rule

When a new theorem ID is added to [proof-spine.md](proof-spine.md), add a row
here before considering the proof spine complete. Each row should name at least
one Rust API and one executable test or example.
