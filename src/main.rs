mod config;
mod indexer;

use clap::Parser;
use config::Config;
use indexer::read_text_files;
use std::path::Path;

fn main() {
    let args = Config::parse();

    println!("📁 Diretório: {}", args.dir);
    println!("🔍 Termo: {}", args.query);
    println!("🔠 Case sensitive: {}", args.case_sensitive);

    let path = Path::new(&args.dir);

    match read_text_files(path) {
        Ok(lines) => {
            println!("📄 {} linhas lidas para indexação", lines.len());
            for line in &lines[0..lines.len().min(5)] {  // Mostra só as 5 primeiras
                println!(
                    "{}:{} -> {}",
                    line.file.display(),
                    line.line_number,
                    line.content
                );
            }
        }
        Err(e) => {
            eprintln!("❌ Erro ao ler arquivos: {}", e);
        }
    }
}

