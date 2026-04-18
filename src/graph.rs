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

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> Result<(), String> {
        if self.nodes.contains_key(&node.id) {
            return Err(format!("Узел {} уже существует", node.id));
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    pub fn add_edge(&mut self, edge: Edge) -> Result<(), String> {
        if !self.nodes.contains_key(&edge.from) {
            return Err(format!("Узел {} не существует", edge.from));
        }
        if !self.nodes.contains_key(&edge.to) {
            return Err(format!("Узел {} не существует", edge.to));
        }
        self.edges.push(edge);
        Ok(())
    }

      pub fn get_neighbors(&self, node_id: &str) -> Vec<(String, Option<String>)> {
        let mut neighbors = Vec::new();
        
        if let Some(node) = self.nodes.get(node_id) {
            if node.node_type == NodeType::Branch {
                for branch in &node.branches {
                    neighbors.push((branch.target.clone(), branch.condition.clone()));
                }
            }
        }
        
        for edge in &self.edges {
            if edge.from == node_id {
                neighbors.push((edge.to.clone(), edge.condition.clone()));
            }
        }
        
        neighbors
    }

    pub fn get_start_nodes(&self) -> Vec<String> {
        let mut has_incoming = std::collections::HashSet::new();
        
        for edge in &self.edges {
            has_incoming.insert(edge.to.clone());
        }
        
        for node in self.nodes.values() {
            if node.node_type == NodeType::Branch {
                for branch in &node.branches {
                    has_incoming.insert(branch.target.clone());
                }
            }
        }
        
        self.nodes.keys()
            .filter(|id| !has_incoming.contains(*id))
            .cloned()
            .collect()
    }

    pub fn get_end_nodes(&self) -> Vec<String> {
        self.nodes.iter()
            .filter(|(_, node)| node.node_type == NodeType::End)
            .map(|(id, _)| id.clone())
            .collect()
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

