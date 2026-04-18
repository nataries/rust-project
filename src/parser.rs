use crate::graph::{Graph, Node, Edge};
use std::fs;
use std::path::Path;

pub fn load_graph(path: &Path) -> Result<Graph, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Не удалось прочитать файл: {}", e))?;
    
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    
    let data: serde_json::Value = if ext == "yaml" || ext == "yml" {
        serde_yaml::from_str(&content)
            .map_err(|e| format!("Ошибка парсинга YAML: {}", e))?
    } else {
        serde_json::from_str(&content)
            .map_err(|e| format!("Ошибка парсинга JSON: {}", e))?
    };
    
    let mut graph = Graph::new();
    
    if let Some(nodes) = data.get("nodes").and_then(|n| n.as_array()) {
        for node_val in nodes {
            let node: Node = serde_json::from_value(node_val.clone())
                .map_err(|e| format!("Ошибка парсинга узла: {}", e))?;
            graph.add_node(node)?;
        }
    }
    
    if let Some(edges) = data.get("edges").and_then(|e| e.as_array()) {
        for edge_val in edges {
            let edge: Edge = serde_json::from_value(edge_val.clone())
                .map_err(|e| format!("Ошибка парсинга ребра: {}", e))?;
            graph.add_edge(edge)?;
        }
    }
    
    Ok(graph)
}

