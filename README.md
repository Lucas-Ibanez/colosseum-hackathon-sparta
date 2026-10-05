# VeriCode

Escrow para tarefas de desenvolvimento entre agentes, liberado após verificação determinística e, quando viável, prova zkVM.

## Escopo do MVP

- Um único artefato serializado restrito, avaliado por regra determinística e harness fixo.
- Receipt RISC Zero com journal público e veredito `PASS` ou `FAIL`.
- Escrow em Solana devnet com Test USDC: `PASS` paga o executor; `FAIL` ou timeout devolve ao comprador.
- Verificação on-chain da receipt por CPI ao verificador Groth16 de `risc0-solana` somente quando comprovada em devnet; caso contrário, fallback atestado e rotulado como não-ZK. O Verifier Router upstream em devnet não está inicializado, então o programa chama o verificador direto (D4a).

O MVP não verifica repositórios arbitrários nem patches gerais e não prova que um código está correto: a prova atesta a execução definida sobre o artefato vinculado ao Job, isto é, o artefato cujo hash o executor comprometeu ao entregar.

## Status

Em desenvolvimento, sem deploy. Comprovado por evidência registrada:

- `crates/vericode-core`: crate Rust pura (`no_std`) com `JournalV1` (v1 congelado, 165 bytes, [`docs/manifest-schema.md`](docs/manifest-schema.md)), compromissos SHA-256, harness restrito de desenvolvimento e política pura de escrow; testes locais passando com Rust `1.85.0` e `1.89.0`.
- `zkvm/`: guest RISC Zero `3.0.3` com dois builds determinísticos idênticos e receipts locais reais de `PASS` e `FAIL` verificadas localmente (tipo `Composite`).
- `anchor/`: programa Anchor `0.31.1` local (Agave `2.3.9`), testado em processo com `solana-program-test` e nunca implantado:
  - criação de Job restrita à spec, ao harness e ao ImageID da v1 e ao mint Test USDC admitido, com prazo numa janela fixa;
  - depósito em vault controlado por PDA;
  - compromisso de entrega assinado pelo executor (`deliver`);
  - release ao executor em `PASS` e refund ao buyer em `FAIL`, somente para o artefato entregue e somente depois que a receipt Groth16 foi verificada por CPI ao verificador Groth16 de `risc0-solana v3.0.0` em `solana-program-test` local, inclusive com os bytes do verificador implantado em devnet (D4a; no D2e a CPI passava pelo Verifier Router);
  - refund por timeout.

Ainda não implementado: devnet (deploy do escrow, criação do Test USDC e transações, previstos para o D4b depois da revisão R-D4a), CLI de ponta a ponta, worker e interface.

## Limitações atuais

- A verificação da prova só foi exercitada em `solana-program-test` local, com o verificador do commit pinado (rebuild offline e bytes lidos de devnet); não é "ZK on-chain" em cluster. O verificador Groth16 de devnet é imutável e foi chamado só em simulação, sem transação.
- Sem Verifier Router, não há e-stop: um bug de soundness do verificador não poderia ser pausado. Isso foi aceito para o MVP em devnet com Test USDC.
- As receipts Groth16 usadas nos testes (PASS e FAIL do Job de teste) estão versionadas em `anchor/tests-local/fixtures/groth16/`; a geração exige o prover Docker fora do repositório.
- Não há deploy nem transação em nenhuma rede. A upgrade authority do escrow só será finalizada no D4b, depois de um smoke run. O mint Test USDC admitido ainda não foi criado em devnet.
- O artefato atual é um registro de desenvolvimento (`saída = entrada * 2`), não código arbitrário. O executor escolhe a entrada, e qualquer par `(n, 2n)` passa: a tarefa é trivial e serve só para demonstrar o fluxo. O compromisso de entrega impede que terceiros troquem o artefato, mas não torna a tarefa difícil.
- O programa de escrow só existe localmente: o pagamento ao executor e os refunds foram exercitados apenas em `solana-program-test`.

## Estrutura

- `crates/vericode-core/` — tipos canônicos, serialização, hashes, harness e política pura de escrow.
- `anchor/` — programa Anchor local de escrow e seus testes em processo.
- `zkvm/` — host, métodos e guest RISC Zero.
- `docs/` — contexto do produto, decisões, evidências e relatórios de cada gate. Comece por [`docs/project-context.md`](docs/project-context.md).

## Evidências

Comandos, versões, hashes e saídas reais estão em [`docs/evidence.md`](docs/evidence.md) e nos relatórios `docs/d*-results.md`.

## Licença

Este projeto é distribuído sob a licença [Apache-2.0](LICENSE).
