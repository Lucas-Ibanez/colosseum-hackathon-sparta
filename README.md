# VeriCode

Escrow para tarefas de desenvolvimento entre agentes, liberado após verificação determinística e prova zkVM.

## O que está demonstrado

Num Job do VeriCode, o comprador deposita Test USDC num vault controlado pelo programa. O executor entrega um artefato restrito. Um guest RISC Zero avalia o artefato com uma regra fixa e publica um journal com o veredito. O programa só move o dinheiro depois de conferir o vínculo entre o journal, o Job e a entrega e de verificar a prova:

> A receipt Groth16 do VeriCode é **verificada em devnet por CPI ao verificador Groth16 imutável de risc0-solana v3.0.0**, na mesma instrução que libera ou devolve o Test USDC do Job.

Não há Verifier Router no caminho e nada disso existe em mainnet.

| Caso em devnet | Resultado | Transação |
| --- | --- | --- |
| PASS, Job P (D7, CLI e prover deste repositório) | `deliver`+`release`: prova verificada (99.541 CU no verificador) e 1 Test USDC ao executor | [`4oWhwZfU…`](https://explorer.solana.com/tx/4oWhwZfUzZhVhrTydrhdBx1TJxWVH2mdWdKwiUhtHtMnhmeJg3MprEshvp592hYMgsaiwwhyvsDHek1egS9zKM1L?cluster=devnet) |
| PASS, Job A (D4b) | `release` ao executor | [`4yWq28Gw…`](https://explorer.solana.com/tx/4yWq28GwkT9uLbG8haXbQYQyMqNWd6Qc29hWrez9cT4tGu36mS1fY8w6ocu75d5JxfQSLzTKKbMPc131fbL7chhR?cluster=devnet) |
| FAIL, Job B (D4b) | `refund_on_fail` ao comprador | [`2osG9m8J…`](https://explorer.solana.com/tx/2osG9m8JcE1CpAKt8JribtJCw8ffrdhBh6hYm5xKeLPptM9YLsugouMRH2KnmRXAymwAMvBABUmuZY6tkwHoVbP4?cluster=devnet) |
| Timeout, Job T (D7) | reembolso ao comprador depois do prazo | [`3fiNWgTW…`](https://explorer.solana.com/tx/3fiNWgTWdscCB8kRTE4gxacQgNXCZ36NHNhazzgUtRBiF7ub7Zs3NDBBZ7qwEdXGEy2HGUsZyyZX9sv7gxFsVZ7?cluster=devnet) |
| Journal de outro Job (D7) | rejeitado (6014), nada move | [`5eNRKgfH…`](https://explorer.solana.com/tx/5eNRKgfHii8gbWvJCNRTUehsT7Z46mXDC4Xorj2mfEfF4QjJ2nvFKwt3wmevJ5L81VXweaYpLzTxk3adKDAcBbJ7?cluster=devnet) |
| Seal adulterado (D4b) | rejeitado pelo verificador (6003), nada move | [`5UwKqSJs…`](https://explorer.solana.com/tx/5UwKqSJsku7rvYCXWz96BYNDNo3DjA8Yn7YXC4eMj1SYMhiUUvDLBvTLwWQEeabffLe8GtVFLC6x8Z1KH1PvCsUU?cluster=devnet) |
| Dupla liquidação (D7) | `release` de novo no Job A → 6007; `refund_on_fail` de novo no Job B → 6008 | [`61de4rBk…`](https://explorer.solana.com/tx/61de4rBkBSN7UjqFrMed8a4KncQvRiBG4rNHJtqVA46B74tMUBoZ6stXVKx3RH2gMydCbRM4Ki9G7EXemgsqY7na?cluster=devnet), [`5vZ6TGRo…`](https://explorer.solana.com/tx/5vZ6TGRoF8hnNMCARsw9AdUkrQSQccavrNVxJLuXbiu1VH3rZ8wM1wfuk2DGpFGq1g4YqdHeysBFKdmi4Zag3pjG?cluster=devnet) |
| Reembolso antes do prazo (D7) | rejeitado (6021), nada move | [`8DmQFdjE…`](https://explorer.solana.com/tx/8DmQFdjEjRsVHTfU1DmgjA6GmXZs52G1fdcuCaKmkSV9qXR6eEKh8MiGpKAKv6xa7LDdNPzK7AS9fbP9DgiyX1S?cluster=devnet) |

A lista completa, com CU, tamanhos e saldos, está em [`docs/d4b-devnet-results.md`](docs/d4b-devnet-results.md) e [`docs/d7-cli-results.md`](docs/d7-cli-results.md).

**O que a prova não diz.** Ela atesta que o guest admitido executou a regra fixa sobre o artefato que o executor comprometeu ao entregar. Não prova que um código está correto: a regra v1 é trivial (`saída = 2 × entrada`) e serve só para demonstrar o fluxo.

## Escopo do MVP

- Um único artefato serializado restrito, avaliado por regra determinística e harness fixo.
- Receipt RISC Zero com journal público e veredito `PASS` ou `FAIL`.
- Escrow em Solana devnet com Test USDC: `PASS` paga o executor; `FAIL` ou timeout devolve ao comprador.

O MVP não verifica repositórios arbitrários nem patches gerais.

## Versões, hashes e endereços

| Item | Valor |
| --- | --- |
| Escrow (devnet) | [`GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH`](https://explorer.solana.com/address/GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH?cluster=devnet): 395.064 bytes, SHA-256 `cdf6967f3abc63d0385e36909fe61b36f639203e2679209be60c4875114d8133`; upgrade authority **`none`** desde a [finalização](https://explorer.solana.com/tx/4AsofYxry7vjdpo2CQxSKSg9MdczuCB4GnRCLzz6ZeVFffPkWdedFBuBQDw58audRHJLcAQ1sGrN5G3t2CoW7eYH?cluster=devnet) |
| Verificador Groth16 (devnet) | [`THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge`](https://explorer.solana.com/address/THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge?cluster=devnet), `risc0-solana v3.0.0` (commit `ee415935`), upgrade authority `None`; bytes de devnet `34ae6e5c…`, rebuild local `dab6746d…`, equivalentes estrutural e funcionalmente |
| Mint Test USDC | [`9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F`](https://explorer.solana.com/address/9TE2VPFmgrNxT22yS3sEZyRcMxLgJkwzgAoquWRXwV2F?cluster=devnet): SPL Token, 6 decimais, sem freeze authority |
| Guest admitido | [`prover/artifacts/vericode-guest.bin`](prover/artifacts/README.md): 180.300 bytes `e09ba8cf…`; ELF `63fac491…`; ImageID `4da06f90da75ec8980c943ce017d69c48370fddbf3aa27689d375d78fac0fb1a` |
| Termos v1 | spec `af642b56…b778`, harness `01124025…6b50`, prazo de 1.500 a 1.512.000 slots ([`docs/manifest-schema.md`](docs/manifest-schema.md)) |
| `JournalV1` | v1 congelado, 165 bytes |
| Toolchains | Rust `1.89.0` (host; core também em `1.85.0`); RISC Zero `3.0.3` (guest Rust `1.88.0`); Anchor `0.31.1` + Agave `2.3.9` |
| Prover Groth16 (Docker) | `risczero/risc0-groth16-prover@sha256:7f173963196570b7a71816ed70565a4579264c5d2e3e0ecb028102538ad0e331` |
| Locks | raiz `191802b2…`; `zkvm` `f5236689…`; guest `1116acef…`; `anchor` `19a1db26…`; `anchor/tests-local` `be94760a…`; `cli` `4d979577…`; `prover` `8b76f1e1…` |

## Reproduzir a partir de um clone

Requisitos: Linux x86_64, Rust `1.89.0` via rustup, compilador C/C++, Docker (só para o Groth16) e uns 8 GiB de RAM (rode um build pesado ou uma prova por vez).

### 1. Testes locais (sem rede, exceto o crates.io no primeiro build)

```bash
cargo +1.89.0 test --locked                                             # core, 42 testes
cargo +1.89.0 test --locked --release --manifest-path prover/Cargo.toml # prover: guest, frame, journal == core
cargo +1.89.0 test --locked --manifest-path cli/Cargo.toml              # CLI: bytes == builders da suíte
```

A suíte do programa (`anchor/tests-local`, 61 testes) roda em processo com o `.so` do escrow e o do verificador. Os dois podem vir de devnet. A CLI 2.3.9 exige um signer padrão até para leitura; `-k` aponta para qualquer keypair de devnet e nada é assinado.

```bash
SBF=~/vericode-sbf; mkdir -p $SBF            # fora do clone
solana -u devnet -k ~/devnet-key.json program dump GZqbL2TbeDVHcNRosngaRfCwzV9YJT6iEbckYr8uwkCH $SBF/vericode_escrow.so   # cdf6967f…
solana -u devnet -k ~/devnet-key.json program dump THq1qFYQoh7zgcjXoMXduDBqiZRCPeg3PvvMbrVQUge $SBF/groth_16_verifier.so # 34ae6e5c…
(cd anchor/tests-local && SBF_OUT_DIR=$SBF cargo +1.89.0 test --locked)
```

Para reconstruir o escrow em vez de baixá-lo: `cargo-build-sbf --manifest-path anchor/programs/vericode-escrow/Cargo.toml -- --locked` com Agave `2.3.9` (ver [`docs/escrow-program.md`](docs/escrow-program.md)).

### 2. Conferir o devnet (só leitura)

```bash
cargo +1.89.0 build --locked --release --manifest-path cli/Cargo.toml
V=cli/target/release/vericode
$V check        # escrow cdf6967f… com authority none, verificador imutável, mint
$V job show --job-id 3e4ca0269e4af134738120703ccbfd751ef0d51fa6dcd2d99525d0d3f24c9c57   # P: Released
$V job show --job-id 05f7493483cba42146cada07822160d2fbec4e67d854fafa32f70ef0bb01d758   # T: RefundedOnTimeout
$V job show --job-id 3f0dd1c833714e193744c5b95879662c1d8b440cdd3c6467a1585ecc60baec8a   # A: Released
$V job show --job-id 5a25ae4808b6da928f57c8787c4fbb173791239d18342b540d4c485647fbc309   # B: RefundedOnFail
```

### 3. Provar localmente

```bash
export RISC0_PROVER=local RISC0_EXECUTOR=local; unset RISC0_DEV_MODE
RUN=~/vericode-run; mkdir -p $RUN          # receipts e registros fora do clone
cargo +1.89.0 build --locked --release --manifest-path prover/Cargo.toml
P=prover/target/release/vericode-prover
$P check
$P prove <JOB_ID_HEX> 21 42 $RUN/P    # Composite, journal == core
$P compress $RUN/P                    # Groth16 (Docker local por digest, sem rede)
$P verify $RUN/P
```

Detalhes, memória e `RECURSION_SRC_PATH` estão em [`prover/README.md`](prover/README.md).

### 4. Fluxo completo em devnet

Exige keypairs de devnet (buyer, executor) fora do clone, com modo `0600`, SOL de devnet e **Test USDC do mint admitido**. A mint authority é a chave de deployer do projeto, então um terceiro precisa receber Test USDC dela.

```bash
$V job create --buyer-keypair ~/keys/buyer.json --executor <PUBKEY_EXECUTOR> --deadline-offset 9000 --job-file $RUN/job.json
$P prove <JOB_ID_HEX> 21 42 $RUN/P && $P compress $RUN/P
$V job settle --job-id <JOB_ID_HEX> --receipt $RUN/P --deliver --executor-keypair ~/keys/executor.json
# ou, sem entrega, depois do prazo:
$V job refund-timeout --job-id <JOB_ID_HEX> --payer-keypair ~/keys/buyer.json --wait
```

Cada operação confere cluster, programa, mint, termos, receipt e estado antes de enviar, simula a transação e imprime o link do Explorer. Os cenários negativos usam `--expect-error`. Ver [`cli/README.md`](cli/README.md) e o roteiro [`docs/demo-script.md`](docs/demo-script.md).

## Limitações

- Só devnet e Test USDC; sem mainnet nem dinheiro real. A verificação não passa pelo Verifier Router, cujo upstream em devnet não está inicializado.
- **Sem e-stop.** O escrow (upgrade authority `none`) e o verificador são imutáveis. Um bug só se corrige com um novo program ID, e um bug de soundness do verificador não poderia ser pausado. Isso foi aceito para o MVP com Test USDC.
- A regra v1 é trivial e quem a escolhe é o executor: qualquer `(n, 2n)` passa. O compromisso de entrega (`deliver`) impede que terceiros troquem o artefato, mas não torna a tarefa difícil.
- `job_id` e Jobs são públicos, e a CLI gera um `job_id` aleatório por Job. O rent do Job e do vault (cerca de 0,0036 SOL) fica preso, porque não há `close`.
- A mint authority do Test USDC é a chave de deployer do projeto.
- O ImageID vem do guest determinístico do D1c2b e não foi recertificado desde então; o core só ganhou o módulo `escrow`, que o guest não usa.
- O prover Groth16 exige Docker x86 e muita memória; no WSL de 7,6 GiB a compressão leva cerca de 2 minutos.
- Worker e interface ainda não existem.

## Estrutura

- `crates/vericode-core/`: tipos canônicos, serialização, hashes, harness e política pura de escrow.
- `zkvm/`: guest e host RISC Zero (build determinístico do D1c2b).
- `anchor/`: programa de escrow e testes em processo (`anchor/tests-local`).
- `prover/`: prover local do guest admitido (Composite → Groth16).
- `cli/`: cliente de devnet (`vericode`).
- `docs/`: contexto do produto, decisões, evidências e relatórios de cada gate. Comece por [`docs/project-context.md`](docs/project-context.md).

## Evidências

Comandos, versões, hashes e saídas reais estão em [`docs/evidence.md`](docs/evidence.md) e nos relatórios `docs/d*-results.md`.

## Licença

Este projeto é distribuído sob a licença [Apache-2.0](LICENSE).
