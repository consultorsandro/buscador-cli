use clap::Parser; // Importa o parser do Clap para análise de argumentos de linha de comando

/// Configuração da linha de comando
#[derive(Parser, Debug)]
#[command(name = "buscador")]
#[command(about = "Motor de busca simples para arquivos de texto", long_about = None)]
pub struct Config {
    /// Diretório a ser indexado
    #[arg(short, long, default_value = ".")]
    pub dir: String,

    /// Palavra ou termo a buscar
    #[arg(short, long, default_value = "")]
    pub query: String,

    /// Sensível a maiúsculas/minúsculas?
    #[arg(long, default_value_t = false)] // por padrão não é sensível a maiúsculas/minúsculas
    pub case_sensitive: bool,
}


#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_default_config() { 
        let args = vec!["buscador"];
        let config = Config::parse_from(args);
        
        assert_eq!(config.dir, ".");
        assert_eq!(config.query, "");
        assert_eq!(config.case_sensitive, false);
    }

    #[test]
    fn test_custom_directory() {
        let args = vec!["buscador", "--dir", "/home/user/docs"];
        let config = Config::parse_from(args);
        
        assert_eq!(config.dir, "/home/user/docs");
        assert_eq!(config.query, "");
        assert_eq!(config.case_sensitive, false);
    }

    #[test]
    fn test_query_argument() {
        let args = vec!["buscador", "--query", "example"];
        let config = Config::parse_from(args);
        
        assert_eq!(config.dir, ".");
        assert_eq!(config.query, "example");
        assert_eq!(config.case_sensitive, false);
    }

    #[test]
    fn test_case_sensitive_flag() {
        let args = vec!["buscador", "--case-sensitive"];
        let config = Config::parse_from(args);
        
        assert_eq!(config.dir, ".");
        assert_eq!(config.query, "");
        assert_eq!(config.case_sensitive, true);
    }

    #[test]
    fn test_short_options() {
        let args = vec!["buscador", "-d", "/tmp", "-q", "test"];
        let config = Config::parse_from(args);
        
        assert_eq!(config.dir, "/tmp");
        assert_eq!(config.query, "test");
        assert_eq!(config.case_sensitive, false);
    }

    #[test]
    fn test_all_options() {
        let args = vec!["buscador", "-d", "/var/log", "-q", "error", "--case-sensitive"];
        let config = Config::parse_from(args);
        
        assert_eq!(config.dir, "/var/log");
        assert_eq!(config.query, "error");
        assert_eq!(config.case_sensitive, true);
    }
}