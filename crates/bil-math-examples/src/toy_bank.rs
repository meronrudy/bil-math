use bil_math_core::{DirectedEdge, EdgeId, InstitutionalGraph, Matrix, Vector, VertexId};
use bil_math_sheaf::{residual::sheaf_residual, CellularSheaf, RestrictionKey, ZeroCochain};
use std::collections::BTreeMap;

pub fn run_toy_bank_sheaf_residual() -> f64 {
    let underwriting = VertexId(0);
    let compliance = VertexId(1);
    let adverse_action = VertexId(2);

    let edge_uc = EdgeId(0);
    let edge_ca = EdgeId(1);

    let graph = InstitutionalGraph::new(
        vec![underwriting, compliance, adverse_action],
        vec![
            DirectedEdge {
                id: edge_uc,
                source: underwriting,
                target: compliance,
            },
            DirectedEdge {
                id: edge_ca,
                source: compliance,
                target: adverse_action,
            },
        ],
    );

    let mut vertex_dims = BTreeMap::new();
    vertex_dims.insert(underwriting, 2);
    vertex_dims.insert(compliance, 2);
    vertex_dims.insert(adverse_action, 2);

    let mut edge_dims = BTreeMap::new();
    edge_dims.insert(edge_uc, 2);
    edge_dims.insert(edge_ca, 2);

    let identity = Matrix::identity(2, 2);

    let mut restrictions = BTreeMap::new();
    restrictions.insert(
        RestrictionKey {
            vertex: underwriting,
            edge: edge_uc,
        },
        identity.clone(),
    );
    restrictions.insert(
        RestrictionKey {
            vertex: compliance,
            edge: edge_uc,
        },
        identity.clone(),
    );
    restrictions.insert(
        RestrictionKey {
            vertex: compliance,
            edge: edge_ca,
        },
        identity.clone(),
    );
    restrictions.insert(
        RestrictionKey {
            vertex: adverse_action,
            edge: edge_ca,
        },
        identity,
    );

    let sheaf = CellularSheaf::new(graph, vertex_dims, edge_dims, restrictions);

    let cochain = ZeroCochain::new(Vector::from_vec(vec![
        0.80, 0.20, // underwriting state
        0.80, 0.20, // compliance state
        0.60, 0.40, // adverse-action notice state
    ]));

    let residual = sheaf_residual(&sheaf, &cochain).unwrap();

    residual.energy
}
