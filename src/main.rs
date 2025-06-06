mod config;

use config::Config;
use clap::Parser;

fn main() {
    let args = Config::parse();

    println!("📁 Diretório: {}", args.dir);
    println!("🔍 Termo: {}", args.query);
    println!("🔠 Case sensitive: {}", args.case_sensitive);
}
// This is a simple CLI application that uses the `clap` crate to parse command line arguments.