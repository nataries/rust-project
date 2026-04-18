use crate::graph::Graph;

pub fn validate(graph: &Graph) -> Vec<String> {
    let mut errors = Vec::new();
    
    for edge in &graph.edges {
        if !graph.nodes.contains_key(&edge.from) {
            errors.push(format!("Ребро из {} в {}: узел {} не существует", 
                edge.from, edge.to, edge.from));
        }
        if !graph.nodes.contains_key(&edge.to) {
            errors.push(format!("Ребро из {} в {}: узел {} не существует", 
                edge.from, edge.to, edge.to));
        }
    }
    
    for node in graph.nodes.values() {
        if node.node_type == crate::graph::NodeType::Branch {
            for branch in &node.branches {
                if !graph.nodes.contains_key(&branch.target) {
                    errors.push(format!("Узел {}: ветка '{}' ведёт к несуществующему узлу {}", 
                        node.id, branch.description, branch.target));
                }
            }
        }
    }
    
    errors
}
