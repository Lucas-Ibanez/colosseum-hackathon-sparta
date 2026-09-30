# Arquitetura do MVP

## Estado D1c1

A primeira implementação existe em `crates/vericode-core`. É uma crate Rust
pura, `std`-only, que modela `Hash32`, `JobId`, `ImageId`, `Verdict`, os
compromissos esperados e `JournalV1`. Ela valida igualdade semântica dos
compromissos e trata `PASS` e `FAIL` como resultados normais.

Essa crate não serializa o journal, não calcula hashes, não executa artefato,
não contém lógica Solana/Anchor/RISC Zero e não autoriza pagamentos. O único
fixture de teste é um registro de desenvolvimento restrito e versionado; ele
não representa suporte a código ou repositórios arbitrários.

## Fluxo

`Buyer cria Job e deposita Test USDC` -> `Executor fornece artefato restrito` -> `host executa o harness e gera receipt` -> `contrato valida prova e journal contra o Job` -> `release ao executor em PASS válido ou refund conforme as regras do Job`

O fluxo acima é alvo de arquitetura, não evidência de integração já funcional. As condições exatas de timeout e refund ainda precisam de especificação.

## Componentes e responsabilidades

| Componente | Responsabilidade | Não deve fazer |
| --- | --- | --- |
| Core Rust puro | Tipos de domínio, regra determinística, validação do artefato e `Verdict` | Depender de Solana, Anchor ou RISC Zero |
| Guest RISC Zero | Ler entrada restrita, chamar o core e publicar `JournalV1` com `PASS` ou `FAIL` | Tratar `FAIL` como panic/assert ou expor dados privados desnecessários |
| Host | Preparar entrada, executar/provar, obter receipt e conferir journal localmente | Ser fonte de verdade para liberar fundos |
| Programa Anchor | Manter Job/escrow e autorizar release/refund somente após validações do Job | Reexecutar regra de negócio, aceitar admin bypass ou confiar apenas no host |
| Router/verificador | Verificar receipt Groth16 por CPI, se a integração Solana for comprovada | Ser considerado disponível sem deployment e CPI validados |
| Front-end | Criar e consultar Jobs e apresentar estados/transações | Decidir verdict ou custodiar segredos do usuário |

## Fronteiras

- O core é a única implementação da regra de negócio e deve compilar como Rust puro.
- Guest e programa Anchor são adaptadores separados para alvos incompatíveis; nenhum importa APIs do outro.
- O host é não confiável para a decisão final: ele transporta artefato, receipt e journal.
- O contrato valida identidade do Job, destinatário, mint, compromissos críticos e, quando comprovado, a prova.
- O front-end nunca substitui validação do contrato.

## Fora do escopo deliberado

- Repositórios arbitrários ou patches gerais.
- Testes que dependem de rede durante a avaliação do artefato.
- Marketplace de tarefas.
- Sistema de reputação.
- Juiz baseado em LLM.
