use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    Action,
    Branch,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchEdge {
    pub description: String,
    pub condition: Option<String>,
    pub target: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub node_type: NodeType,
    #[serde(default)]
    pub branches: Vec<BranchEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub condition: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub edges: Vec<Edge>,
}
