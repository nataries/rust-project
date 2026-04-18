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
