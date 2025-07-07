# Projetário
Encontre projetos de extensão e pesquisas de iniciação científicas com vagas abertas,
ou seu mentor para o TCC!

## Objetivos
Todos os requisitos funcionais e não-funcionais, os quais guiam o desenvolvimento
deste sistema, estão disponíveis no documento de
[Objetivos e Requisitos](.github/requisitos.md).

## Rodando Localmente
Para rodar, será necessário ter o [Docker](https://docs.docker.com/get-started/get-docker/)
à disposição, bem como o [GIT](https://git-scm.com/book/pt-br/v2/Come%C3%A7ando-Instalando-o-Git).

Feito isso, siga o passo-a-passo adiante:
1. Clone este repositório com o comando `git clone git@github.com:Projetario-UTFPR/projetario.git projetario`
2. Navegue para o diretório do repositório: `cd projetario`
3. Adicione as variáveis de ambiente: `cp .env.sample .env`
4. Crie a variável `APP_KEY` com o comando `head -c 64 /dev/urandom | base64 -w 0 >> .env`
5. Gere a imagem Docker: `docker build -t projetario/app:0.1.0 .` (observe que o ponto
é necessário)
6. Levante os serviços com o comando `docker compose --profile preview up -d`

Feito isso, basta acessar o sistema através da URL http://localhost:3000.

Digite `docker compose stop` para parar os containers ou `docker compose down` para destruí-los.

## Contribuindo
Para contribuir com o desenvolvimento do Projetário UTFPR, leia o
[Guia de Contribuição](.github/contribuindo.md).

## Licença
O Projetário UTFPR é open-source e pode ser distribuído sob a [licença MIT](./LICENSE).
