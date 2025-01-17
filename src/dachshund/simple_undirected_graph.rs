/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
extern crate fxhash;
use crate::dachshund::algorithms::adjacency_matrix::AdjacencyMatrix;
use crate::dachshund::algorithms::algebraic_connectivity::AlgebraicConnectivity;
use crate::dachshund::algorithms::betweenness::Betweenness;
use crate::dachshund::algorithms::clustering::Clustering;
use crate::dachshund::algorithms::cnm_communities::CNMCommunities;
use crate::dachshund::algorithms::connected_components::{
    ConnectedComponents, ConnectedComponentsUndirected,
};
use crate::dachshund::algorithms::connectivity::{Connectivity, ConnectivityUndirected};
use crate::dachshund::algorithms::coreness::Coreness;
use crate::dachshund::algorithms::eigenvector_centrality::EigenvectorCentrality;
use crate::dachshund::algorithms::laplacian::Laplacian;
use crate::dachshund::algorithms::shortest_paths::ShortestPaths;
use crate::dachshund::algorithms::transitivity::Transitivity;
use crate::dachshund::graph_base::GraphBase;
use crate::dachshund::id_types::NodeId;
use crate::dachshund::node::{NodeBase, NodeEdgeBase, SimpleNode};
use fxhash::FxHashMap;
use std::collections::hash_map::{Keys, Values};
use crate::dachshund::algorithms::k_peaks::KPeaks;

pub trait UndirectedGraph
    where
        Self: GraphBase,
{
}

/// Keeps track of a simple undirected graph, composed of nodes without any type information.
pub struct SimpleUndirectedGraph {
    pub nodes: FxHashMap<NodeId, SimpleNode>,
    pub ids: Vec<NodeId>,
}
impl GraphBase for SimpleUndirectedGraph {
    type NodeType = SimpleNode;

    /// core and non-core IDs are the same for a `SimpleUndirectedGraph`.
    fn get_core_ids(&self) -> &Vec<NodeId> {
        &self.ids
    }
    /// core and non-core IDs are the same for a `SimpleUndirectedGraph`.
    fn get_non_core_ids(&self) -> Option<&Vec<NodeId>> {
        Some(&self.ids)
    }
    fn get_ids_iter(&self) -> Keys<NodeId, SimpleNode> {
        self.nodes.keys()
    }
    fn get_nodes_iter(&self) -> Values<NodeId, SimpleNode> {
        self.nodes.values()
    }
    fn get_mut_nodes(&mut self) -> &mut FxHashMap<NodeId, SimpleNode> {
        &mut self.nodes
    }
    fn has_node(&self, node_id: NodeId) -> bool {
        self.nodes.contains_key(&node_id)
    }
    fn get_node(&self, node_id: NodeId) -> &SimpleNode {
        &self.nodes[&node_id]
    }
    fn count_edges(&self) -> usize {
        let mut num_edges: usize = 0;
        for node in self.nodes.values() {
            num_edges += node.neighbors.len();
        }
        num_edges / 2
    }
    fn count_nodes(&self) -> usize {
        self.nodes.len()
    }
    fn create_empty() -> Self {
        SimpleUndirectedGraph {
            nodes: FxHashMap::default(),
            ids: Vec::new(),
        }
    }
}
impl SimpleUndirectedGraph {
    pub fn as_input_rows(&self, graph_id: usize) -> String {
        let mut rows: Vec<String> = Vec::new();
        for (id, node) in &self.nodes {
            for e in node.get_edges() {
                if *id < e.get_neighbor_id() {
                    rows.push(format!(
                        "{}\t{}\t{}",
                        graph_id,
                        id.value(),
                        e.get_neighbor_id().value()
                    ));
                }
            }
        }
        rows.join("\n")
    }
    pub fn get_node_degree(&self, id: NodeId) -> usize {
        self.nodes[&id].degree()
    }
}
impl UndirectedGraph for SimpleUndirectedGraph {}

impl CNMCommunities for SimpleUndirectedGraph {}
impl ConnectedComponents for SimpleUndirectedGraph {}
impl ConnectedComponentsUndirected for SimpleUndirectedGraph {}
impl Coreness for SimpleUndirectedGraph {}
impl KPeaks for SimpleUndirectedGraph {}

impl AdjacencyMatrix for SimpleUndirectedGraph {}
impl Clustering for SimpleUndirectedGraph {}
impl Connectivity for SimpleUndirectedGraph {}
impl ConnectivityUndirected for SimpleUndirectedGraph {}
impl Betweenness for SimpleUndirectedGraph {}
impl Laplacian for SimpleUndirectedGraph {}
impl Transitivity for SimpleUndirectedGraph {}
impl ShortestPaths for SimpleUndirectedGraph {}
impl AlgebraicConnectivity for SimpleUndirectedGraph {}
impl EigenvectorCentrality for SimpleUndirectedGraph {}
