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

## Estado D2a/D2b

O módulo `crates/vericode-core/src/escrow.rs` contém a política pura de
escrow (D2a, alinhada ao guia de produto no D2b): identidades de 32 bytes
para buyer, executor e mint, `Amount` não nulo, `JobV1` imutável com
`deadline_slot`, estados `Created`/`Funded`/`Released`/`Refunded`, erros
explícitos e três liquidações. `Pass` vinculado ao Job libera ao executor até
o prazo; `Fail` vinculado devolve ao buyer em qualquer slot; timeout devolve
ao buyer somente após o prazo. O `artifact_hash` era registrado na liquidação
(substituído no D2b.1 pelo compromisso de entrega, abaixo).
A política é um predicado puro: não custodia fundos, não transfere tokens,
não verifica receipt/prova e recebe o slot como entrada. Detalhes em
[`docs/escrow-state-machine.md`](escrow-state-machine.md).

## Estado D2c

O workspace `anchor/` contém o programa local `vericode_escrow`
(Anchor `0.31.1`, Agave `2.3.9`):
- `create_job` persiste os termos imutáveis do Job no PDA `["job", job_id]` e
  cria o vault PDA `["vault", job]`, controlado pelo Job;
- `fund` deposita exatamente o valor do Job;
- `refund_on_timeout`, permissionless, devolve ao buyer após o prazo.

Toda regra econômica é delegada ao `vericode-core`. O programa foi testado em
processo (`solana-program-test 2.3.9`) e nunca implantado. Ainda não existem
`release`, `refund_on_fail` nem verificação de prova. Especificação em
[`docs/escrow-program.md`](escrow-program.md).

## Estado D2b.1

A revisão adversarial R-D2 mostrou que, sem compromisso de entrega, não
existe "o" artefato do Job: qualquer pessoa provava um FAIL de artefato
arbitrário para qualquer `job_id`. O D2b.1 vincula a liquidação à entrega:

- `deliver(artifact_hash)`, assinado só pelo executor, uma única vez, até o
  prazo, grava `Delivered { artifact_hash }`;
- `release` e `refund_on_fail` (core) exigem o journal desse artefato;
- o timeout devolve ao buyer a partir de `Funded` ou `Delivered`;
- `create_job` só admite a spec, o harness e o ImageID da v1, e um prazo na
  janela de 1.500 a 1.512.000 slots.

"Artefato vinculado ao Job" passa a significar: o artefato cujo hash o
executor comprometeu com `deliver`. Isso está testado no core e no programa
local; a verificação da receipt on-chain continua pendente (D2e).

## Fluxo

`Buyer cria Job (termos da v1) e deposita Test USDC` -> `Executor entrega e compromete o hash do artefato (deliver)` -> `host executa o harness e gera receipt` -> `contrato valida prova e journal contra o Job e a entrega` -> `release ao executor em PASS válido ou refund conforme as regras do Job`

O fluxo acima é alvo de arquitetura, não evidência de integração já funcional. As condições de entrega, release, refund e timeout estão especificadas e testadas como política pura (`docs/escrow-state-machine.md`). O programa local aplica criação, depósito, entrega e timeout; release e refund por `FAIL` ainda não existem on-chain.

## Componentes e responsabilidades

| Componente | Responsabilidade | Não deve fazer |
| --- | --- | --- |
| Core Rust puro | Tipos de domínio, bytes candidatos, hashes, regra determinística, validação do artefato e `Verdict`; política pura de escrow (D2a) | Depender de Solana, Anchor ou RISC Zero; custodiar fundos, transferir tokens, verificar prova ou deter autoridade de release |
| Guest RISC Zero | Ler entrada restrita, chamar o core e publicar `JournalV1` com `PASS` ou `FAIL` | Tratar `FAIL` como panic/assert ou expor dados privados desnecessários |
| Host | Preparar entrada, executar/provar, obter receipt e conferir journal localmente | Ser fonte de verdade para liberar fundos |
| Programa Anchor (`anchor/programs/vericode-escrow`) | Manter Job/escrow, custodiar no vault PDA e liquidar somente pelo core após validações do Job | Reexecutar regra de negócio, aceitar admin bypass ou confiar apenas no host |
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
