use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

/// Representa uma linha lida de um arquivo
#[derive(Debug)]
pub struct FileLine {
    pub file: PathBuf,
    pub line_number: usize,
    pub content: String,
}

/// Lê todos os arquivos de texto em um diretório e coleta suas linhas
pub fn read_text_files(dir: &Path) -> io::Result<Vec<FileLine>> {
    let mut results = Vec::new();

    println!("🔍 Verificando diretório: {}", dir.display());

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            println!("➡️ Encontrado: {}", path.display());

            if path.is_dir() {
                println!("📁 É um subdiretório. Entrando...");
                let sub_results = read_text_files(&path)?;
                results.extend(sub_results);
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let ext = ext.to_lowercase();
                if ext == "txt" || ext == "md" {
                    println!("📄 Arquivo válido para leitura: {}", path.display());

                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);

                    for (i, line) in reader.lines().enumerate() {
                        let line = line?;
                        println!("✅ Linha {} lida de {}: {}", i + 1, path.display(), line);

                        results.push(FileLine {
                            file: path.clone(),
                            line_number: i + 1,
                            content: line,
                        });
                    }
                } else {
                    println!("🚫 Ignorado (extensão inválida): {}", path.display());
                }
            } else {
                println!("🚫 Ignorado (sem extensão): {}", path.display());
            }
        }
    } else {
        println!("❌ O caminho fornecido não é um diretório: {}", dir.display());
    }

    println!("📦 Total de linhas coletadas: {}", results.len());

    Ok(results)
}
