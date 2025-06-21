mod config;
mod indexer;

use clap::Parser;
use config::Coinfg;
use indexer::read_text_files;
use std::path::Path;
use std::io::{self, Write};

fn main() {
    println!("👋 Bem-vindo ao Buscador CLI! 🚀");
    let mut args = Config::parse();

    if args.query.is_empty() {
        print!("Por favor, digite o termo de busca: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        args.query = input.trim().to_string();

        if args.query.is_empty() {
            eprintln!("⚠️  Nenhum termo de busca informado. Encerrando o programa.");
            std::process::exit(1);
        }
    }

    println!("📁 Diretório: {}", args.dir);
    println!("🔍 Termo: {}", args.query);
    println!("🔠 Case sensitive: {}", args.case_sensitive);

    let path = Path::new(&args.dir);

        // ...código existente...
    
        match read_text_files(path) {
            Ok(lines) => {
                println!("📄 {} linhas lidas para indexação", lines.len());
    
                let termo = &args.query;
                let case_sensitive = args.case_sensitive;
    
                let resultados: Vec<_> = lines
                    .into_iter()
                    .filter(|line| {
                        if case_sensitive {
                            line.content.contains(termo)
                        } else {
                            line.content.to_lowercase().contains(&termo.to_lowercase())
                        }
                    })
                    .collect();
    
                if resultados.is_empty() {
                    println!("🔎 Nenhum resultado encontrado para \"{}\".", termo);
                } else {
                    println!("✅ {} resultados encontrados:", resultados.len());
                    for line in resultados.iter().take(15) {
                        println!(
                            "{}:{} -> {}",
                            line.file.display(),
                            line.line_number,
                            line.content
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Erro ao ler arquivos: {}", e);
            }
        }
    }
    // ...código existente...
   