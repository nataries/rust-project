use crate::graph::Graph;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

#[derive(Debug, Clone)]
pub struct PathResult {
    pub nodes: Vec<String>,
    pub conditions: Vec<String>,
}

pub async fn find_paths(
    graph: Arc<Graph>,
    start_nodes: Vec<String>,
    end_nodes: Vec<String>,
    max_depth: usize,
) -> Vec<PathResult> {
    let semaphore = Arc::new(Semaphore::new(10));
    let mut handles = Vec::new();
    
    for start in start_nodes {
        let graph_clone = graph.clone();
        let sem_clone = semaphore.clone();
        let end_clone = end_nodes.clone();
        
        let handle = task::spawn(async move {
            let _permit = sem_clone.acquire().await.unwrap();
            find_paths_from_start(&graph_clone, &start, &end_clone, max_depth).await
        });
        
        handles.push(handle);
    }
    
    let mut all_paths = Vec::new();
    for handle in handles {
        if let Ok(paths) = handle.await {
            all_paths.extend(paths);
        }
    }
    
    all_paths
}

async fn find_paths_from_start(
    graph: &Graph,
    start: &str,
    end_nodes: &[String],
    max_depth: usize,
) -> Vec<PathResult> {
    let mut results = Vec::new();
    let mut queue = VecDeque::new();
    
    queue.push_back((start.to_string(), vec![start.to_string()], vec![], 0));
    
    while let Some((current, path, conditions, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }
        
        if end_nodes.contains(&current) {
            results.push(PathResult {
                nodes: path.clone(),
                conditions: conditions.clone(),
            });
            continue;
        }
        
        let neighbors = graph.get_neighbors(&current);
        
        for (next, condition) in neighbors {
            let mut new_path = path.clone();
            new_path.push(next.clone());
            
            let mut new_conditions = conditions.clone();
            if let Some(cond) = condition {
                new_conditions.push(cond);
            }
            
            queue.push_back((next, new_path, new_conditions, depth + 1));
        }
    }
    
    results
}

pub fn export_to_dot(graph: &Graph, _paths: &[PathResult]) -> String {
    let mut dot = "digraph G {\n".to_string();
    dot.push_str("    rankdir=TB;\n");
    dot.push_str("    node [shape=box, style=filled];\n");
    
    for (id, node) in &graph.nodes {
        let color = match node.node_type {
            crate::graph::NodeType::Action => "lightblue",
            crate::graph::NodeType::Branch => "lightgreen",
            crate::graph::NodeType::End => "lightcoral",
        };
        
        let label = format!("{}\n{}", node.name, node.description.as_deref().unwrap_or(""));
        dot.push_str(&format!("    \"{}\" [label=\"{}\", fillcolor={}];\n", id, label, color));
    }
    
    for edge in &graph.edges {
        let label = edge.description.as_deref().unwrap_or("");
        if label.is_empty() {
            dot.push_str(&format!("    \"{}\" -> \"{}\";\n", edge.from, edge.to));
        } else {
            dot.push_str(&format!("    \"{}\" -> \"{}\" [label=\"{}\"];\n", edge.from, edge.to, label));
        }
    }
    
    for node in graph.nodes.values() {
        if node.node_type == crate::graph::NodeType::Branch {
            for branch in &node.branches {
                let label = branch.description.replace("\"", "\\\"");
                dot.push_str(&format!("    \"{}\" -> \"{}\" [label=\"{}\", color=blue];\n", 
                    node.id, branch.target, label));
            }
        }
    }
    
    dot.push_str("}\n");
    dot
}

pub fn export_to_mermaid(graph: &Graph, _paths: &[PathResult]) -> String {
    let mut mermaid = String::new();
    mermaid.push_str("flowchart TB\n");
    
    for (id, node) in &graph.nodes {
        let name = node.name.replace("\"", "\\\"");
        match node.node_type {
            crate::graph::NodeType::Action => {
                mermaid.push_str(&format!("    {0}[\"{1}\"]\n", id, name));
            }
            crate::graph::NodeType::Branch => {
                mermaid.push_str(&format!("    {0}{{\"{1}\"}}\n", id, name));
            }
            crate::graph::NodeType::End => {
                mermaid.push_str(&format!("    {0}(\"{1}\")\n", id, name));
            }
        }
    }
    
    mermaid.push_str("\n");
    
    for edge in &graph.edges {
        mermaid.push_str(&format!("    {} --> {}\n", edge.from, edge.to));
    }
    
    mermaid.push_str("\n");
    
    for node in graph.nodes.values() {
        if node.node_type == crate::graph::NodeType::Branch {
            for branch in &node.branches {
                let cond = branch.condition.as_deref().unwrap_or("");
                if !cond.is_empty() {
                    mermaid.push_str(&format!("    {} -->|{}| {}\n", node.id, cond, branch.target));
                } else {
                    mermaid.push_str(&format!("    {} --> {}\n", node.id, branch.target));
                }
            }
        }
    }
    
    mermaid
}