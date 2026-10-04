# VeriCode

Escrow para tarefas de desenvolvimento entre agentes, liberado após verificação determinística e, quando viável, prova zkVM.

## Escopo do MVP

- Um único artefato serializado restrito, avaliado por regra determinística e harness fixo.
- Receipt RISC Zero com journal público e veredito `PASS` ou `FAIL`.
- Escrow em Solana devnet com Test USDC: `PASS` paga o executor; `FAIL` ou timeout devolve ao comprador.
- Verificação on-chain pelo Verifier Router somente quando comprovada; caso contrário, fallback atestado e rotulado como não-ZK.

O MVP não verifica repositórios arbitrários nem patches gerais e não prova que um código está correto: a prova atesta a execução definida sobre o artefato vinculado ao Job.

## Status

Em desenvolvimento, sem deploy. Comprovado por evidência registrada:

- `crates/vericode-core`: crate Rust pura (`no_std`) com `JournalV1`, wire format Borsh candidato, compromissos SHA-256, harness restrito de desenvolvimento e política pura de escrow; testes locais passando com Rust `1.85.0` e `1.89.0`.
- `zkvm/`: guest RISC Zero `3.0.3` com dois builds determinísticos idênticos e receipts locais reais de `PASS` e `FAIL` verificadas localmente (tipo `Composite`).

Ainda não implementado: programa Anchor, custódia SPL, devnet, Router/CPI, CLI de ponta a ponta, worker e interface.

## Limitações atuais

- Não há verificação on-chain; Router/CPI/devnet: `STATUS: NÃO VALIDADO`.
- As receipts locais são `Composite`, não Groth16.
- Não há deploy nem transação em nenhuma rede.
- O artefato atual é um registro de desenvolvimento (`saída = entrada * 2`), não código arbitrário.
- A política de escrow existe apenas como lógica Rust pura; ela não custodia nem transfere tokens.

## Estrutura

- `crates/vericode-core/` — tipos canônicos, serialização, hashes, harness e política pura de escrow.
- `zkvm/` — host, métodos e guest RISC Zero.
- `docs/` — contexto do produto, decisões, evidências e relatórios de cada gate. Comece por [`docs/project-context.md`](docs/project-context.md).

## Evidências

Comandos, versões, hashes e saídas reais estão em [`docs/evidence.md`](docs/evidence.md) e nos relatórios `docs/d*-results.md`.

## Licença

Este projeto é distribuído sob a licença [Apache-2.0](LICENSE).
