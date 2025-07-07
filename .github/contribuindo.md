# Guia de Contribuição
Neste documento, constam os padrões adotados para o desenvolvimento do sistema do
Projetário UTFPR, bem como guias de instalação e sobre a arquitetura de diretórios.

## Setup
Para contribuir, é necessário ter instalado os seguintes softwares:
- [Rust e sua toolchain](https://www.rust-lang.org/pt-BR/tools/install);
    - Instale, também, a toolchain nightly + clippy com o comando `rustup toolchain install nightly ; rustup +nightly component add clippy`
- [Docker](https://www.docker.com/);
- [Node.js](https://nodejs.org/en).

### Configurando o ambiente de desenvolvimento
1. Copie as configurações bases do vscode com o comando `cp .vscode/sample.settings.json .vscode/settings.json`.

## Extensões (VS Code)
As extensões listadas adiante permitem um desenvolvimento mais simplificado e com mais corretude.
Por exemplo, autocomplete, formatação automática, entre outras regras de linting que podem barrar o
código se não corrigidas.

- [Biome](https://marketplace.visualstudio.com/items?itemName=biomejs.biome)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [npm intellisense](https://marketplace.visualstudio.com/items?itemName=christian-kohler.npm-intellisense)

## Testes
Para rodar testes, utilize o comando:
```bash
cargo test
```

## Rodando em desenvolvimento
### Variáveis de ambiente
Rode `cp .env.sample .env` e preencha os campos necessário.

#### Gere uma `APP_KEY`
Essa variável é utilizada para assinar os cookies de sessão. Gere-a utilizando um dos comandos abaixo
e coloque o resultado como o valor desta chave no arquivo `.env`:

##### Powershell
```powershell
[Convert]::ToBase64String((1..64 | ForEach-Object { Get-Random -Minimum 0 -Maximum 256 }) -as [byte[]])

# ou injete diretamente no arquivo .env

[Convert]::ToBase64String((1..64 | ForEach-Object { Get-Random -Minimum 0 -Maximum 256 }) -as [byte[]]) >> .env
```
##### Shell
```shell
head -c 64 /dev/urandom | base64

# ou injete diretamente no arquivo .env

head -c 64 /dev/urandom | base64 -w 0 >> .env
```

### Inicie o servidor do font-end em um processo (terminal)
```bash
npm install
npm run dev
```

### Inicie o servidor do back-end em outro processo
```bash
docker compose up -d # ergue os serviços necessários para rodar a aplicação
cargo run
```
