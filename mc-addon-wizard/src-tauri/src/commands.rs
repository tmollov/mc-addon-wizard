use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct ItemForm {
    pub identifier: String,
    pub category: String,
    pub damage: i32,
    pub durability: i32,
    pub stack_size: i32,
    pub texture: String,
}

#[tauri::command]
pub fn generate_item(form: ItemForm, output_dir: String) -> Result<String, String> {
    // Make sure the directory exists
    let items_dir = format!("{}/BP/items", output_dir); //
    fs::create_dir_all(&items_dir).map_err(|e| format!("Failed to create folder: {}", e))?;

    let template = r#"
    {
      "format_version": "1.20.0",
      "minecraft:item": {
        "description": {
          "identifier": "{{identifier}}",
          "category": "{{category}}"
        },
        "components": {
          "minecraft:icon": "{{texture}}",
          "minecraft:max_stack_size": {{stack_size}},
          "minecraft:durability": {{durability}},
          "minecraft:damage": {{damage}}
        }
      }
    }
    "#;

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("item", template)
        .map_err(|e| e.to_string())?;

    let json = handlebars
        .render("item", &form)
        .map_err(|e| e.to_string())?;
    let file_path = format!("{}/{}.json", items_dir, form.identifier);

    fs::write(&file_path, json).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(format!("File created: {}", file_path))
}
