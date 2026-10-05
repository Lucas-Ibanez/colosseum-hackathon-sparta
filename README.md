# VeriCode

Escrow para tarefas de desenvolvimento entre agentes, liberado após verificação determinística e, quando viável, prova zkVM.

## Escopo do MVP

- Um único artefato serializado restrito, avaliado por regra determinística e harness fixo.
- Receipt RISC Zero com journal público e veredito `PASS` ou `FAIL`.
- Escrow em Solana devnet com Test USDC: `PASS` paga o executor; `FAIL` ou timeout devolve ao comprador.
- Verificação da receipt por CPI ao verificador Groth16 de `risc0-solana` na mesma instrução que libera ou devolve o pagamento, comprovada em devnet no D4b. O Verifier Router upstream em devnet não está inicializado, então o programa chama o verificador direto (D4a).

O MVP não verifica repositórios arbitrários nem patches gerais e não prova que um código está correto: a prova atesta a execução definida sobre o artefato vinculado ao Job, isto é, o artefato cujo hash o executor comprometeu ao entregar.

## Status

Em desenvolvimento. Escrow implantado e imutável em Solana devnet (D4b). Comprovado por evidência registrada:

- `crates/vericode-core`: crate Rust pura (`no_std`) com `JournalV1` (v1 congelado, 165 bytes, [`docs/manifest-schema.md`](docs/manifest-schema.md)), compromissos SHA-256, harness restrito de desenvolvimento e política pura de escrow; testes locais passando com Rust `1.85.0` e `1.89.0`.
- `zkvm/`: guest RISC Zero `3.0.3` com dois builds determinísticos idênticos e receipts locais reais de `PASS` e `FAIL` verificadas localmente (tipo `Composite`).
- `anchor/`: programa Anchor `0.31.1` (Agave `2.3.9`), testado em processo com `solana-program-test` e implantado em devnet:
  - criação de Job restrita à spec, ao harness e ao ImageID da v1 e ao mint Test USDC admitido, com prazo numa janela fixa;
  - depósito em vault controlado por PDA;
  - compromisso de entrega assinado pelo executor (`deliver`);
  - release ao executor em `PASS` e refund ao buyer em `FAIL`, somente para o artefato entregue e somente depois que a receipt Groth16 foi verificada por CPI ao verificador Groth16 de `risc0-solana v3.0.0` em `solana-program-test` local, inclusive com os bytes do verificador implantado em devnet (D4a; no D2e a CPI passava pelo Verifier Router);
  - refund por timeout.
- Devnet (D4b, [`docs/d4b-devnet-results.md`](docs/d4b-devnet-results.md)):
  - escrow `GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`, com os 395.064 bytes `cdf6967f…` do D4a e upgrade authority `none`;
  - mint Test USDC `9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F`, com 6 decimais e sem freeze authority;
  - a receipt Groth16 do VeriCode é **verificada em devnet por CPI ao verificador Groth16 imutável de risc0-solana v3.0.0** na liquidação PASS ([release do Job A](https://explorer.solana.com/tx/4yWq28GwkT9uLbG8haXbQYQyMqNWd6Qc29hWrez9cT4tGu36mS1fY8w6ocu75d5JxfQSLzTKKbMPc131fbL7chhR?cluster=devnet)) e na FAIL ([refund_on_fail do Job B](https://explorer.solana.com/tx/2osG9m8JcE1CpAKt8JribtJCw8ffrdhBh6hYm5xKeLPptM9YLsugouMRH2KnmRXAymwAMvBABUmuZY6tkwHoVbP4?cluster=devnet));
  - refund por timeout em devnet ([Job C](https://explorer.solana.com/tx/5sHg855QNbKoXqnveLG6aUqvLcvtVkkUThN7hcKcuktNjMwh5EJ5yYJEy8QvgZjFfFFi144yKz4VYnAcBP8qdSTV?cluster=devnet));
  - seal adulterado, seal de outro journal, journal de outro Job ou artefato, selector errado, release com FAIL e timeout antecipado foram rejeitados em transações aterrissadas, sem mover fundos.

Ainda não implementado: CLI de ponta a ponta no repositório (as transações do D4b foram feitas por ferramentas fora do clone), worker e interface.

## Limitações atuais

- A verificação on-chain existe só em devnet, com Test USDC, e não passa pelo Verifier Router. Não há deploy em mainnet.
- Sem Verifier Router, não há e-stop: um bug de soundness do verificador não poderia ser pausado. Isso foi aceito para o MVP em devnet com Test USDC.
- As receipts Groth16 usadas nos testes (PASS e FAIL do Job de teste) estão versionadas em `anchor/tests-local/fixtures/groth16/`; a geração exige o prover Docker fora do repositório.
- O escrow é imutável (upgrade authority `none`): um bug só se corrige com um novo program ID. O rent de cada Job (cerca de 0,0036 SOL) fica preso, porque não há `close`. A mint authority do Test USDC é a chave de deployer do projeto.
- O artefato atual é um registro de desenvolvimento (`saída = entrada * 2`), não código arbitrário. O executor escolhe a entrada, e qualquer par `(n, 2n)` passa: a tarefa é trivial e serve só para demonstrar o fluxo. O compromisso de entrega impede que terceiros troquem o artefato, mas não torna a tarefa difícil.
- Os Jobs de devnet do D4b usaram receipts geradas fora do repositório; ainda não existe uma CLI reproduzível no repositório para criar, provar e liquidar um Job.

## Estrutura

- `crates/vericode-core/` — tipos canônicos, serialização, hashes, harness e política pura de escrow.
- `anchor/` — programa Anchor local de escrow e seus testes em processo.
- `zkvm/` — host, métodos e guest RISC Zero.
- `docs/` — contexto do produto, decisões, evidências e relatórios de cada gate. Comece por [`docs/project-context.md`](docs/project-context.md).

## Evidências

Comandos, versões, hashes e saídas reais estão em [`docs/evidence.md`](docs/evidence.md) e nos relatórios `docs/d*-results.md`.

## Licença

Este projeto é distribuído sob a licença [Apache-2.0](LICENSE).
