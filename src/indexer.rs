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
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir; // Biblioteca útil para testes com arquivos temporários

    // Teste helper: cria arquivos temporários para testes
    fn create_test_files(dir: &Path, files: &[(&str, &str)]) {
        for (name, content) in files {
            let path = dir.join(name);
            let mut file = File::create(path).unwrap();
            writeln!(file, "{}", content).unwrap();
        }
    }

    #[test]
    fn test_read_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let result = read_text_files(temp_dir.path()).unwrap();
        assert_eq!(result.len(), 0, "Deveria retornar vetor vazio para diretório vazio");
    }

    #[test]
    fn test_read_single_text_file() {
        let temp_dir = tempdir().unwrap();
        create_test_files(temp_dir.path(), &[("test.txt", "linha 1\nlinha 2")]);
        
        let result = read_text_files(temp_dir.path()).unwrap();
        assert_eq!(result.len(), 2, "Deveria ler todas as linhas do arquivo");
        
        assert_eq!(result[0].content, "linha 1");
        assert_eq!(result[0].line_number, 1);
        assert_eq!(result[1].content, "linha 2");
        assert_eq!(result[1].line_number, 2);
    }

    #[test]
    fn test_filter_only_txt_and_md_files() {
        let temp_dir = tempdir().unwrap();
        create_test_files(temp_dir.path(), &[
            ("valid.txt", "conteúdo"),
            ("valid.md", "markdown"),
            ("invalid.rust", "não deve ser lido"),
            ("invalid.csv", "não deve ser lido")
        ]);
        
        let result = read_text_files(temp_dir.path()).unwrap();
        assert_eq!(result.len(), 2, "Deveria ler apenas .txt e .md");
    }

    #[test]
    fn test_read_recursive_directories() {
        let temp_dir = tempdir().unwrap();
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();
        
        create_test_files(temp_dir.path(), &[("root.txt", "root file")]);
        create_test_files(&sub_dir, &[("sub.txt", "sub file")]);
        
        let result = read_text_files(temp_dir.path()).unwrap();
        assert_eq!(result.len(), 2, "Deveria ler arquivos recursivamente");
    }

    #[test]
    fn test_empty_lines_are_included() {
        let temp_dir = tempdir().unwrap();
        create_test_files(temp_dir.path(), &[("empty.txt", "\n\n")]);
        
        let result = read_text_files(temp_dir.path()).unwrap();
        assert_eq!(result.len(), 2, "Deveria incluir linhas vazias");
        assert_eq!(result[0].content, "");
    }

    #[test]
    fn test_error_on_nonexistent_directory() {
        let temp_dir = tempdir().unwrap();
        let nonexistent_path = temp_dir.path().join("nonexistent");
        
        let result = read_text_files(&nonexistent_path);
        assert!(result.is_err(), "Deveria retornar erro para diretório inexistente");
    }

    #[test]
    fn test_file_metadata_correct() {
        let temp_dir = tempdir().unwrap();
        create_test_files(temp_dir.path(), &[("meta.txt", "conteúdo")]);
        
        let result = read_text_files(temp_dir.path()).unwrap();
        assert_eq!(result[0].file.file_name().unwrap(), "meta.txt");
        assert_eq!(result[0].line_number, 1);
    }
}
