mod graph;
mod parser;
mod validator;
mod analyzer;

use clap::{Parser, Subcommand};
use colored::*;
use std::path::Path;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "graph-constructor")]
#[command(version = "1.0")]
#[command(about = "Конструктор и анализатор графов сценариев", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Analyze {
        #[arg(short = 'f', long)]
        file: String,
        
        #[arg(short = 's', long)]
        start: String,
        
        #[arg(short = 'e', long)]
        end: String,
        
        #[arg(short = 'd', long, default_value_t = 100)]
        depth: usize,
        
        #[arg(long, default_value = "mermaid")]
        format: String,
        
        #[arg(short = 'o', long)]
        output: Option<String>,
    },
    
    
    Info {
        #[arg(short = 'f', long)]
        file: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run(cli).await {
        eprintln!("{} {}", "Ошибка:".red().bold(), e.red());
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Commands::Analyze { file, start, end, depth, format, output } => {
            println!("{} {}", "Загрузка графа из:".green(), file);
            
            let path = Path::new(&file);
            let graph = parser::load_graph(path)?;
            
            // Валидация
            let errors = validator::validate(&graph);
            if !errors.is_empty() {
                println!("{}", "Предупреждения:".yellow());
                for err in errors {
                    println!("  {}", err.yellow());
                }
            }
            
            let start_nodes: Vec<String> = start.split(',')
                .map(|s| s.trim().to_string())
                .collect();
            let end_nodes: Vec<String> = end.split(',')
                .map(|s| s.trim().to_string())
                .collect();
            
            println!("{}", "Анализ путей...".cyan());
            let graph_arc = Arc::new(graph);
            let paths = analyzer::find_paths(graph_arc.clone(), start_nodes, end_nodes, depth).await;
            
            // Вывод результатов
            if paths.is_empty() {
                println!("{}", "Пути не найдены".yellow());
            } else {
                println!("{} {}", "Найдено путей:".green(), paths.len());
                for (i, path) in paths.iter().enumerate() {
                    println!("\n{} {}:", "Путь".cyan(), i + 1);
                    println!("  {}", path.nodes.join(" -> "));
                    if !path.conditions.is_empty() {
                        println!("  {} {}", "Условия:".purple(), path.conditions.join(" AND "));
                    }
                }
            }
            
            if let Some(out_file) = output {
                let export = if format == "dot" {
                    analyzer::export_to_dot(&graph_arc, &paths)
                } else {
                    analyzer::export_to_mermaid(&graph_arc, &paths)
                };
                
                std::fs::write(&out_file, export)
                    .map_err(|e| format!("Не удалось записать файл: {}", e))?;
                println!("\n{} {}", "Экспортировано в:".green(), out_file);
            }
        }Commands::Info { file } => {
            println!("{} {}", "Информация о графе из:".green(), file);
            
            let path = Path::new(&file);
            let graph = parser::load_graph(path)?;
            
            println!("\n{}", "Узлы:".cyan().bold());
            for (id, node) in &graph.nodes {
                let node_type = match node.node_type {
                    graph::NodeType::Action => "Действие",
                    graph::NodeType::Branch => "Ветвление",
                    graph::NodeType::End => "Конец",
                };
                println!("  {} [{}]: {}", id, node_type, node.name);
                if let Some(desc) = &node.description {
                    println!("    Описание: {}", desc);
                }
                if node.node_type == graph::NodeType::Branch {
                    for branch in &node.branches {
                        println!("    -> {}: {}", branch.target, branch.description);
                        if let Some(cond) = &branch.condition {
                            println!("       Условие: {}", cond);
                        }
                    }
                }
            }
            
            println!("\n{}", "Рёбра:".cyan().bold());
            for edge in &graph.edges {
                println!("  {} -> {}", edge.from, edge.to);
                if let Some(desc) = &edge.description {
                    println!("    Описание: {}", desc);
                }
                if let Some(cond) = &edge.condition {
                    println!("    Условие: {}", cond);
                }
            }
            
            let start_nodes = graph.get_start_nodes();
            println!("\n{} {:?}", "Начальные узлы:".cyan(), start_nodes);
            
            let end_nodes = graph.get_end_nodes();
            println!("{} {:?}", "Конечные узлы:".cyan(), end_nodes);
        }
    }
    
    Ok(())
}

