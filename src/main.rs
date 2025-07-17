mod config;
mod indexer;

use indexer::read_text_files;
use std::path::Path;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    
    // Clone para usar nos callbacks
    let ui_weak = ui.as_weak();
    
    // Callback para buscar
    ui.on_search_clicked(move || {
        let ui = ui_weak.unwrap();
        let search_term = ui.get_search_term().to_string();
        let directory = ui.get_directory().to_string();
        let case_sensitive = ui.get_case_sensitive();
        
        if search_term.is_empty() {
            ui.set_status_message("⚠️ Digite um termo de busca".into());
            return;
        }
        
        ui.set_status_message("🔄 Buscando...".into());
        
        // Realizar busca
        match perform_search(&directory, &search_term, case_sensitive) {
            Ok(results) => {
                let result_strings: Vec<slint::SharedString> = results
                    .into_iter()
                    .map(|r| format!("{}:{} -> {}", r.file.display(), r.line_number, r.content).into())
                    .collect();
                
                ui.set_results(slint::ModelRc::from(result_strings.as_slice()));
                ui.set_status_message(format!("✅ {} resultados encontrados", result_strings.len()).into());
            }
            Err(e) => {
                ui.set_status_message(format!("❌ Erro: {}", e).into());
            }
        }
    });
    
    // Callback para limpar
    let ui_weak = ui.as_weak();
    ui.on_clear_clicked(move || {
        let ui = ui_weak.unwrap();
        ui.set_search_term("".into());
        ui.set_results(Default::default());
        ui.set_status_message("Pronto para buscar".into());
    });
    
    ui.run()
}

fn perform_search(directory: &str, search_term: &str, case_sensitive: bool) -> Result<Vec<indexer::FileLine>, Box<dyn std::error::Error>> {
    let path = Path::new(directory);
    let lines = read_text_files(path)?;
    
    let results: Vec<_> = lines
        .into_iter()
        .filter(|line| {
            if case_sensitive {
                line.content.contains(search_term)
            } else {
                line.content.to_lowercase().contains(&search_term.to_lowercase())
            }
        })
        .collect();
    
    Ok(results)
}