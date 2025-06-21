# 🚀 Buscador CLI

![Rust](https://img.shields.io/badge/Rust-Programming%20Language-orange?logo=rust)
![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![Contributions Welcome](https://img.shields.io/badge/contributions-welcome-blue)

**Buscador CLI** é uma aplicação de linha de comando desenvolvida em **Rust** 🦀 para **indexação rápida** e **busca eficiente** de arquivos de texto (`.txt`, `.md`) em diretórios locais.

👉 Permite **buscas por palavras ou frases**, com **opção de busca sensível a maiúsculas/minúsculas**, exibindo o resultado de forma clara e detalhada.

---

## 📌 Destaques

✅ **Indexação automática** de arquivos de texto (com busca recursiva)
✅ **Busca rápida** por **palavras** ou **frases completas**
✅ Exibe **nome do arquivo**, **número da linha** e **conteúdo da linha**
✅ Suporte opcional a **Case Sensitive**
✅ **Interface amigável** no terminal
✅ Inclui **testes automatizados de integração e performance**

---

## 📥 Instalação

### 🧰 Pré-requisitos:

* 🦀 [Rust](https://www.rust-lang.org/tools/install) (versão estável)
* 📦 Cargo (já vem junto com o Rust)

### 📲 Passos:

```bash
# Clone o repositório
git clone https://github.com/consultorsandro/buscador-cli.git
cd buscador-cli

# Compile o projeto
cargo build --release
```

---

## 🚀 Como Usar

### 🔑 Comando básico:

```bash
cargo run --release -- --dir <diretório> --query <termo> [--case-sensitive]
```

### 📋 Opções:

| Parâmetro          | Descrição                                                        |
| ------------------ | ---------------------------------------------------------------- |
| `--dir`            | 📁 Caminho do diretório a ser indexado (padrão: diretório atual) |
| `--query`          | 🔍 Palavra ou frase a ser buscada (obrigatório)                  |
| `--case-sensitive` | 🔠 Ativa busca **sensível a maiúsculas/minúsculas**              |

### 💡 Exemplo de uso:

```bash
cargo run --release -- --dir ./documentos --query "Rust" --case-sensitive
```

> ℹ️ Se o termo de busca não for informado, o programa solicitará interativamente.

---

## 🧪 Testes

### ✅ Executar todos os testes unitários e de integração:

```bash
cargo test
```

### 🚀 Rodar apenas o teste de performance:

```bash
cargo test test_performance_large_file_search
```

---

## 🧹 Qualidade de Código

Utilize o [Clippy](https://github.com/rust-lang/rust-clippy) para análise de linting e sugestões de melhoria de código:

```bash
cargo clippy
```

---

## 📂 Estrutura do Projeto

```
buscador-cli/
├── src/
│   ├── main.rs
│   ├── config.rs
│   └── indexer.rs
├── tests/
│   ├── integration.rs
│   └── performance_test.rs
├── Cargo.toml
└── README.md
```

---

## 🌱 Contribuições

Contribuições são **muito bem-vindas**! 😄

📌 **Como contribuir:**

* Abra uma **issue** com sugestões ou problemas encontrados
* Envie um **Pull Request** com correções ou novas funcionalidades
* Siga as boas práticas de código Rust e use o Clippy antes de submeter

---

## 📜 Licença

Este projeto está licenciado sob a [MIT License](LICENSE).

---

## 👨‍💻 Autor

Desenvolvido por [Sandro Reis](https://github.com/consultorsandro) com Rust 🦀 e prompts de IA.

---
