use clap::Parser;

/// Configuração da linha de comando
#[derive(Parser, Debug)]
#[command(name = "buscador")]
#[command(about = "Motor de busca simples para arquivos de texto", long_about = None)]
pub struct Config {
    /// Diretório a ser indexado
    #[arg(short, long)]
    pub dir: String,

    /// Palavra ou termo a buscar
    #[arg(short, long)]
    pub query: String,

    /// Sensível a maiúsculas/minúsculas?
    #[arg(long, default_value_t = false)]
    pub case_sensitive: bool,
}
