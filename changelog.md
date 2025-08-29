# Changelog
Nesse arquivo constam as alterações realizadas ao longo das versões do Projetário.

## Não lançado
### Adicionados
* Temas — claro, escuro e padrão do sistema — agora funcionam e podem ser alterados pelo usuário
    (PR [#25], issue [#9])
* Footer foi adicionado ao layout

### Removidos
* Serviço, fábrica e controller de buscar projetos

### Mudanças
* É possível selecionar o projeto (de extensão) para o qual se quer criar vagas por meio de um
    dropdown de select — anteriormente, era necessário inserir o ID do projeto diretamente
* O domínio de vagas (das entidades aos serviços) foi completamente remodelado (PR [#21], issue [#19])

### Correções
* Agora é possível iniciar o programa com o SSR ativado quando em produção: anteriormente,
    resultaria no erro "Error: Ssr is not enabled and, hence, a ssr server cannot be raised."
* Todas as páginas e componentes — existentes até o momento — agora suportam o modo escuro [#26]

[#26]: https://github.com/Projetario-UTFPR/projetario/pull/26
[#25]: https://github.com/Projetario-UTFPR/projetario/pull/25
[#9]: https://github.com/Projetario-UTFPR/projetario/pull/9
[#21]: https://github.com/Projetario-UTFPR/projetario/pull/21
[#19]: https://github.com/Projetario-UTFPR/projetario/pull/19

## Projetário v0.1.0
### Adicionados
* Agora é possível se autenticar [#10], criar um projeto de extensão [#8] e criar vagas para um projeto [#20]
* Services para cancelar e alterar vagas [#18]
* Código base para buscar vagas de projetos [#15]
* Workflows de testes e linting

[#10]: https://github.com/Projetario-UTFPR/projetario/pull/10
[#8]: https://github.com/Projetario-UTFPR/projetario/pull/8
[#20]: https://github.com/Projetario-UTFPR/projetario/pull/20
[#18]: https://github.com/Projetario-UTFPR/projetario/pull/18
[#15]: https://github.com/Projetario-UTFPR/projetario/pull/15


### Novos Contribuidores
* @dotwolf em https://github.com/Projetario-UTFPR/projetario/pull/13
* @JoaoR090 em https://github.com/Projetario-UTFPR/projetario/pull/15
* @Guedinsi em https://github.com/Projetario-UTFPR/projetario/pull/18

**Changelog completo:** https://github.com/Projetario-UTFPR/projetario/commits/v0.1.0