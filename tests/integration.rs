// filepath: buscador-cli/tests/integration.rs
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_integration_search_functionality() {
    let temp_dir = tempdir().unwrap();
    let test_dir_path = temp_dir.path();

    // Create test files
    let files = vec![
        ("file1.txt", "This is a test file.\nIt contains some text."),
        ("file2.md", "Markdown content here.\nAnother line."),
        ("file3.txt", "Another test file with different content."),
    ];

    for (name, content) in files {
        let path = test_dir_path.join(name);
        let mut file = fs::File::create(path).unwrap();
        writeln!(file, "{}", content).unwrap();
    }

    // Run the main application with the test directory and a search term
    let output = Command::new("cargo")
        .args(&["run", "--", "--dir", test_dir_path.to_str().unwrap(), "--query", "test"])
        .output()
        .expect("Failed to execute command");

    // Check if the output contains expected results
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("✅ 2 resultados encontrados:"), "Deveria encontrar 2 resultados");
    assert!(stdout.contains("file1.txt:1 -> This is a test file."), "Deveria incluir a linha do file1.txt");
    assert!(stdout.contains("file3.txt:1 -> Another test file with different content."), "Deveria incluir a linha do file3.txt");
}

#[test]
fn test_integration_no_results() {
    let temp_dir = tempdir().unwrap();
    let test_dir_path = temp_dir.path();

    // Create a test file
    let path = test_dir_path.join("empty.txt");
    let mut file = fs::File::create(path).unwrap();
    writeln!(file, "This file does not contain the search term.").unwrap();

    // Run the main application with the test directory and a search term that does not exist
    let output = Command::new("cargo")
        .args(&["run", "--", "--dir", test_dir_path.to_str().unwrap(), "--query", "nonexistent"])
        .output()
        .expect("Failed to execute command");

    // Check if the output indicates no results found
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("🔎 Nenhum resultado encontrado para \"nonexistent\"."), "Deveria indicar que nenhum resultado foi encontrado");
}