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


