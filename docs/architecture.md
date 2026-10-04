# Arquitetura do MVP

## Estado D1c2a

A primeira implementação existe em `crates/vericode-core`. É uma crate Rust
pura que modela `Hash32`, `JobId`, `ImageId`, `Verdict`, os compromissos
esperados e `JournalV1`. D1c2a acrescenta um wire format Borsh `0.10.4`
candidato, compromissos SHA-256 por `sha2 0.10.9` e um harness determinístico
para um único registro de desenvolvimento restrito.

O registro contém somente versão, entrada `u32` e saída alegada `u32`. O
harness aplica a especificação fixa `saída = entrada * 2`, sem I/O, e trata
`PASS` e `FAIL` como resultados normais. Entrada malformada, versão
incompatível e valor fora do limite retornam erro explícito.

A crate não contém lógica Solana/Anchor/RISC Zero e não autoriza pagamentos.
Borsh e SHA-2 são bibliotecas Rust puras fixadas pelo lock; não introduzem SDK
de blockchain. O formato continua candidato local até validação por guest e
por qualquer adaptador on-chain futuro.

## Estado D2a

O módulo `crates/vericode-core/src/escrow.rs` acrescenta a política pura de
escrow: identidades de 32 bytes para buyer, executor e mint, `Amount` não
nulo, `JobV1` imutável, estados `Created`/`Funded`/`Delivered`/`Released`,
erros explícitos e a elegibilidade de release. O release exige journal com
todos os compromissos do Job e do artefato registrado pelo executor,
`Verdict::Pass`, executor e mint do Job. A política é um predicado puro: não
custodia fundos, não transfere tokens, não verifica receipt/prova e não lê
relógio. Refund, prazo e timeout continuam pendentes. Detalhes em
[`docs/escrow-state-machine.md`](escrow-state-machine.md).

## Fluxo

`Buyer cria Job e deposita Test USDC` -> `Executor fornece artefato restrito` -> `host executa o harness e gera receipt` -> `contrato valida prova e journal contra o Job` -> `release ao executor em PASS válido ou refund conforme as regras do Job`

O fluxo acima é alvo de arquitetura, não evidência de integração já funcional. As condições exatas de timeout e refund ainda precisam de especificação.

## Componentes e responsabilidades

| Componente | Responsabilidade | Não deve fazer |
| --- | --- | --- |
| Core Rust puro | Tipos de domínio, bytes candidatos, hashes, regra determinística, validação do artefato e `Verdict`; política pura de escrow (D2a) | Depender de Solana, Anchor ou RISC Zero; custodiar fundos, transferir tokens, verificar prova ou deter autoridade de release |
| Guest RISC Zero | Ler entrada restrita, chamar o core e publicar `JournalV1` com `PASS` ou `FAIL` | Tratar `FAIL` como panic/assert ou expor dados privados desnecessários |
| Host | Preparar entrada, executar/provar, obter receipt e conferir journal localmente | Ser fonte de verdade para liberar fundos |
| Programa Anchor | Manter Job/escrow e autorizar release/refund somente após validações do Job | Reexecutar regra de negócio, aceitar admin bypass ou confiar apenas no host |
| Router/verificador | Verificar receipt Groth16 por CPI, se a integração Solana for comprovada | Ser considerado disponível sem deployment e CPI validados |
| Front-end | Criar e consultar Jobs e apresentar estados/transações | Decidir verdict ou custodiar segredos do usuário |

## Fronteiras

- O core é a única implementação da regra de negócio e deve compilar como Rust puro.
- Host e futuro guest devem chamar a mesma API do core e comparar os vetores
  candidatos byte a byte; nenhuma cópia paralela da regra é autorizada.
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
