# Guia de Contribuição
Neste documento, constam os padrões adotados para o desenvolvimento do sistema do
Projetário UTFPR, bem como guias de instalação e sobre a arquitetura de diretórios.

## Setup
Para contribuir, é necessário ter instalado os seguintes softwares:
- [Rust e sua toolchain](https://www.rust-lang.org/pt-BR/tools/install);
- [Docker](https://www.docker.com/);
- [Node.js](https://nodejs.org/en).

## Testes
Para rodar testes, utilize o comando:
```bash
cargo test
```

## Rodando em desenvolvimento
```bash
docker compose up -d # ergue os serviços necessários para rodar a aplicação
cargo run
```

