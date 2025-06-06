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

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Chamada recursiva para subdiretórios
                let sub_results = read_text_files(&path)?;
                results.extend(sub_results);
            } else if let Some(ext) = path.extension() {
                if ext == "txt" || ext == "md" {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);

                    for (i, line) in reader.lines().enumerate() {
                        let line = line?;
                        results.push(FileLine {
                            file: path.clone(),
                            line_number: i + 1,
                            content: line,
                        });
                    }
                }
            }
        }
    }

    Ok(results)
}
/// Busca por um termo em uma lista de linhas de arquivos