# VeriCode

Escrow para tarefas de desenvolvimento entre agentes, liberado após verificação determinística e prova zkVM.

## O que está demonstrado

Num Job do VeriCode, o comprador deposita Test USDC num vault controlado pelo programa. O executor entrega um artefato restrito. Um guest RISC Zero avalia o artefato com uma regra fixa e publica um journal com o veredito. O programa só move o dinheiro depois de conferir o vínculo entre o journal, o Job e a entrega e de verificar a prova:

> A receipt Groth16 do VeriCode é **verificada em devnet por CPI ao verificador Groth16 imutável de risc0-solana v3.0.0**, na mesma instrução que libera ou devolve o Test USDC do Job.

Não há Verifier Router no caminho e nada disso existe em mainnet.

| Caso em devnet | Resultado | Transação |
| --- | --- | --- |
| PASS, Job `bc334093…` (D10, pelo worker local sobre a CLI e o prover deste repositório) | `deliver`+`release`: prova verificada (99.541 CU no verificador) e 1 Test USDC ao executor | [`ByGF4BFP…`](https://explorer.solana.com/tx/ByGF4BFPD8zkyqc6bg3Hd2FnQ1bF9gjo1EWp4fyVQkcGWfcj53gi99kWPLgyherzmvYp8knoqAzgjLLyhgpLnVR?cluster=devnet) |
| Timeout, Jobs `8ab4ee8d…` e `8bce67f2…` das gravações (D10, pelo worker) | reembolso ao comprador depois do prazo | [`57UYbVX9…`](https://explorer.solana.com/tx/57UYbVX9gEXdp7dproD5mEyxj5UzxuQZdjwhQZe96XKTfCKbrWSxrfhKTmqrTdnAby8PGyJVSa1gDZsdk7P9Xd9v?cluster=devnet), [`625JvmR8…`](https://explorer.solana.com/tx/625JvmR85QE6LbRZaE49kf3Y2cSFVnbKGaynVBbb8eeqM1Dj6r8UVUZiUbDoA4g5egJfqYUW175pG6d4NHYZWc3y?cluster=devnet) |
| Negativos do D10 (pelo worker) | reembolso antes do prazo (6021), seal adulterado rejeitado pelo verificador (6003) e dupla liquidação (6007); nada move | [`3PJfg2jU…`](https://explorer.solana.com/tx/3PJfg2jUmmDVc5RuN31DgF3pJwcYnog8GUCYy9vvyB56n1drScwiY8dTTsXwZWsX59DCuRDn33YhwViff3shxv4L?cluster=devnet), [`4cKK83JB…`](https://explorer.solana.com/tx/4cKK83JBYxRVfrAT5t6visKQdEWim5NANMe3eMH4bHsDmPumzq3WZb83F4eWiuYC694uivPqhsip3D8ySxJBmX2u?cluster=devnet), [`5ug6Pggx…`](https://explorer.solana.com/tx/5ug6PggxL9qZdgwkCu1Vt3oEYSFiBKXLdCi9bE6W1bewLjfa2yKpYjf8BeHzEjtvdkYDNLHZY4cjjBvkqtF9Bn5w?cluster=devnet) |
| PASS, Job P′ (D9, ambiente limpo, CLI e prover deste repositório) | `deliver`+`release`: prova verificada (99.541 CU no verificador) e 1 Test USDC ao executor | [`5tjezXYh…`](https://explorer.solana.com/tx/5tjezXYhN361HHUiZcMcJQFXwViWc4cSUfdreHfokdXuB89LrLDt6WPE8KRLhHwNE7rx5nGAgxjdfod6r7pPvDs1?cluster=devnet) |
| Timeout, Job T′ (D9) | reembolso ao comprador depois do prazo | [`33ezPvow…`](https://explorer.solana.com/tx/33ezPvow48vSFHYS43i76pDJwkpyKTnDCN3Tdr1p7he8mbHjEpXvsBkt2MrMHKPUNW6AbqWE7oRJ7MriRZNxrjkh?cluster=devnet) |
| Negativos do D9 | reembolso antes do prazo (6021), receipt do Job P do D7 em P′ (6014), seal adulterado rejeitado pelo verificador (6003) e dupla liquidação (6007); nada move | [`3BKnczeK…`](https://explorer.solana.com/tx/3BKnczeKPQd15iCkYfPbnEvDbYayRfbVV9Zokqn3xLyVehJuZSNffgDy1paFTxof8tpYfaN92N789LzB2w4DQTd6?cluster=devnet), [`8Kk5UkX5…`](https://explorer.solana.com/tx/8Kk5UkX5XW8tssPMxNSCEBt71jiv4RriHLDtYqRzb1f1U5WLZfgQmotQTD1ZqecEbyqMmhWSjQqWpXxCJUubJLg?cluster=devnet), [`HXgnGUhh…`](https://explorer.solana.com/tx/HXgnGUhhB4zRgD8Bubs6ULCqpuhoqBSuim3Vbx1kz7V3bK2FFJG3ND2jcTtRFf6QxD9i1bpwKW4z32yFYYEofVr?cluster=devnet), [`5T9XE5Y3…`](https://explorer.solana.com/tx/5T9XE5Y3DzqKffoeFFzEhgqxsuzqMstpxtMCQikrZPe1RyP1KWvRmPNs9CFuke1yBgtwg5vVnGZVpwHWZr31Dfrs?cluster=devnet) |
| PASS, Job P (D7, CLI e prover deste repositório) | `deliver`+`release`: prova verificada (99.541 CU no verificador) e 1 Test USDC ao executor | [`4oWhwZfU…`](https://explorer.solana.com/tx/4oWhwZfUzZhVhrTydrhdBx1TJxWVH2mdWdKwiUhtHtMnhmeJg3MprEshvp592hYMgsaiwwhyvsDHek1egS9zKM1L?cluster=devnet) |
| PASS, Job A (D4b) | `release` ao executor | [`4yWq28Gw…`](https://explorer.solana.com/tx/4yWq28GwkT9uLbG8haXbQYQyMqNWd6Qc29hWrez9cT4tGu36mS1fY8w6ocu75d5JxfQSLzTKKbMPc131fbL7chhR?cluster=devnet) |
| FAIL, Job B (D4b) | `refund_on_fail` ao comprador | [`2osG9m8J…`](https://explorer.solana.com/tx/2osG9m8JcE1CpAKt8JribtJCw8ffrdhBh6hYm5xKeLPptM9YLsugouMRH2KnmRXAymwAMvBABUmuZY6tkwHoVbP4?cluster=devnet) |
| Timeout, Job T (D7) | reembolso ao comprador depois do prazo | [`3fiNWgTW…`](https://explorer.solana.com/tx/3fiNWgTWdscCB8kRTE4gxacQgNXCZ36NHNhazzgUtRBiF7ub7Zs3NDBBZ7qwEdXGEy2HGUsZyyZX9sv7gxFsVZ7?cluster=devnet) |
| Journal de outro Job (D7) | rejeitado (6014), nada move | [`5eNRKgfH…`](https://explorer.solana.com/tx/5eNRKgfHii8gbWvJCNRTUehsT7Z46mXDC4Xorj2mfEfF4QjJ2nvFKwt3wmevJ5L81VXweaYpLzTxk3adKDAcBbJ7?cluster=devnet) |
| Seal adulterado (D4b) | rejeitado pelo verificador (6003), nada move | [`5UwKqSJs…`](https://explorer.solana.com/tx/5UwKqSJsku7rvYCXWz96BYNDNo3DjA8Yn7YXC4eMj1SYMhiUUvDLBvTLwWQEeabffLe8GtVFLC6x8Z1KH1PvCsUU?cluster=devnet) |
| Dupla liquidação (D7) | `release` de novo no Job A → 6007; `refund_on_fail` de novo no Job B → 6008 | [`61de4rBk…`](https://explorer.solana.com/tx/61de4rBkBSN7UjqFrMed8a4KncQvRiBG4rNHJtqVA46B74tMUBoZ6stXVKx3RH2gMydCbRM4Ki9G7EXemgsqY7na?cluster=devnet), [`5vZ6TGRo…`](https://explorer.solana.com/tx/5vZ6TGRoF8hnNMCARsw9AdUkrQSQccavrNVxJLuXbiu1VH3rZ8wM1wfuk2DGpFGq1g4YqdHeysBFKdmi4Zag3pjG?cluster=devnet) |
| Reembolso antes do prazo (D7) | rejeitado (6021), nada move | [`8DmQFdjE…`](https://explorer.solana.com/tx/8DmQFdjEjRsVHTfU1DmgjA6GmXZs52G1fdcuCaKmkSV9qXR6eEKh8MiGpKAKv6xa7LDdNPzK7AS9fbP9DgiyX1S?cluster=devnet) |

A lista completa, com CU, tamanhos e saldos, está em [`docs/d4b-devnet-results.md`](docs/d4b-devnet-results.md), [`docs/d7-cli-results.md`](docs/d7-cli-results.md), [`docs/d9-demo-results.md`](docs/d9-demo-results.md) e [`docs/d10-worker-results.md`](docs/d10-worker-results.md).

**O que a prova não diz.** Ela atesta que o guest admitido executou a regra fixa sobre o artefato que o executor comprometeu ao entregar. Não prova que um código está correto: a regra v1 é trivial (`saída = 2 × entrada`) e serve só para demonstrar o fluxo.

## Frases permitidas (congeladas no D9)

README, roteiro, vídeo, worker e telas usam só estas frases, ou paráfrases que não digam mais do que elas. Mudar a lista exige decisão registrada em [`docs/decisions.md`](docs/decisions.md).

1. **Claim:** "A receipt Groth16 do VeriCode é verificada em devnet por CPI ao verificador Groth16 imutável de risc0-solana v3.0.0, na mesma instrução que libera ou devolve o Test USDC do Job." Sempre com o link de uma transação da tabela acima.
2. **Afirmação:** "Um avaliador determinístico previamente comprometido executou sobre o artefato entregue no Job e produziu o veredito publicado. O programa só move o Test USDC depois de conferir o vínculo entre journal, Job e entrega e de verificar a prova."
3. **Limite:** "A prova atesta a execução da regra fixa sobre este artefato, não a qualidade de um software. A regra v1 é trivial (saída = 2 × entrada) e serve para demonstrar o fluxo."
4. **Sem administrador:** "O escrow e o verificador são imutáveis (upgrade authority `none`). Ninguém, nem o projeto, muda as regras ou decide o pagamento; também não existe e-stop."
5. **Rede:** "Só devnet e Test USDC; nada disso existe em mainnet."
6. **Negativos:** "Em devnet, uma prova adulterada, um journal de outro Job, um reembolso antes do prazo e uma segunda liquidação foram rejeitados sem mover fundos." Sempre com os links.
7. **Reprodução:** "O fluxo foi reproduzido pela CLI e pelo prover deste repositório num ambiente limpo: clone e targets novos, com toolchains isoladas copiadas, não uma máquina nova." (D9)
8. **Prova:** "A prova é gerada localmente e comprimida para Groth16 num container Docker local, sem rede."

**Não dizer:** "o código está correto"; "trustless" ou "ninguém precisa confiar em ninguém"; "qualquer repositório"; "Verifier Router" como caminho atual; "mainnet" ou "dinheiro real"; "ZK on-chain" sem a frase 1; "auditado"; "privado"; "máquina nova". Também não apresentar uma execução anterior como ao vivo, nem mostrar uma receipt sem dizer de qual Job ela é.

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

Requisitos:
- Linux x86_64, Rust `1.89.0` via rustup e compilador C/C++;
- uns 8 GiB de RAM: rode um build pesado ou uma prova por vez;
- Docker, só para o Groth16, com a imagem `risczero/risc0-groth16-prover@sha256:7f173963…` (5,21 GB) já baixada por digest. O prover nunca faz pull;
- a CLI do Agave `2.3.9` (`solana`), só para baixar os programas na seção 1.

**Rede no primeiro build:** rustup (toolchain `1.89.0`), crates.io e, no build do prover, o `recursion_zkr.zip` do S3 da RISC Zero (SHA-256 `744b999f…`), a menos que `RECURSION_SRC_PATH` aponte para uma cópia local ([`prover/README.md`](prover/README.md)). Com isso presente, os builds e testes rodam com `--offline`.

### 1. Testes locais

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
$V job show --job-id 91ea6fcdd2d0c29942246c664d893634b7c66f6cb2babbe436734a74bef5418e   # P′ (D9): Released
$V job show --job-id ec9afb74ceb080e09cb3d605d94883ff5ea90cc638c3f086f5e5c6d4cfed5c48   # T′ (D9): RefundedOnTimeout
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
- Achados do R-D7 na CLI e no prover ([`docs/r-d7-review-results.md`](docs/r-d7-review-results.md)), corrigidos no D10a com testes ([`docs/d10a-hardening-results.md`](docs/d10a-hardening-results.md)) e confirmados pela revisão delta R-D10a ([`docs/r-d10a-review-results.md`](docs/r-d10a-review-results.md)):
  - `--expect-error escrow:N` só casa quando o escrow é a falha mais interna (RD7-01);
  - o shim do Docker é uma allowlist exata de argv, com a imagem por digest, sem rede e no daemon local (RD7-02);
  - o prover usa só o prover local e recusa `RISC0_PROVER` diferente de `local`, `BONSAI_*` e `RISC0_DEV_MODE` (RD7-03);
  - o casamento do `--expect-error` e o `--tamper-seal` têm testes (RD7-04);
  - um "already processed" é resolvido pelo status da assinatura, e as leituras depois da transação usam `minContextSlot` (RD7-07).

  Os binários do D9, usados na seção "Gravação" do roteiro, são anteriores a essas correções. Os negativos documentados continuam sendo 6014, 6007/6008, 6021 e `verifier:6003`. Continua aberto o RD7-08: um negativo `escrow:6021` enviado perto do prazo pode virar reembolso real, e a CLI então reporta `UNEXPECTED`.
- **Worker local, sem carteira no navegador (D10, decisão P3).** O worker ([`worker/`](worker/README.md)) roda só em `127.0.0.1` e só chama os binários do D10a. Ele guarda, por caminho, as chaves de devnet do projeto (buyer e executor; nunca a do deployer), e o mesmo operador local opera os dois papéis. A prova (`Proving`) é uma etapa local do executor, fora da cadeia. A interface (D11–D12) ainda não existe.
- A frase 8 vale para receipts cuja compressão registrou a linha `docker_run` do shim (`--context default run --pull=never --network=none … @sha256:7f173963…`; condição C10-2 do R-D10a). O worker recusa as demais.

## Estrutura

- `crates/vericode-core/`: tipos canônicos, serialização, hashes, harness e política pura de escrow.
- `zkvm/`: guest e host RISC Zero (build determinístico do D1c2b).
- `anchor/`: programa de escrow e testes em processo (`anchor/tests-local`).
- `prover/`: prover local do guest admitido (Composite → Groth16).
- `cli/`: cliente de devnet (`vericode`).
- `worker/`: worker local HTTP em `127.0.0.1` sobre a CLI e o prover (D10), base das telas.
- `docs/`: contexto do produto, decisões, evidências e relatórios de cada gate. Comece por [`docs/project-context.md`](docs/project-context.md).

## Evidências

Comandos, versões, hashes e saídas reais estão em [`docs/evidence.md`](docs/evidence.md) e nos relatórios `docs/d*-results.md`.

## Licença

Este projeto é distribuído sob a licença [Apache-2.0](LICENSE).
