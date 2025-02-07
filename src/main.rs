use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct ChatModels {
    preferred: HashMap<String, String>,
    others: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelList {
    models: HashMap<String, serde_json::Value>,
}

fn generate_csv(model_list: &ModelList) -> String {
    let mut csv = String::from("category,subcategory,model_name,model_id\n");
    
    for (category, models) in &model_list.models {
        match category.as_str() {
            "chat" => {
                let chat_models: ChatModels = serde_json::from_value(models.clone())
                    .expect("Failed to parse chat models");
                
                // Add preferred chat models
                for (name, id) in &chat_models.preferred {
                    csv.push_str(&format!("chat,preferred,{},{}\n", name, id));
                }
                
                // Add other chat models
                for (name, id) in &chat_models.others {
                    csv.push_str(&format!("chat,others,{},{}\n", name, id));
                }
            },
            _ => {
                let category_models: HashMap<String, String> = serde_json::from_value(models.clone())
                    .expect("Failed to parse models");
                
                for (name, id) in category_models {
                    csv.push_str(&format!("{},default,{},{}\n", category, name, id));
                }
            }
        }
    }
    csv
}

fn generate_markdown(model_list: &ModelList) -> String {
    let mut md = String::from("# AI Model List\n\n");
    
    // Memo Models
    if let Some(memo_models) = model_list.models.get("memo") {
        let memo: HashMap<String, String> = serde_json::from_value(memo_models.clone())
            .expect("Failed to parse memo models");
        
        md.push_str("## Memo Models\n\n");
        md.push_str("| Model Name | Model ID |\n");
        md.push_str("|------------|----------|\n");
        for (name, id) in memo {
            md.push_str(&format!("| {} | {} |\n", name, id));
        }
        md.push_str("\n");
    }
    
    // Chat Models
    if let Some(chat_models) = model_list.models.get("chat") {
        let chat: ChatModels = serde_json::from_value(chat_models.clone())
            .expect("Failed to parse chat models");
        
        md.push_str("## Chat Models\n\n");
        md.push_str("### Preferred Models\n\n");
        md.push_str("| Model Name | Model ID |\n");
        md.push_str("|------------|----------|\n");
        for (name, id) in chat.preferred {
            md.push_str(&format!("| {} | {} |\n", name, id));
        }
        
        md.push_str("\n### Other Models\n\n");
        md.push_str("| Model Name | Model ID |\n");
        md.push_str("|------------|----------|\n");
        for (name, id) in chat.others {
            md.push_str(&format!("| {} | {} |\n", name, id));
        }
        md.push_str("\n");
    }
    
    // Graph Models
    if let Some(graph_models) = model_list.models.get("graph") {
        let graph: HashMap<String, String> = serde_json::from_value(graph_models.clone())
            .expect("Failed to parse graph models");
        
        md.push_str("## Graph Models\n\n");
        md.push_str("| Model Name | Model ID |\n");
        md.push_str("|------------|----------|\n");
        for (name, id) in graph {
            md.push_str(&format!("| {} | {} |\n", name, id));
        }
    }
    
    md
}

fn main() {
    // Read the JSON file
    let json_content = fs::read_to_string("models.json")
        .expect("Failed to read models.json");
    
    let model_list: ModelList = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON");

    // Create generated directory if it doesn't exist
    fs::create_dir_all("generated").expect("Failed to create generated directory");

    // Generate CSV
    let csv_content = generate_csv(&model_list);
    fs::write("generated/models.csv", csv_content)
        .expect("Failed to write CSV");

    // Generate Markdown
    let md_content = generate_markdown(&model_list);
    fs::write("generated/models.md", md_content)
        .expect("Failed to write Markdown");

    println!("Successfully generated files in ./generated directory");
}