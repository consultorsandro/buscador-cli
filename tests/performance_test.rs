use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;
use std::time::Instant;

/// Teste de performance: cria um arquivo grande e mede o tempo de busca
#[test]
fn test_performance_large_file_search() {
    let temp_dir = tempdir().unwrap();
    let test_dir_path = temp_dir.path();

    // Cria um arquivo grande com 200_000 linhas, todas contendo a palavra "performance"
    let path = test_dir_path.join("large.txt");
    let mut file = File::create(&path).unwrap();
    for _ in 0..200_000 {
        writeln!(file, "Linha de teste para performance.").unwrap();
    }

    // Mede o tempo de execução da busca
    let start = Instant::now();
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "--dir", test_dir_path.to_str().unwrap(), "--query", "performance"])
        .output()
        .expect("Falha ao executar o comando");
    let duration = start.elapsed();

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("✅"), "Deveria encontrar resultados no arquivo grande");
    // O teste falha se demorar mais de 10 segundos (ajuste conforme necessário)
    assert!(
        duration.as_secs() < 10,
        "A busca demorou mais de 10 segundos! Tempo: {:?}",
        duration
    );
}