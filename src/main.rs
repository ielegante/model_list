use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct ModelList {
    models: HashMap<String, HashMap<String, String>>,
}

fn generate_csv(model_list: &ModelList) -> String {
    let mut csv = String::from("category,model_name,model_id\n");
    
    for (category, models) in &model_list.models {
        for (name, id) in models {
            csv.push_str(&format!("{},{},{}\n", category, name, id));
        }
    }
    csv
}

fn generate_markdown(model_list: &ModelList) -> String {
    let mut md = String::from("# AI Model List\n\n");
    
    for (category, models) in &model_list.models {
        md.push_str(&format!("## {} Models\n\n", category.to_uppercase()));
        md.push_str("| Model Name | Model ID |\n");
        md.push_str("|------------|----------|\n");
        
        for (name, id) in models {
            md.push_str(&format!("| {} | {} |\n", name, id));
        }
        md.push_str("\n");
    }
    md
}

fn main() {
    // Read the JSON file
    let json_content = fs::read_to_string("models.json")
        .expect("Failed to read models.json");
    
    let model_list: ModelList = serde_json::from_str(&json_content)
        .expect("Failed to parse JSON");

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